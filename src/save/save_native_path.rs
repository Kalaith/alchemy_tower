use std::path::PathBuf;

use super::save_errors::SAVE_ERROR_USER_DATA_DIR_MISSING;

const SAVE_PATH: &str = "save_slot_0.json";
const SAVE_DIR: &str = "AlchemyTower";

pub(super) fn save_path() -> Result<PathBuf, String> {
    macroquad_toolkit::persistence::get_app_data_path(SAVE_DIR, SAVE_PATH)
        .ok_or_else(|| SAVE_ERROR_USER_DATA_DIR_MISSING.to_owned())
}
