//! Which of the player's held units were gathered as wild variants, and what
//! that is worth at the bench.
//!
//! Gathering under the right sky used to change a string. The variant was
//! computed at pickup, written into the herb journal's best-seen line, and
//! thrown away — inventory counts units and had nowhere to record that one unit
//! of quarry lichen came up sparked. So `bonus_traits`, `elements` and the two
//! synthesis bonuses on all forty variants were dead, and `quality_bonus`
//! reached the journal but never the brew.
//!
//! Stock is tracked alongside the plain count rather than replacing it: the
//! inventory still says how many you hold, and this says how many of those were
//! the good ones. Brewing spends the best variant first and folds its bonuses
//! into the ingredient it stands in for, which is enough for every downstream
//! calculation — quality, traits, elements, volatility, synthesis — to pick the
//! difference up without knowing variants exist.

use super::gameplay_reagent_bottles::pour_bottle;
use super::GameplayState;
use crate::data::{GameData, ItemCategory, ItemDefinition, WildVariantDefinition};

impl GameplayState {
    /// Record that one gathered unit of `item_id` came up as `variant_id`.
    pub(super) fn note_variant_gathered(&mut self, item_id: &str, variant_id: &str) {
        if variant_id.is_empty() {
            return;
        }
        *self
            .progression
            .variant_stock
            .entry(item_id.to_owned())
            .or_default()
            .entry(variant_id.to_owned())
            .or_insert(0) += 1;
    }

    /// The best variant currently held for this ingredient, if any. "Best" is
    /// the largest quality bonus, which is also how the herb journal ranks them.
    pub(super) fn best_held_variant<'a>(
        &self,
        data: &'a GameData,
        item_id: &str,
    ) -> Option<&'a WildVariantDefinition> {
        let held = self.progression.variant_stock.get(item_id)?;
        let item = data.item(item_id)?;
        item.wild_variants
            .iter()
            .filter(|variant| held.get(&variant.id).copied().unwrap_or_default() > 0)
            .max_by_key(|variant| variant.quality_bonus)
    }

    /// The best strain currently in the bag for this ingredient, and how many
    /// of them there are. The journal has recorded the best strain ever *seen*
    /// since variants were authored; what it could not say is whether the
    /// player is carrying one right now, which is the half that decides whether
    /// to walk back out or brew with what is in the bag.
    pub(super) fn held_variant_summary(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<(String, u32)> {
        let variant = self.best_held_variant(data, item_id)?;
        let count = self
            .progression
            .variant_stock
            .get(item_id)
            .and_then(|held| held.get(&variant.id))
            .copied()
            .unwrap_or_default();
        (count > 0).then(|| (variant.name.clone(), count))
    }

    /// Spend one held unit of `variant_id`, dropping the entry when it runs out
    /// so `best_held_variant` stops offering something that is gone.
    pub(super) fn spend_variant_unit(&mut self, item_id: &str, variant_id: &str) {
        let Some(held) = self.progression.variant_stock.get_mut(item_id) else {
            return;
        };
        if let Some(count) = held.get_mut(variant_id) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                held.remove(variant_id);
            }
        }
        if held.is_empty() {
            self.progression.variant_stock.remove(item_id);
        }
    }

    /// The ingredients a brew of `selected_items` would actually be made from,
    /// with the best held variant folded into each herb and the best held bottle
    /// folded into each potion. Returned owned because neither a variant-grade
    /// reagent nor a graded bottle is any `ItemDefinition` the data file holds.
    pub(super) fn brew_ingredients(
        &self,
        data: &GameData,
        selected_items: &[String],
    ) -> Vec<ItemDefinition> {
        // One unit of each variant can only be spent once, so a brew calling for
        // two of the same herb gets the variant for the first and the plain herb
        // for the second. Bottles work the same way, best first.
        let mut remaining = self.progression.variant_stock.clone();
        let mut pour = self.reagent_bottle_pour();
        selected_items
            .iter()
            .filter_map(|item_id| {
                let item = data.item(item_id)?;
                if item.category == ItemCategory::Potion {
                    return Some(pour_bottle(item, &mut pour));
                }
                let variant = self
                    .best_held_variant(data, item_id)
                    .filter(|variant| {
                        remaining
                            .get(item_id)
                            .and_then(|held| held.get(&variant.id))
                            .copied()
                            .unwrap_or_default()
                            > 0
                    })
                    .cloned();
                let Some(variant) = variant else {
                    return Some(item.clone());
                };
                if let Some(count) = remaining
                    .get_mut(item_id)
                    .and_then(|held| held.get_mut(&variant.id))
                {
                    *count = count.saturating_sub(1);
                }
                Some(apply_variant(item, &variant))
            })
            .collect()
    }

    /// Spend the variant units a brew of `selected_items` just used up. Mirrors
    /// the choice `brew_ingredients` made, so what was paid for is what was
    /// consumed.
    pub(super) fn spend_brew_variants(&mut self, data: &GameData, selected_items: &[String]) {
        for item_id in selected_items {
            let Some(variant_id) = self
                .best_held_variant(data, item_id)
                .map(|variant| variant.id.clone())
            else {
                continue;
            };
            self.spend_variant_unit(item_id, &variant_id);
        }
    }
}

/// An ingredient as the variant makes it. Every field a variant carries lands on
/// the reagent itself, so the rest of the brewer needs no notion of variants.
fn apply_variant(item: &ItemDefinition, variant: &WildVariantDefinition) -> ItemDefinition {
    let mut adjusted = item.clone();
    adjusted.quality = (adjusted.quality + variant.quality_bonus).min(100);
    for extra in &variant.bonus_traits {
        if !adjusted.traits.iter().any(|held| held == extra) {
            adjusted.traits.push(extra.clone());
        }
    }
    adjusted.elements.add_assign(&variant.elements);
    adjusted.synthesis_weight = adjusted
        .synthesis_weight
        .saturating_add(variant.synthesis_weight_bonus);
    adjusted.synthesis_value = adjusted
        .synthesis_value
        .saturating_add(variant.synthesis_value_bonus);
    adjusted
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::GameData;

    /// The first item in the data that has a variant worth something, so these
    /// tests follow the content rather than pinning one herb by name.
    fn an_item_with_a_variant(data: &GameData) -> (String, String) {
        let item = data
            .items
            .iter()
            .find(|item| {
                item.wild_variants
                    .iter()
                    .any(|variant| variant.quality_bonus > 0)
            })
            .expect("some ingredient should have a variant worth gathering for");
        let variant = item
            .wild_variants
            .iter()
            .max_by_key(|variant| variant.quality_bonus)
            .expect("the variant just found");
        (item.id.clone(), variant.id.clone())
    }

    /// A variant used to be a string in the journal. Gathering one under the
    /// right sky recorded the name against the herb's best-seen line and threw
    /// the unit's quality away, because inventory counts units and had no way
    /// to say one of them was better. This asserts the pickup reaches the pot.
    #[test]
    fn a_variant_gathered_under_the_right_sky_reaches_the_bench() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let (item_id, variant_id) = an_item_with_a_variant(&data);
        let selected = vec![item_id.clone()];

        let plain = state.brew_ingredients(&data, &selected);
        state.note_variant_gathered(&item_id, &variant_id);
        let improved = state.brew_ingredients(&data, &selected);

        assert!(
            improved[0].quality > plain[0].quality,
            "the variant unit brews no better than the plain herb"
        );
        assert!(improved[0].traits.len() >= plain[0].traits.len());
    }

    /// One unit is one unit. A recipe calling for two of the same herb when only
    /// one variant was gathered should get the good one and then a plain one,
    /// not the good one counted twice.
    #[test]
    fn a_single_variant_unit_is_only_spent_once() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let (item_id, variant_id) = an_item_with_a_variant(&data);
        state.note_variant_gathered(&item_id, &variant_id);

        let both = state.brew_ingredients(&data, &[item_id.clone(), item_id.clone()]);
        let base = data.item(&item_id).expect("the item").quality;
        assert!(
            both[0].quality > base,
            "the first slot should get the good one"
        );
        assert_eq!(both[1].quality, base, "the second slot should be plain");
    }

    /// The gap this system shipped with, named in the TODO for six passes: the
    /// belt shows one stack per id, the bench quietly spends the best unit in
    /// it, and *nothing on screen said which stacks held one*. The player was
    /// making the one decision the system exists for — brew now or walk back
    /// out for a better strain — with no information at all.
    ///
    /// Both surfaces are checked here because they answer different questions:
    /// the bench says which stack is worth loading, the journal says what is in
    /// the bag and how much of it.
    #[test]
    fn a_held_variant_is_visible_at_the_bench_and_in_the_journal() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let (item_id, variant_id) = an_item_with_a_variant(&data);
        state.inventory.insert(item_id.clone(), 3);

        let plain_quality = state.reagent_quality(&data, &item_id);
        let plain_title = state
            .alchemy_materials_panel_view(&data)
            .rows
            .into_iter()
            .find(|row| row.title.starts_with(data.item_name(&item_id)))
            .map(|row| row.title)
            .expect("the herb should be listed at the bench");

        state.note_variant_gathered(&item_id, &variant_id);
        state.note_variant_gathered(&item_id, &variant_id);

        assert!(
            state.reagent_quality(&data, &item_id) > plain_quality,
            "the bench still reads the plain quality for a stack holding a variant"
        );
        let marked_title = state
            .alchemy_materials_panel_view(&data)
            .rows
            .into_iter()
            .find(|row| row.title.starts_with(data.item_name(&item_id)))
            .map(|row| row.title)
            .expect("the herb should still be listed");
        assert_ne!(
            marked_title, plain_title,
            "the row reads the same whether or not the stack holds a variant"
        );

        let (name, count) = state
            .held_variant_summary(&data, &item_id)
            .expect("two gathered units should be held");
        assert_eq!(count, 2, "the journal should count what is in the bag");
        assert!(!name.is_empty(), "the strain should be named");
    }

    /// Brewing consumes the variant along with the herb. Without this the same
    /// lucky pickup would improve every future brew of that recipe forever.
    #[test]
    fn brewing_spends_the_variant_it_used() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let (item_id, variant_id) = an_item_with_a_variant(&data);
        state.note_variant_gathered(&item_id, &variant_id);
        assert!(state.best_held_variant(&data, &item_id).is_some());

        state.spend_brew_variants(&data, std::slice::from_ref(&item_id));
        assert!(
            state.best_held_variant(&data, &item_id).is_none(),
            "the variant survived the brew that used it"
        );
        assert!(
            !state.progression.variant_stock.contains_key(&item_id),
            "an emptied entry should be dropped rather than left at zero"
        );
    }
}
