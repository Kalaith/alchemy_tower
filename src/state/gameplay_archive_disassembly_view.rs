use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::archive::{
    ArchiveDisassemblyRecipeEntry, ArchiveDisassemblySectionView,
};

impl GameplayState {
    pub(super) fn archive_disassembly_section_view(
        &self,
        data: &GameData,
    ) -> ArchiveDisassemblySectionView {
        let recipes = self.available_disassembly_recipes(data);
        if recipes.is_empty() {
            return ArchiveDisassemblySectionView {
                empty_text: self.unavailable_state_text(
                    ui_copy("overlay_archive_empty_disassembly"),
                ),
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
            empty_text: String::new(),
            entries,
            selected_inputs,
        }
    }
}
