use super::GameplayState;
use crate::alchemy::BrewResolution;
use crate::content::narrative_text;
use crate::data::{CraftedItemProfileEntry, GameData, ItemCategory};

impl GameplayState {
    pub(super) fn consume_brew_inputs(&mut self, data: &GameData, selected: &[String]) {
        // A poured bottle has to leave the shelf before the count drops, or the
        // reconcile inside `take_from_inventory` would trim the *worst* batch
        // and quietly keep the good bottle the bench just used.
        self.spend_brew_bottles(data, selected);
        for item_id in selected {
            self.take_from_inventory(item_id, 1);
        }
        if let Some(item_id) = self.selected_catalyst().map(str::to_owned) {
            self.take_from_inventory(&item_id, 1);
        }
    }

    pub(super) fn record_brew_inventory_result(
        &mut self,
        data: &GameData,
        resolution: &BrewResolution<'_>,
        stable_brew: bool,
    ) -> Option<CraftedItemProfileEntry> {
        *self
            .inventory
            .entry(resolution.output_item_id.clone())
            .or_insert(0) += resolution.output_amount;
        self.note_inventory_observation(data, &resolution.output_item_id);
        self.progression.total_brews += 1;
        // The bottles keep the quality they were brewed at.
        self.record_bottle_batch(resolution);
        self.record_experiment_log(resolution);
        if self.progression.total_brews == 1 {
            let milestone = &narrative_text().milestones.first_true_brew;
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }

        let previous_profile = self
            .progression
            .crafted_item_profiles
            .get(&resolution.output_item_id)
            .cloned();
        self.record_crafted_item_profile(data, &resolution.output_item_id, resolution);
        if data
            .item(&resolution.output_item_id)
            .map(|item| item.category == ItemCategory::Potion)
            .unwrap_or(false)
        {
            self.record_potion_result_memory(
                &resolution.output_item_id,
                resolution.recipe.map(|recipe| recipe.id.as_str()),
                stable_brew,
                resolution.quality_score,
                resolution.quality_band,
            );
        }
        previous_profile
    }
}
