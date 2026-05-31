use crate::content::{ui_copy, ui_format};
use crate::save::{
    SAVE_ERROR_USER_DATA_DIR_MISSING, SAVE_ERROR_WASM_LOAD_UNAVAILABLE,
    SAVE_ERROR_WASM_SAVE_UNAVAILABLE,
};

pub(super) fn saved() -> String {
    ui_format("gameplay_saved_progress", &[])
}

pub(super) fn save_failed(error: &str) -> String {
    ui_format(
        "gameplay_save_failed",
        &[("error", save_error_message(error).as_str())],
    )
}

pub(super) fn loaded() -> String {
    ui_format("gameplay_loaded_progress", &[])
}

pub(super) fn load_failed(error: &str) -> String {
    ui_format(
        "gameplay_load_failed",
        &[("error", save_error_message(error).as_str())],
    )
}

fn save_error_message(error: &str) -> String {
    match error {
        SAVE_ERROR_USER_DATA_DIR_MISSING => ui_copy(SAVE_ERROR_USER_DATA_DIR_MISSING).to_owned(),
        SAVE_ERROR_WASM_SAVE_UNAVAILABLE => ui_copy(SAVE_ERROR_WASM_SAVE_UNAVAILABLE).to_owned(),
        SAVE_ERROR_WASM_LOAD_UNAVAILABLE => ui_copy(SAVE_ERROR_WASM_LOAD_UNAVAILABLE).to_owned(),
        _ => error.to_owned(),
    }
}
