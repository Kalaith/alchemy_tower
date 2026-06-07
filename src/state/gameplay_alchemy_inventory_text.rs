use crate::content::{narrative_text, ui_format};
use crate::data::GameData;

pub(super) fn new_best_brew(data: &GameData, item_id: &str, quality_band: &str) -> String {
    ui_format(
        "inventory_new_best",
        &[("item", data.item_name(item_id)), ("band", quality_band)],
    )
}

pub(super) fn greenhouse_unlock_toast() -> String {
    ui_format("inventory_greenhouse_unlock", &[])
}

pub(super) fn greenhouse_unlock_status() -> String {
    narrative_text().statuses.greenhouse_unlock.clone()
}
