use super::GameplayState;
use crate::content::{narrative_text, ui_format};
use crate::data::{GameData, RuneRecipeDefinition, StationDefinition};

impl GameplayState {
    pub(super) fn available_rune_recipes<'a>(
        &self,
        data: &'a GameData,
        station: &StationDefinition,
    ) -> Vec<&'a RuneRecipeDefinition> {
        data.rune_recipes
            .iter()
            .filter(|recipe| recipe.station_id == station.id)
            .filter(|recipe| self.has_rune_recipe_inputs(recipe))
            .collect()
    }

    pub(super) fn apply_rune_recipe(&mut self, data: &GameData, recipe: &RuneRecipeDefinition) {
        self.consume_rune_recipe_inputs(recipe);
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

    fn has_rune_recipe_inputs(&self, recipe: &RuneRecipeDefinition) -> bool {
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
    }

    fn consume_rune_recipe_inputs(&mut self, recipe: &RuneRecipeDefinition) {
        for item_id in [&recipe.input_item_id, &recipe.rune_item_id] {
            if let Some(amount) = self.inventory.get_mut(item_id) {
                *amount = amount.saturating_sub(1);
            }
        }
        self.inventory.retain(|_, amount| *amount > 0);
    }
}
