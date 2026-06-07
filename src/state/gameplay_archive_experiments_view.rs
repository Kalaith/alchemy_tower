use super::GameplayState;
use crate::alchemy::mastery_stage;
use crate::content::{ui_copy, ui_format};
use crate::data::{ExperimentLogEntry, GameData};
use crate::view_models::archive::{
    ArchiveExperimentEntryView, ArchiveExperimentRecipeMemoryView, ArchiveExperimentRecordView,
    ArchiveExperimentsSectionView,
};

impl GameplayState {
    pub(super) fn archive_experiments_section_view(
        &self,
        data: &GameData,
    ) -> ArchiveExperimentsSectionView {
        let entries = self.archive_experiment_entries(data);
        let filter_text = ui_format(
            "overlay_filter",
            &[("mode", self.archive_experiment_filter_label())],
        );

        if entries.is_empty() {
            return ArchiveExperimentsSectionView {
                title: ui_copy("overlay_experiment_history").to_owned(),
                filter_text,
                page_text: None,
                empty_text: self
                    .unavailable_state_text(ui_copy("overlay_archive_empty_experiments")),
                entries: Vec::new(),
                selected_record: None,
            };
        }

        let selected_index = self.archive_selected_index(entries.len());
        let page = selected_index / 6;
        let page_count = entries.len().div_ceil(6);
        let page_start = page * 6;
        let row_entries = entries
            .iter()
            .skip(page_start)
            .take(6)
            .enumerate()
            .map(|(offset, entry)| {
                let index = page_start + offset;
                ArchiveExperimentEntryView {
                    title: data.item_name(&entry.output_item_id).to_owned(),
                    detail: experiment_recipe_name(data, entry),
                    meta: ui_format(
                        "overlay_archive_entry_meta",
                        &[
                            ("day", &(entry.day_index + 1).to_string()),
                            ("band", &entry.quality_band),
                            (
                                "state",
                                ui_copy(if entry.stable {
                                    "overlay_archive_state_stable"
                                } else {
                                    "overlay_archive_state_unstable"
                                }),
                            ),
                        ],
                    ),
                    selected: index == selected_index,
                }
            })
            .collect();

        ArchiveExperimentsSectionView {
            title: ui_copy("overlay_experiment_history").to_owned(),
            filter_text,
            page_text: Some(ui_format(
                "overlay_page",
                &[
                    ("page", &(page + 1).to_string()),
                    ("pages", &page_count.to_string()),
                ],
            )),
            empty_text: String::new(),
            entries: row_entries,
            selected_record: Some(
                self.archive_experiment_record_view(data, entries[selected_index]),
            ),
        }
    }

    fn archive_experiment_record_view(
        &self,
        data: &GameData,
        entry: &ExperimentLogEntry,
    ) -> ArchiveExperimentRecordView {
        ArchiveExperimentRecordView {
            title: ui_copy("overlay_selected_record").to_owned(),
            output_text: ui_format(
                "overlay_output",
                &[("item", data.item_name(&entry.output_item_id))],
            ),
            quality_text: ui_format(
                "overlay_archive_quality",
                &[
                    ("quality", &entry.quality_score.to_string()),
                    ("band", &entry.quality_band),
                ],
            ),
            result_text: ui_format(
                "overlay_archive_result",
                &[(
                    "result",
                    ui_copy(if entry.stable {
                        "overlay_archive_result_stable"
                    } else {
                        "overlay_archive_result_unstable"
                    }),
                )],
            ),
            catalyst_text: ui_format(
                "overlay_archive_catalyst",
                &[("item", &archive_item_name(data, &entry.catalyst_item_id))],
            ),
            morph_text: ui_format(
                "overlay_archive_morph",
                &[(
                    "item",
                    &archive_item_name(data, &entry.morph_output_item_id),
                )],
            ),
            recipe_memory: self.archive_experiment_recipe_memory_view(data, entry),
        }
    }

    fn archive_experiment_recipe_memory_view(
        &self,
        data: &GameData,
        entry: &ExperimentLogEntry,
    ) -> Option<ArchiveExperimentRecipeMemoryView> {
        let recipe = data
            .recipes
            .iter()
            .find(|recipe| recipe.id == entry.recipe_id)?;
        Some(ArchiveExperimentRecipeMemoryView {
            mastery_text: ui_format(
                "overlay_archive_mastery_now",
                &[(
                    "stage",
                    mastery_stage(self.recipe_mastery_brews(&recipe.id)),
                )],
            ),
            memory_text: ui_format(
                "overlay_archive_memory",
                &[("text", &self.recipe_memory_meta(data, recipe))],
            ),
            detail_text: self.recipe_memory_detail(data, recipe),
        })
    }
}

fn experiment_recipe_name(data: &GameData, entry: &ExperimentLogEntry) -> String {
    if entry.recipe_id.is_empty() {
        ui_copy("overlay_archive_unknown_recipe").to_owned()
    } else {
        data.recipes
            .iter()
            .find(|recipe| recipe.id == entry.recipe_id)
            .map(|recipe| recipe.name.clone())
            .unwrap_or_else(|| entry.recipe_id.clone())
    }
}

fn archive_item_name(data: &GameData, item_id: &str) -> String {
    if item_id.is_empty() {
        ui_copy("overlay_archive_none").to_owned()
    } else {
        data.item_name(item_id).to_owned()
    }
}
