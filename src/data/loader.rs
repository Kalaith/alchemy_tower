//! Embedded data loading.

#[path = "loader_embedded.rs"]
mod loader_embedded;

use crate::data::GameData;
use self::loader_embedded::load_embedded_parts;

pub(crate) fn load_embedded_or_fallback() -> GameData {
    load_embedded().unwrap_or_else(|error| {
        eprintln!("Failed to load embedded game data: {error}");
        GameData::runtime_fallback()
    })
}

pub(crate) fn load_embedded() -> Result<GameData, String> {
    let parts = load_embedded_parts()?;
    GameData::from_parts(parts)
}
