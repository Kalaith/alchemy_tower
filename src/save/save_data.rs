//! Save repository abstraction with native and wasm-safe behavior.

use std::path::{Path, PathBuf};

use crate::data::SaveData;

pub struct SaveRepository;

impl SaveRepository {
    const SAVE_PATH: &'static str = "save_slot_0.json";
    const SAVE_DIR: &'static str = "AlchemyTower";

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(save_data: &SaveData) -> Result<(), String> {
        let json = serde_json::to_string_pretty(save_data).map_err(|error| error.to_string())?;
        let save_path = Self::save_path()?;
        let save_dir = save_path
            .parent()
            .ok_or_else(|| "Save path has no parent directory".to_owned())?;
        std::fs::create_dir_all(save_dir).map_err(|error| error.to_string())?;

        let temp_path = save_path.with_extension("json.tmp");
        std::fs::write(&temp_path, json).map_err(|error| error.to_string())?;
        Self::replace_atomically(&temp_path, &save_path)
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
        let base_dir = std::env::var_os("LOCALAPPDATA")
            .or_else(|| std::env::var_os("APPDATA"))
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| {
                    let mut path = PathBuf::from(home);
                    path.push(".local");
                    path.push("share");
                    path
                })
            })
            .ok_or_else(|| "Could not resolve a user data directory for saves".to_owned())?;

        Ok(base_dir.join(Self::SAVE_DIR).join(Self::SAVE_PATH))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn replace_atomically(temp_path: &Path, save_path: &Path) -> Result<(), String> {
        if save_path.exists() {
            std::fs::remove_file(save_path).map_err(|error| error.to_string())?;
        }
        std::fs::rename(temp_path, save_path).map_err(|error| error.to_string())
    }
}
