use super::save_codec::{decode_save, encode_save};
use crate::data::SaveData;

const SAVE_GAME_NAME: &str = "AlchemyTower";
const SAVE_SLOT_KEY: &str = "save_slot_0";

pub(crate) fn save(save_data: &SaveData) -> Result<(), String> {
    let json = encode_save(save_data)?;
    macroquad_toolkit::persistence::save_string_key(SAVE_GAME_NAME, SAVE_SLOT_KEY, &json)
}

pub(crate) fn exists() -> bool {
    macroquad_toolkit::persistence::json_key_exists(SAVE_GAME_NAME, SAVE_SLOT_KEY)
}

pub(crate) fn load() -> Result<SaveData, String> {
    let json = macroquad_toolkit::persistence::load_string_key(SAVE_GAME_NAME, SAVE_SLOT_KEY)?;
    decode_save(&json)
}
