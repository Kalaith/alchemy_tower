//! What a finished bottle is worth when it goes back into the pot.
//!
//! Second-order brewing — a bench that takes bottles as reagents — was blind to
//! how well its inputs were made. Every potion in the data files leaves
//! `quality` unset, so the schema default of 20 stood in for a Crude draught and
//! a Masterwork one alike, and a compound brew leaned entirely on process
//! bonuses and the catalyst to reach a band. Brewing the input well changed
//! nothing.
//!
//! The bottles already carry what they were brewed at, in `bottle_stock`. This
//! folds that into the reagent the way `gameplay_variant_stock` folds a wild
//! variant: the pot is filled with the *best* bottle held, its quality and the
//! traits it came out with land on the ingredient, and the brew spends that
//! batch rather than the worst on the shelf. Nothing downstream needs to know
//! bottles are graded.
//!
//! Elements are deliberately not folded. A batch records quality and traits
//! because that is what a brew resolves; a potion's element profile is authored
//! on the item and is the same for every bottle of it.

use std::collections::BTreeMap;

use super::GameplayState;
use crate::data::{BottleBatchEntry, GameData, ItemCategory, ItemDefinition};

/// The bottles a brew could pour, per item id, best last so a pour can take
/// them off the end.
pub(super) type BottlePour = BTreeMap<String, Vec<BottleBatchEntry>>;

impl GameplayState {
    /// The graded bottles on the shelf, ready to be poured one at a time.
    pub(super) fn reagent_bottle_pour(&self) -> BottlePour {
        self.progression
            .bottle_stock
            .keys()
            .map(|item_id| {
                let mut batches = self.live_batches(item_id);
                batches.sort_by_key(|batch| batch.quality_score);
                (item_id.clone(), batches)
            })
            .collect()
    }

    /// Spend the graded bottles a brew of `selected_items` just poured. Mirrors
    /// the choice `reagent_bottle_pour` made — best first — so the bottle that
    /// improved the brew is the one that leaves the shelf, rather than the worst
    /// one `reconcile_bottle_stock` would otherwise trim.
    pub(super) fn spend_brew_bottles(&mut self, data: &GameData, selected_items: &[String]) {
        for item_id in selected_items {
            if !is_potion(data, item_id) {
                continue;
            }
            self.reconcile_bottle_stock(item_id);
            let Some(batches) = self.progression.bottle_stock.get_mut(item_id) else {
                continue;
            };
            if let Some(best) = batches.iter_mut().max_by_key(|batch| batch.quality_score) {
                best.count = best.count.saturating_sub(1);
            }
            batches.retain(|batch| batch.count > 0);
            if batches.is_empty() {
                self.progression.bottle_stock.remove(item_id);
            }
        }
    }

    /// The quality a reagent would actually go into the pot at. Ingredients are
    /// worth what the data file says; a bottle is worth what it was brewed at.
    /// The materials list reads this, because a row promising quality 20 beside
    /// a bench that pours a Masterwork solution is a lie about the only decision
    /// the late tier asks the player to make.
    pub(super) fn reagent_quality(&self, data: &GameData, item_id: &str) -> u32 {
        let base = data
            .item(item_id)
            .map(|item| item.quality)
            .unwrap_or_default();
        if !is_potion(data, item_id) {
            // A herb gathered under the right sky goes into the pot better than
            // the data file's number, and the bench spends the best one held —
            // so this is the figure that decides the brew, and it was the only
            // one the player could not see.
            return base
                + self
                    .best_held_variant(data, item_id)
                    .map(|variant| variant.quality_bonus)
                    .unwrap_or_default();
        }
        self.live_batches(item_id)
            .iter()
            .map(|batch| batch.quality_score)
            .max()
            .unwrap_or(base)
    }
}

fn is_potion(data: &GameData, item_id: &str) -> bool {
    data.item(item_id)
        .is_some_and(|item| item.category == ItemCategory::Potion)
}

/// Take the best remaining bottle of this item out of the pour and fold it into
/// the reagent. Bottles with no batch — bought, gifted, granted — are plain
/// examples of the item and go in as authored.
pub(super) fn pour_bottle(item: &ItemDefinition, pour: &mut BottlePour) -> ItemDefinition {
    let Some(batches) = pour.get_mut(&item.id) else {
        return item.clone();
    };
    let Some(best) = batches.last_mut() else {
        return item.clone();
    };
    let mut poured = item.clone();
    poured.quality = best.quality_score;
    for inherited in &best.traits {
        if !poured.traits.iter().any(|held| held == inherited) {
            poured.traits.push(inherited.clone());
        }
    }
    best.count = best.count.saturating_sub(1);
    batches.retain(|batch| batch.count > 0);
    poured
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{BottleBatchEntry, GameData, ItemCategory};

    /// A potion that some recipe asks for as a reagent, so these tests follow
    /// the content rather than pinning one bottle by name.
    fn a_bottle_used_as_a_reagent(data: &GameData) -> String {
        data.recipes
            .iter()
            .flat_map(|recipe| recipe.ingredients.iter())
            .map(|ingredient| ingredient.item_id.clone())
            .find(|item_id| {
                data.item(item_id)
                    .is_some_and(|item| item.category == ItemCategory::Potion)
            })
            .expect("some recipe should call for a finished bottle")
    }

    fn batch(item_id: &str, band: &str, score: u32, count: u32) -> BottleBatchEntry {
        BottleBatchEntry {
            item_id: item_id.to_owned(),
            quality_score: score,
            quality_band: band.to_owned(),
            traits: vec!["luminous".to_owned()],
            count,
        }
    }

    /// The gap the late tier shipped with: every potion leaves `quality` unset,
    /// so a poured bottle was worth the schema default of 20 whether it was
    /// Crude or Masterwork, and brewing the input well bought nothing.
    #[test]
    fn the_bottle_you_poured_is_the_bottle_the_brew_gets() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let item_id = a_bottle_used_as_a_reagent(&data);
        let selected = vec![item_id.clone()];

        state.inventory.insert(item_id.clone(), 1);
        let plain = state.brew_ingredients(&data, &selected);

        state
            .progression
            .bottle_stock
            .insert(item_id.clone(), vec![batch(&item_id, "Masterwork", 92, 1)]);
        let graded = state.brew_ingredients(&data, &selected);

        assert!(
            graded[0].quality > plain[0].quality,
            "a masterwork bottle poured in at quality {} against the plain {}",
            graded[0].quality,
            plain[0].quality
        );
        assert!(
            graded[0].traits.iter().any(|held| held == "luminous"),
            "the traits the bottle was brewed with did not reach the pot"
        );
    }

    /// The pot takes the best bottle, and only once. A recipe asking for two of
    /// the same solution when one of them is good gets the good one and then an
    /// ordinary one.
    #[test]
    fn one_good_bottle_improves_one_slot() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let item_id = a_bottle_used_as_a_reagent(&data);

        state.inventory.insert(item_id.clone(), 2);
        state.progression.bottle_stock.insert(
            item_id.clone(),
            vec![
                batch(&item_id, "Crude", 15, 1),
                batch(&item_id, "Masterwork", 92, 1),
            ],
        );

        let both = state.brew_ingredients(&data, &[item_id.clone(), item_id.clone()]);
        assert_eq!(
            both[0].quality, 92,
            "the first slot should get the good one"
        );
        assert_eq!(both[1].quality, 15, "the second slot should get the other");
    }

    /// Brewing has to spend the bottle it poured. `take_from_inventory` trims
    /// the *worst* batch, which is right for a sale and wrong here: pour a
    /// Masterwork and the shelf would quietly keep it and drop a Crude one, so
    /// the same bottle would improve every future brew of that recipe.
    #[test]
    fn brewing_spends_the_bottle_it_poured() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let item_id = a_bottle_used_as_a_reagent(&data);

        state.inventory.insert(item_id.clone(), 2);
        state.progression.bottle_stock.insert(
            item_id.clone(),
            vec![
                batch(&item_id, "Crude", 15, 1),
                batch(&item_id, "Masterwork", 92, 1),
            ],
        );

        // The real consume path, so the ordering it depends on is under test
        // rather than the helper on its own.
        state.consume_brew_inputs(&data, std::slice::from_ref(&item_id));

        let left = &state.progression.bottle_stock[&item_id];
        assert_eq!(left.len(), 1);
        assert_eq!(
            left[0].quality_band, "Crude",
            "the masterwork survived the brew that poured it"
        );
    }

    /// The whole reason the late tier exists: a compound brew is worth what its
    /// inputs were worth. Run every second-order recipe to its own spec twice —
    /// once with ordinary bottles and once with bottles brewed at Masterwork —
    /// and the second run has to score higher, or "brew the input well" is
    /// advice with nothing behind it.
    #[test]
    fn a_compound_brew_is_worth_what_its_inputs_were_worth() {
        use crate::alchemy::resolve_brew;

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut checked = 0usize;

        for recipe in &data.recipes {
            let bottles = recipe
                .ingredients
                .iter()
                .filter(|ingredient| {
                    data.item(&ingredient.item_id)
                        .is_some_and(|item| item.category == ItemCategory::Potion)
                })
                .map(|ingredient| ingredient.item_id.clone())
                .collect::<Vec<_>>();
            if bottles.is_empty() {
                continue;
            }
            let station = data
                .stations
                .iter()
                .find(|station| station.id == recipe.station_id)
                .expect("the recipe's bench");
            let selected = recipe
                .ingredients
                .iter()
                .flat_map(|ingredient| {
                    std::iter::repeat_n(ingredient.item_id.clone(), ingredient.amount as usize)
                })
                .collect::<Vec<_>>();

            let mut state = GameplayState::new(&data);
            for item_id in &selected {
                *state.inventory.entry(item_id.clone()).or_insert(0) += 1;
            }
            let brew = |state: &GameplayState| {
                resolve_brew(
                    &data,
                    station,
                    &selected,
                    &state.brew_ingredients(&data, &selected),
                    None,
                    recipe.required_heat,
                    recipe.required_stirs,
                    &recipe.required_timing,
                    0,
                )
                .quality_score
            };

            let ordinary = brew(&state);
            for item_id in &bottles {
                state
                    .progression
                    .bottle_stock
                    .insert(item_id.clone(), vec![batch(item_id, "Masterwork", 92, 1)]);
            }
            let well_made = brew(&state);

            assert!(
                well_made > ordinary,
                "{} scored {well_made} on masterwork reagents against {ordinary} on plain ones",
                recipe.id
            );
            checked += 1;
        }

        assert!(checked > 0, "no second-order recipe to check");
    }

    /// The materials list reads the quality of the bottle it would pour, not the
    /// item file's default. Without it the one decision the late tier asks for —
    /// brew the input well before folding it — is invisible at the bench.
    #[test]
    fn the_materials_list_grades_the_bottle_on_the_shelf() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let item_id = a_bottle_used_as_a_reagent(&data);
        let authored = data.item(&item_id).expect("the reagent").quality;

        state.inventory.insert(item_id.clone(), 1);
        assert_eq!(state.reagent_quality(&data, &item_id), authored);

        state
            .progression
            .bottle_stock
            .insert(item_id.clone(), vec![batch(&item_id, "Masterwork", 92, 1)]);
        assert_eq!(state.reagent_quality(&data, &item_id), 92);
    }
}
