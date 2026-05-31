use crate::content::ui_format;
use crate::data::GameData;

pub(super) fn potion_used(item_name: &str) -> String {
    ui_format("inventory_used", &[("name", item_name)])
}

pub(super) fn not_enough_coins(data: &GameData, item_id: &str) -> String {
    ui_format(
        "inventory_not_enough_coins",
        &[("item", data.item_name(item_id))],
    )
}

pub(super) fn bought(data: &GameData, item_id: &str) -> String {
    ui_format("inventory_bought", &[("item", data.item_name(item_id))])
}

pub(super) fn sold(item_name: &str, price: u32) -> String {
    ui_format(
        "inventory_sold",
        &[("name", item_name), ("price", &price.to_string())],
    )
}

pub(super) fn sold_safe(item_name: &str, price: u32) -> String {
    ui_format(
        "inventory_sold_safe",
        &[("name", item_name), ("price", &price.to_string())],
    )
}
