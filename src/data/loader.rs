//! Embedded data loading.

#[path = "loader_embedded.rs"]
mod loader_embedded;

use self::loader_embedded::load_embedded_parts;
use crate::data::GameData;

pub(crate) fn load_embedded_or_fallback() -> GameData {
    match load_embedded() {
        Ok(data) => data,
        Err(error) => handle_embedded_load_failure(error),
    }
}

pub(crate) fn load_embedded() -> Result<GameData, String> {
    let parts = load_embedded_parts()?;
    GameData::from_parts(parts)
}

#[cfg(debug_assertions)]
fn handle_embedded_load_failure(error: String) -> GameData {
    eprintln!("Failed to load embedded game data: {error}");
    GameData::runtime_fallback()
}

#[cfg(not(debug_assertions))]
fn handle_embedded_load_failure(error: String) -> GameData {
    panic!("Failed to load embedded game data: {error}");
}
