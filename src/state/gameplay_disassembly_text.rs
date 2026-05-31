use crate::content::ui_format;
use crate::data::GameData;

pub(super) fn cannot_disassemble(recipe_name: &str) -> String {
    ui_format("progression_no_disassemble", &[("name", recipe_name)])
}

pub(super) fn returned_input(data: &GameData, item_id: &str, amount: u32) -> String {
    ui_format(
        "overlay_input_amount",
        &[
            ("item", data.item_name(item_id)),
            ("amount", &amount.to_string()),
        ],
    )
}

pub(super) fn toast(recipe_name: &str) -> String {
    ui_format("progression_disassembly_toast", &[("name", recipe_name)])
}

pub(super) fn disassembled(recipe_name: &str, returned_items: &[String]) -> String {
    ui_format(
        "progression_disassembled",
        &[("name", recipe_name), ("items", &returned_items.join(", "))],
    )
}
