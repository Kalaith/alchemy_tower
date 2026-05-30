use super::GameplayState;
use crate::content::ui_text;
use crate::data::{GameData, StationKind};
use macroquad::prelude::{is_key_pressed, KeyCode};

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
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::E) {
                self.clear_overlay();
            }
            return;
        }
        if is_key_pressed(KeyCode::Up) {
            self.ui.rune_index = self.ui.rune_index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            self.ui.rune_index = (self.ui.rune_index + 1).min(recipes.len().saturating_sub(1));
        }
        if is_key_pressed(KeyCode::Enter) {
            if let Some(recipe) = recipes.get(self.ui.rune_index) {
                self.apply_rune_recipe(data, recipe);
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.clear_overlay();
            self.runtime.status_text = ui_text().statuses.closed_rune.clone();
        }
    }
}
