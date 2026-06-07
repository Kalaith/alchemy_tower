use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::archive::{ArchiveDisassemblyRecipeEntry, ArchiveDisassemblySectionView};

impl GameplayState {
    pub(super) fn archive_disassembly_section_view(
        &self,
        data: &GameData,
    ) -> ArchiveDisassemblySectionView {
        let recipes = self.available_disassembly_recipes(data);
        if recipes.is_empty() {
            return ArchiveDisassemblySectionView {
                title: ui_copy("overlay_disassembly").to_string(),
                selected_inputs_title: ui_copy("overlay_recovered_inputs").to_string(),
                help_text: disassembly_help_text(),
                empty_text: self
                    .unavailable_state_text(ui_copy("overlay_archive_empty_disassembly")),
                entries: Vec::new(),
                selected_inputs: Vec::new(),
            };
        }

        let selected_index = self.archive_selected_index(recipes.len());
        let entries = recipes
            .iter()
            .take(6)
            .enumerate()
            .map(|(index, recipe)| ArchiveDisassemblyRecipeEntry {
                title: data.item_name(&recipe.output_item_id).to_owned(),
                detail: recipe.name.clone(),
                meta: ui_format(
                    "overlay_archive_owned",
                    &[(
                        "count",
                        &self
                            .inventory
                            .get(&recipe.output_item_id)
                            .copied()
                            .unwrap_or_default()
                            .to_string(),
                    )],
                ),
                selected: index == selected_index,
            })
            .collect();

        let selected_inputs = recipes[selected_index]
            .ingredients
            .iter()
            .map(|ingredient| {
                ui_format(
                    "overlay_input_amount",
                    &[
                        ("item", data.item_name(&ingredient.item_id)),
                        ("amount", &ingredient.amount.to_string()),
                    ],
                )
            })
            .collect();

        ArchiveDisassemblySectionView {
            title: ui_copy("overlay_disassembly").to_string(),
            selected_inputs_title: ui_copy("overlay_recovered_inputs").to_string(),
            help_text: disassembly_help_text(),
            empty_text: String::new(),
            entries,
            selected_inputs,
        }
    }
}

fn disassembly_help_text() -> String {
    ui_format(
        "overlay_archive_disassembly_help",
        &[("confirm", &input_bindings().global.confirm)],
    )
}
