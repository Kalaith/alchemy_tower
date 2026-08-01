use super::gameplay_overlay_window::{paged_window, ARCHIVE_PAGE_ROWS};
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::archive::{
    ArchiveMorphDetailView, ArchiveMorphRecipeEntry, ArchiveMorphTargetView,
    ArchiveMorphsSectionView,
};

impl GameplayState {
    pub(super) fn archive_morphs_section_view(&self, data: &GameData) -> ArchiveMorphsSectionView {
        let recipes = self.morph_recipes(data);
        if recipes.is_empty() {
            return ArchiveMorphsSectionView {
                title: ui_copy("overlay_morph_previews").to_string(),
                detail_title: ui_copy("overlay_branch_detail").to_string(),
                page_text: None,
                empty_text: self.unavailable_state_text(ui_copy("overlay_archive_empty_morphs")),
                entries: Vec::new(),
                detail: None,
            };
        }

        let selected_index = self.archive_selected_index(recipes.len());
        let (page_start, page_text) =
            paged_window(selected_index, recipes.len(), ARCHIVE_PAGE_ROWS);
        let entries = recipes
            .iter()
            .skip(page_start)
            .take(6)
            .enumerate()
            .map(|(index, recipe)| ArchiveMorphRecipeEntry {
                title: recipe.name.clone(),
                detail: recipe.description.clone(),
                meta: ui_format(
                    "overlay_archive_branches",
                    &[("count", &recipe.morph_targets.len().to_string())],
                ),
                selected: page_start + index == selected_index,
            })
            .collect();

        let recipe = recipes[selected_index];
        let last_morph_text = self.last_morph_output(&recipe.id).map(|entry| {
            ui_format(
                "overlay_archive_last_morph",
                &[
                    ("day", &(entry.day_index + 1).to_string()),
                    ("item", data.item_name(&entry.morph_output_item_id)),
                ],
            )
        });
        let targets = recipe
            .morph_targets
            .iter()
            .map(|morph| ArchiveMorphTargetView {
                title: ui_format(
                    "overlay_archive_morph_target",
                    &[
                        ("item", data.item_name(&morph.output_item_id)),
                        (
                            "suffix",
                            if self.morph_output_discovered(&morph.output_item_id) {
                                ui_copy("overlay_archive_logged_suffix")
                            } else {
                                ""
                            },
                        ),
                    ],
                ),
                conditions: archive_morph_conditions(morph),
            })
            .collect();

        ArchiveMorphsSectionView {
            title: ui_copy("overlay_morph_previews").to_string(),
            detail_title: ui_copy("overlay_branch_detail").to_string(),
            page_text,
            empty_text: String::new(),
            entries,
            detail: Some(ArchiveMorphDetailView {
                last_morph_text,
                targets,
            }),
        }
    }
}

fn archive_morph_conditions(morph: &crate::data::MorphDefinition) -> String {
    [
        ui_format(
            "overlay_condition_quality",
            &[("value", &morph.minimum_quality.to_string())],
        ),
        ui_format(
            "overlay_condition_heat",
            &[("value", &morph.required_heat.to_string())],
        ),
        ui_format(
            "overlay_condition_stirs",
            &[("value", &morph.required_stirs.to_string())],
        ),
        if morph.catalyst_tag.is_empty() {
            ui_format(
                "overlay_condition_catalyst",
                &[("value", ui_copy("overlay_any"))],
            )
        } else {
            ui_format(
                "overlay_condition_catalyst",
                &[("value", &morph.catalyst_tag)],
            )
        },
        if morph.required_timing.is_empty() {
            ui_format(
                "overlay_condition_timing",
                &[("value", ui_copy("overlay_any"))],
            )
        } else {
            ui_format(
                "overlay_condition_timing",
                &[("value", &morph.required_timing)],
            )
        },
    ]
    .join("  |  ")
}
