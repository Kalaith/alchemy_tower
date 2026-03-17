//! Embedded data loading.

use crate::data::GameData;

pub struct GameDataLoader;

impl GameDataLoader {
    pub fn load_embedded() -> Result<GameData, serde_json::Error> {
        let mut data: GameData =
            serde_json::from_str(include_str!("../../assets/data/game_data.json"))?;
        data.build_indexes();
        Ok(data)
    }
}
