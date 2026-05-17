//! Save repository abstraction with native and wasm-safe behavior.

use std::path::PathBuf;

use crate::data::SaveData;

pub struct SaveRepository;

impl SaveRepository {
    const SAVE_PATH: &'static str = "save_slot_0.json";
    const SAVE_DIR: &'static str = "AlchemyTower";

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(save_data: &SaveData) -> Result<(), String> {
        let json = serde_json::to_string_pretty(save_data).map_err(|error| error.to_string())?;
        macroquad_toolkit::persistence::save_string_atomic(Self::save_path()?, &json)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save(_save_data: &SaveData) -> Result<(), String> {
        Err("Save is not wired for wasm yet".to_owned())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Result<SaveData, String> {
        let json =
            std::fs::read_to_string(Self::save_path()?).map_err(|error| error.to_string())?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load() -> Result<SaveData, String> {
        Err("Load is not wired for wasm yet".to_owned())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_path() -> Result<PathBuf, String> {
        macroquad_toolkit::persistence::get_app_data_path(Self::SAVE_DIR, Self::SAVE_PATH)
            .ok_or_else(|| "Could not resolve a user data directory for saves".to_owned())
    }
}
