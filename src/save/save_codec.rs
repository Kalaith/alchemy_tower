use crate::data::SaveData;

pub(super) fn encode_save(save_data: &SaveData) -> Result<String, String> {
    serde_json::to_string_pretty(save_data).map_err(|error| error.to_string())
}

pub(super) fn decode_save(json: &str) -> Result<SaveData, String> {
    serde_json::from_str(json).map_err(|error| error.to_string())
}
