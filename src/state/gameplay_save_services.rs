use super::gameplay_persistence;
use super::GameplayState;
use crate::data::GameData;

#[path = "gameplay_save_status_text.rs"]
mod save_status_text;

impl GameplayState {
    pub(crate) fn saved_progress_exists() -> bool {
        gameplay_persistence::save_slot_exists()
    }

    pub(crate) fn save_progress(&mut self, data: &GameData) {
        self.runtime.status_text = match gameplay_persistence::save_slot(self, data) {
            Ok(()) => save_status_text::saved(),
            Err(error) => save_status_text::save_failed(&error),
        };
    }

    pub(crate) fn load_progress(&mut self, data: &GameData) -> bool {
        match gameplay_persistence::load_slot(self, data) {
            Ok(()) => {
                self.runtime.status_text = save_status_text::loaded();
                true
            }
            Err(error) => {
                self.runtime.status_text = save_status_text::load_failed(&error);
                false
            }
        }
    }

    pub(crate) fn pause_status_text(&self) -> &str {
        &self.runtime.status_text
    }
}
