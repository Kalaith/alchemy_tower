use super::GameplayState;
use crate::data::{GameData, ItemCategory, PlanterStateEntry};

#[path = "gameplay_planter_mutation_text.rs"]
mod mutation_text;

impl GameplayState {
    /// Which formula a bed would take and which bottle it would eat.
    ///
    /// A mutation asks for an effect kind rather than a named brew — the one
    /// place in the game that already worked that way — so any glow bottle in
    /// the bag will do. It used to take the *first* one it found while walking
    /// the inventory, which is a `BTreeMap` and therefore alphabetical by item
    /// id: planting a bed could spend a 284-coin Heldstar Vigil because `h`
    /// sorts before `k`, while a Kindling Tonic worth 22 sat beside it.
    ///
    /// Every other spend in the game already knows better — a delivery hands
    /// over the worst bottle that qualifies, a sale parts with the worst held,
    /// the bench pours the best deliberately — so this takes the cheapest thing
    /// that fits, which is also exactly what the sinkless tail is for.
    pub(super) fn planter_mutation_candidate(
        &self,
        data: &GameData,
        planted_item_id: &str,
    ) -> Option<(String, String)> {
        for formula in data.mutation_formulas_for_seed(planted_item_id) {
            let cheapest = self
                .inventory
                .iter()
                .filter(|(item_id, amount)| **amount > 0 && *item_id != planted_item_id)
                .filter_map(|(item_id, _)| Some((item_id, data.item(item_id)?)))
                .filter(|(_, item)| item.category == ItemCategory::Potion)
                .filter(|(_, item)| {
                    formula.required_effect_kind.is_empty()
                        || item
                            .effects
                            .iter()
                            .any(|effect| effect.kind.as_str() == formula.required_effect_kind)
                })
                .min_by_key(|(item_id, item)| (item.base_value, (*item_id).clone()));
            if let Some((item_id, _)) = cheapest {
                return Some((formula.id.clone(), item_id.clone()));
            }
        }
        None
    }

    pub(super) fn apply_planter_mutation(
        &mut self,
        data: &GameData,
        state: &mut PlanterStateEntry,
        candidate: Option<&(String, String)>,
    ) -> Option<String> {
        if !state.mutation_formula_id.is_empty() {
            return None;
        }
        let (formula_id, catalyst_item_id) = candidate?;
        let formula = data
            .mutation_formulas
            .iter()
            .find(|formula| formula.id == *formula_id)?;

        self.inventory.get(catalyst_item_id)?;
        self.take_from_inventory(catalyst_item_id, 1);

        state.mutation_formula_id = formula.id.clone();
        state.mutation_yield_bonus = formula.yield_bonus;
        state.mutation_growth_bonus_days = formula.growth_bonus_days;
        state.mutation_note = formula.mutation_note.clone();

        // Name the bottle as well as the bed. A mutation costs a brew, and
        // until the banners were fixed the only thing the player was told was
        // that something had changed colour.
        self.trigger_planter_mutation_feedback(mutation_text::toast(
            data,
            &state.planted_item_id,
            catalyst_item_id,
        ));

        Some(mutation_text::status(
            data,
            catalyst_item_id,
            &formula.mutation_note,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{GameData, PlanterStateEntry};

    #[test]
    fn planter_mutation_consumes_matching_potion_and_stores_bonus() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        let mut planter = PlanterStateEntry {
            station_id: "greenhouse_planter_east".to_owned(),
            planted_item_id: "moon_fern".to_owned(),
            planted_day: 0,
            ready: false,
            tended_day: 0,
            tended_days: 0,
            growth_days: 0,
            mutation_formula_id: String::new(),
            mutation_yield_bonus: 0,
            mutation_growth_bonus_days: 0,
            mutation_note: String::new(),
        };

        state.inventory.insert("glow_potion".to_owned(), 1);

        let candidate = state.planter_mutation_candidate(&data, "moon_fern");
        let text = state.apply_planter_mutation(&data, &mut planter, candidate.as_ref());

        assert!(text.is_some());
        assert_eq!(planter.mutation_formula_id, "moon_fern_glow_mutation");
        assert_eq!(planter.mutation_yield_bonus, 1);
        assert_eq!(planter.mutation_growth_bonus_days, 1);
        assert_eq!(
            state
                .inventory
                .get("glow_potion")
                .copied()
                .unwrap_or_default(),
            0
        );
    }

    /// A bed asks for an effect kind, so any glow bottle in the bag will do —
    /// and it used to eat whichever one sorted first by item id, because the
    /// inventory is a `BTreeMap`. Holding a Heldstar Vigil (284 coins, `h`) and
    /// a Kindling Tonic (22 coins, `k`) meant planting a bed spent the vigil.
    ///
    /// Every other spend in the game reaches for the least valuable thing that
    /// qualifies, and this is where the sinkless tail is supposed to go.
    #[test]
    fn a_bed_eats_the_cheapest_bottle_that_fits() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        // Two glow brews where the *dear* one sorts first by id, because
        // sorting first by id is exactly what the old picker went by. Found
        // from the data rather than named, so a re-priced bottle cannot quietly
        // turn this into a test of nothing.
        let glow = data
            .items
            .iter()
            .filter(|item| item.category == crate::data::ItemCategory::Potion)
            .filter(|item| {
                item.effects
                    .iter()
                    .any(|effect| effect.kind.as_str() == "glow")
            })
            .collect::<Vec<_>>();
        let (dear, cheap) = glow
            .iter()
            .flat_map(|first| glow.iter().map(move |second| (first, second)))
            .find(|(first, second)| first.id < second.id && first.base_value > second.base_value)
            .map(|(first, second)| (*first, *second))
            .expect("some dear glow brew should sort before a cheaper one");

        state.inventory.insert(dear.id.clone(), 1);
        state.inventory.insert(cheap.id.clone(), 1);

        let (_, taken) = state
            .planter_mutation_candidate(&data, "moon_fern")
            .expect("a glow bottle should satisfy the moon fern formula");
        assert_eq!(
            taken, cheap.id,
            "the bed reached past the cheap bottle for the expensive one"
        );
    }

    /// The murky concoction is the one bottle in the game worth two coins and
    /// wanted by nothing: it is the misfire, and no formula asked for one. It
    /// has a home now, and the ordering is the design — a bed prefers a proper
    /// brew and takes the unlabelled one only when that is all there is, which
    /// is what `mutation_formulas_for_seed` returning data order means.
    #[test]
    fn the_bottle_nobody_can_identify_is_still_worth_something_to_a_bed() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        state.inventory.insert("murky_concoction".to_owned(), 1);
        let (formula_id, taken) = state
            .planter_mutation_candidate(&data, "moon_fern")
            .expect("a misfire should be worth something to a bed");
        assert_eq!(taken, "murky_concoction");
        assert!(
            formula_id.contains("misfire"),
            "the murky bottle matched {formula_id} rather than a misfire strain"
        );

        // Hand the bed a proper glow brew as well and it takes that instead.
        state.inventory.insert("glow_potion".to_owned(), 1);
        let (formula_id, taken) = state
            .planter_mutation_candidate(&data, "moon_fern")
            .expect("a glow brew should still satisfy the clean formula");
        assert_eq!(taken, "glow_potion");
        assert!(!formula_id.contains("misfire"));
    }

    /// The banner names the bottle as well as the bed. A mutation costs a brew,
    /// and while the toast channel was dead nobody could see that anything had
    /// been spent at all.
    #[test]
    fn the_bed_says_what_it_ate() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let mut planter = PlanterStateEntry {
            station_id: "greenhouse_planter_east".to_owned(),
            planted_item_id: "moon_fern".to_owned(),
            planted_day: 0,
            ready: false,
            tended_day: 0,
            tended_days: 0,
            growth_days: 0,
            mutation_formula_id: String::new(),
            mutation_yield_bonus: 0,
            mutation_growth_bonus_days: 0,
            mutation_note: String::new(),
        };
        state.inventory.insert("duskbell_tonic".to_owned(), 1);

        let candidate = state.planter_mutation_candidate(&data, "moon_fern");
        state.apply_planter_mutation(&data, &mut planter, candidate.as_ref());

        let banner = state
            .build_hud_toasts()
            .into_iter()
            .next()
            .expect("mutating a bed should raise a banner");
        assert!(
            banner.text.contains(data.item_name("duskbell_tonic")),
            "the banner does not say which bottle the bed took: {}",
            banner.text
        );
        assert!(
            banner.text.contains(data.item_name("moon_fern")),
            "the banner does not say which bed changed: {}",
            banner.text
        );
    }
}
