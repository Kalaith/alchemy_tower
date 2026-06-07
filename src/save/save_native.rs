use super::save_codec::{decode_save, encode_save};
use super::save_native_path::save_path;
use crate::data::SaveData;

pub(crate) fn save(save_data: &SaveData) -> Result<(), String> {
    let json = encode_save(save_data)?;
    macroquad_toolkit::persistence::save_string_atomic(save_path()?, &json)
}

pub(crate) fn exists() -> bool {
    save_path().map(|path| path.exists()).unwrap_or_default()
}

pub(crate) fn load() -> Result<SaveData, String> {
    let json = std::fs::read_to_string(save_path()?).map_err(|error| error.to_string())?;
    decode_save(&json)
}
