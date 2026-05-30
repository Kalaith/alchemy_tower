use super::{gameplay_persistence, GameplayState};
use crate::content::{narrative_text, ui_format};
use crate::data::{GameData, RuneRecipeDefinition, StationDefinition};

impl GameplayState {
    pub fn save_progress(&mut self, data: &GameData) {
        self.runtime.status_text =
            match gameplay_persistence::GameplayStateLoader::save_slot(self, data) {
                Ok(()) => ui_format("gameplay_saved_progress", &[]),
                Err(error) => ui_format("gameplay_save_failed", &[("error", &error)]),
            };
    }

    pub fn load_progress(&mut self, data: &GameData) -> bool {
        match gameplay_persistence::GameplayStateLoader::load_slot(self, data) {
            Ok(()) => {
                self.runtime.status_text = ui_format("gameplay_loaded_progress", &[]);
                true
            }
            Err(error) => {
                self.runtime.status_text = ui_format("gameplay_load_failed", &[("error", &error)]);
                false
            }
        }
    }

    pub fn pause_status_text(&self) -> &str {
        &self.runtime.status_text
    }

    pub(super) fn can_reconstruct_archive(&self) -> bool {
        self.progression
            .completed_quests
            .contains("star_elixir_for_ione")
            && self
                .progression
                .completed_quests
                .contains("containment_for_lyra")
            && self.has_journal_milestone("greenhouse_repaired")
            && self.has_journal_milestone("containment_repaired")
            && self.has_journal_milestone("rune_workshop_restored")
    }

    pub(super) fn available_rune_recipes<'a>(
        &self,
        data: &'a GameData,
        station: &StationDefinition,
    ) -> Vec<&'a RuneRecipeDefinition> {
        data.rune_recipes
            .iter()
            .filter(|recipe| recipe.station_id == station.id)
            .filter(|recipe| {
                self.inventory
                    .get(&recipe.input_item_id)
                    .copied()
                    .unwrap_or_default()
                    > 0
                    && self
                        .inventory
                        .get(&recipe.rune_item_id)
                        .copied()
                        .unwrap_or_default()
                        > 0
            })
            .collect()
    }

    pub(super) fn apply_rune_recipe(&mut self, data: &GameData, recipe: &RuneRecipeDefinition) {
        for item_id in [&recipe.input_item_id, &recipe.rune_item_id] {
            if let Some(amount) = self.inventory.get_mut(item_id) {
                *amount = amount.saturating_sub(1);
            }
        }
        self.inventory.retain(|_, amount| *amount > 0);
        *self
            .inventory
            .entry(recipe.output_item_id.clone())
            .or_insert(0) += 1;
        self.note_inventory_observation(data, &recipe.output_item_id);
        self.ensure_potion_memory_learned(&recipe.output_item_id, None);
        let milestone = &narrative_text().milestones.first_rune_imbuing;
        self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        self.runtime.status_text = ui_format(
            "rune_imbued_status",
            &[
                ("input", data.item_name(&recipe.input_item_id)),
                ("rune", data.item_name(&recipe.rune_item_id)),
                ("output", data.item_name(&recipe.output_item_id)),
            ],
        );
    }

}
