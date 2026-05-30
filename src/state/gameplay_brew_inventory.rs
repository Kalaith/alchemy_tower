use super::GameplayState;
use crate::alchemy::BrewResolution;
use crate::content::narrative_text;
use crate::data::{CraftedItemProfileEntry, GameData, ItemCategory};

impl GameplayState {
    pub(super) fn consume_brew_inputs(&mut self, selected: &[String]) {
        for item_id in selected {
            if let Some(amount) = self.inventory.get_mut(item_id) {
                *amount -= 1;
            }
        }
        if let Some(item_id) = self.selected_catalyst().map(str::to_owned) {
            if let Some(amount) = self.inventory.get_mut(&item_id) {
                *amount -= 1;
            }
        }
        self.inventory.retain(|_, amount| *amount > 0);
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
