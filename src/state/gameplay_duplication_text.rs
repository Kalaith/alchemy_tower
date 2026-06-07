use crate::content::ui_format;
use crate::data::GameData;

pub(super) fn unstable() -> String {
    ui_format("progression_duplicate_unstable", &[])
}

pub(super) fn resists(item_name: &str) -> String {
    ui_format("progression_duplicate_resists", &[("name", item_name)])
}

pub(super) fn missing_source(item_name: &str) -> String {
    ui_format("progression_duplicate_none", &[("name", item_name)])
}

pub(super) fn need_coins(item_name: &str, missing_coins: u32) -> String {
    ui_format(
        "progression_duplicate_need_coins",
        &[("coins", &missing_coins.to_string()), ("name", item_name)],
    )
}

pub(super) fn need_catalyst() -> String {
    ui_format("progression_duplicate_need_catalyst", &[])
}

pub(super) fn toast(item_name: &str) -> String {
    ui_format("progression_duplicate_toast", &[("name", item_name)])
}

pub(super) fn duplicated(
    data: &GameData,
    item_name: &str,
    cost: u32,
    catalyst_item_id: &str,
) -> String {
    ui_format(
        "progression_duplicate_status",
        &[
            ("name", item_name),
            ("cost", &cost.to_string()),
            ("catalyst", data.item_name(catalyst_item_id)),
        ],
    )
}
