use super::gameplay_persistence;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::save::{
    SAVE_ERROR_USER_DATA_DIR_MISSING, SAVE_ERROR_WASM_LOAD_UNAVAILABLE,
    SAVE_ERROR_WASM_SAVE_UNAVAILABLE,
};

impl GameplayState {
    pub(crate) fn saved_progress_exists() -> bool {
        gameplay_persistence::save_slot_exists()
    }

    pub(crate) fn save_progress(&mut self, data: &GameData) {
        self.runtime.status_text = match gameplay_persistence::save_slot(self, data) {
            Ok(()) => ui_format("gameplay_saved_progress", &[]),
            Err(error) => {
                let message = save_error_message(&error);
                ui_format("gameplay_save_failed", &[("error", message.as_str())])
            }
        };
    }

    pub(crate) fn load_progress(&mut self, data: &GameData) -> bool {
        match gameplay_persistence::load_slot(self, data) {
            Ok(()) => {
                self.runtime.status_text = ui_format("gameplay_loaded_progress", &[]);
                true
            }
            Err(error) => {
                let message = save_error_message(&error);
                self.runtime.status_text =
                    ui_format("gameplay_load_failed", &[("error", message.as_str())]);
                false
            }
        }
    }

    pub(crate) fn pause_status_text(&self) -> &str {
        &self.runtime.status_text
    }
}

fn save_error_message(error: &str) -> String {
    match error {
        SAVE_ERROR_USER_DATA_DIR_MISSING => ui_copy(SAVE_ERROR_USER_DATA_DIR_MISSING).to_owned(),
        SAVE_ERROR_WASM_SAVE_UNAVAILABLE => ui_copy(SAVE_ERROR_WASM_SAVE_UNAVAILABLE).to_owned(),
        SAVE_ERROR_WASM_LOAD_UNAVAILABLE => ui_copy(SAVE_ERROR_WASM_LOAD_UNAVAILABLE).to_owned(),
        _ => error.to_owned(),
    }
}
