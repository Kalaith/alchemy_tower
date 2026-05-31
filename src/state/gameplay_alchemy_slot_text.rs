use crate::content::ui_format;
use crate::data::GameData;

pub(super) fn fill_slot_requires_ingredient(item_name: &str) -> String {
    ui_format("inventory_fill_slot_catalyst", &[("name", item_name)])
}

pub(super) fn no_more_ready(data: &GameData, item_id: &str) -> String {
    no_more_ready_name(data.item_name(item_id))
}

pub(super) fn no_more_ready_name(item_name: &str) -> String {
    ui_format("inventory_no_more_ready", &[("name", item_name)])
}

pub(super) fn added_slot(data: &GameData, item_id: &str, slot: usize) -> String {
    ui_format(
        "inventory_added_slot",
        &[
            ("item", data.item_name(item_id)),
            ("slot", &(slot + 1).to_string()),
        ],
    )
}

pub(super) fn fill_catalyst_invalid(item_name: &str) -> String {
    ui_format("inventory_fill_catalyst_invalid", &[("name", item_name)])
}

pub(super) fn prepared_catalyst(item_name: &str) -> String {
    ui_format("inventory_prepared_catalyst", &[("name", item_name)])
}
