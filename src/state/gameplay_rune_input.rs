use super::GameplayState;
use crate::data::{GameData, StationKind};
use crate::input::{
    cancel_pressed, confirm_pressed, interact_pressed, select_next_pressed, select_previous_pressed,
};

#[path = "gameplay_rune_input_text.rs"]
mod rune_input_text;

impl GameplayState {
    pub(super) fn handle_rune_inputs(&mut self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            self.clear_overlay();
            return;
        };
        if station.kind != StationKind::RuneWorkshop {
            self.clear_overlay();
            return;
        }
        let recipes = self.available_rune_recipes(data, station);
        if recipes.is_empty() {
            if cancel_pressed() || interact_pressed() {
                self.clear_overlay();
            }
            return;
        }
        if select_previous_pressed() {
            self.ui.rune_index = self.ui.rune_index.saturating_sub(1);
        }
        if select_next_pressed() {
            self.ui.rune_index = (self.ui.rune_index + 1).min(recipes.len().saturating_sub(1));
        }
        if confirm_pressed() {
            if let Some(recipe) = recipes.get(self.ui.rune_index) {
                self.apply_rune_recipe(data, recipe);
            }
        }
        if cancel_pressed() {
            self.clear_overlay();
            self.runtime.status_text = rune_input_text::closed();
        }
    }
}
