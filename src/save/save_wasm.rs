use super::save_errors::{
    SAVE_ERROR_WASM_LOAD_UNAVAILABLE, SAVE_ERROR_WASM_SAVE_UNAVAILABLE,
};
use crate::data::SaveData;

pub(crate) fn save(_save_data: &SaveData) -> Result<(), String> {
    Err(SAVE_ERROR_WASM_SAVE_UNAVAILABLE.to_owned())
}

pub(crate) fn exists() -> bool {
    false
}

pub(crate) fn load() -> Result<SaveData, String> {
    Err(SAVE_ERROR_WASM_LOAD_UNAVAILABLE.to_owned())
}
