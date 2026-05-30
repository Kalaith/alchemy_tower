use super::GameplayState;
use crate::data::GameData;
use crate::input::quick_potion_pressed;

impl GameplayState {
    pub(super) fn handle_potion_inputs(&mut self, data: &GameData) {
        let potions = self.quick_potions(data);
        for (index, item_id) in potions.iter().take(3).enumerate() {
            if quick_potion_pressed(index) {
                self.consume_potion(data, item_id);
                return;
            }
        }
    }
}
