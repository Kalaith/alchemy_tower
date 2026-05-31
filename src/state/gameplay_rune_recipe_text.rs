use crate::content::ui_format;
use crate::data::{GameData, RuneRecipeDefinition};

pub(super) fn imbued(data: &GameData, recipe: &RuneRecipeDefinition) -> String {
    ui_format(
        "rune_imbued_status",
        &[
            ("input", data.item_name(&recipe.input_item_id)),
            ("rune", data.item_name(&recipe.rune_item_id)),
            ("output", data.item_name(&recipe.output_item_id)),
        ],
    )
}
