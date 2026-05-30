use super::GameplayState;
use crate::data::GameData;
use macroquad::prelude::{is_key_pressed, KeyCode};

impl GameplayState {
    pub(super) fn handle_potion_inputs(&mut self, data: &GameData) {
        let potions = self.quick_potions(data);
        for (index, item_id) in potions.iter().take(3).enumerate() {
            let key = [KeyCode::Z, KeyCode::X, KeyCode::C][index];
            if is_key_pressed(key) {
                self.consume_potion(data, item_id);
                return;
            }
        }
    }
}
