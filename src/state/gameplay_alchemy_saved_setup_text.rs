use crate::content::ui_format;
use crate::data::GameData;

pub(super) fn repeat_none() -> String {
    ui_format("alchemy_repeat_none", &[])
}

pub(super) fn repeat_missing(data: &GameData, item_id: &str, required_count: u32) -> String {
    ui_format(
        "alchemy_repeat_missing",
        &[
            ("name", data.item_name(item_id)),
            ("count", &required_count.to_string()),
        ],
    )
}

pub(super) fn repeat_loaded() -> String {
    ui_format("alchemy_repeat_loaded", &[])
}
