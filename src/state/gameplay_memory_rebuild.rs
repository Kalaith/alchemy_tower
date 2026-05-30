use super::GameplayState;
use crate::data::{GameData, ItemCategory};

impl GameplayState {
    pub(super) fn rebuild_memory_state(&mut self, data: &GameData) {
        let inventory_items = self.inventory.keys().cloned().collect::<Vec<_>>();
        for item_id in inventory_items {
            self.note_inventory_observation(data, &item_id);
        }

        if self.progression.potion_memories.is_empty() {
            for entry in self.progression.experiment_log.clone() {
                if data
                    .item(&entry.output_item_id)
                    .map(|item| item.category == ItemCategory::Potion)
                    .unwrap_or(false)
                {
                    self.record_potion_result_memory(
                        &entry.output_item_id,
                        (!entry.recipe_id.is_empty()).then_some(entry.recipe_id.as_str()),
                        entry.stable,
                        entry.quality_score,
                        &entry.quality_band,
                    );
                }
            }
        }

        for recipe in &data.recipes {
            if self.progression.known_recipes.contains(&recipe.id) {
                self.ensure_potion_memory_learned(&recipe.output_item_id, Some(&recipe.id));
            }
        }

        let crafted_profiles = self
            .progression
            .crafted_item_profiles
            .values()
            .cloned()
            .collect::<Vec<_>>();
        for profile in crafted_profiles {
            if data
                .item(&profile.item_id)
                .map(|item| item.category == ItemCategory::Potion)
                .unwrap_or(false)
            {
                self.ensure_potion_memory_learned(&profile.item_id, None);
                if let Some(memory) = self.progression.potion_memories.get_mut(&profile.item_id) {
                    if profile.best_quality_score >= memory.best_quality_score {
                        memory.best_quality_score = profile.best_quality_score;
                        memory.best_quality_band = profile.best_quality_band.clone();
                    }
                }
            }
        }
    }
}
