use super::GameplayState;
use crate::data::GameData;
use crate::input::{load_pressed, save_pressed};

impl GameplayState {
    pub(super) fn handle_save_shortcuts(&mut self, data: &GameData) {
        if save_pressed() {
            self.save_progress(data);
        }
        if load_pressed() {
            self.load_progress(data);
        }
    }
}
