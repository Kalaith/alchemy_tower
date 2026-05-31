use crate::content::ui_format;
use crate::data::GameData;

pub(super) fn toast(data: &GameData, planted_item_id: &str) -> String {
    ui_format(
        "progression_planter_mutation",
        &[("item", data.item_name(planted_item_id))],
    )
}

pub(super) fn status(data: &GameData, catalyst_item_id: &str, mutation_note: &str) -> String {
    ui_format(
        "progression_planter_mutation_status",
        &[
            ("catalyst", data.item_name(catalyst_item_id)),
            ("strain", mutation_note),
        ],
    )
}
