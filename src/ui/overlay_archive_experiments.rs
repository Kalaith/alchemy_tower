use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::ui::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_archive_experiments_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text(
            ui_copy("overlay_experiment_history"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let entries = self.archive_experiment_entries(data);
        draw_text(
            &ui_format(
                "overlay_filter",
                &[("mode", self.archive_experiment_filter_label())],
            ),
            x + 220.0,
            y + 122.0,
            20.0,
            dark::TEXT_DIM,
        );
        if entries.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text(ui_copy("overlay_archive_empty_experiments")),
                false,
            );
            return;
        }

        let selected_index = self.ui.archive_index.min(entries.len().saturating_sub(1));
        let page = selected_index / 6;
        let page_count = entries.len().div_ceil(6);
        draw_text(
            &ui_format(
                "overlay_page",
                &[
                    ("page", &(page + 1).to_string()),
                    ("pages", &page_count.to_string()),
                ],
            ),
            x + 320.0,
            y + 122.0,
            20.0,
            dark::TEXT_DIM,
        );
        let mut list_y = y + 154.0;
        let page_start = page * 6;
        for (offset, entry) in entries.iter().skip(page_start).take(6).enumerate() {
            let index = page_start + offset;
            let selected = index == selected_index;
            let title = data.item_name(&entry.output_item_id);
            let detail = if entry.recipe_id.is_empty() {
                ui_copy("overlay_archive_unknown_recipe").to_owned()
            } else {
                data.recipes
                    .iter()
                    .find(|recipe| recipe.id == entry.recipe_id)
                    .map(|recipe| recipe.name.clone())
                    .unwrap_or_else(|| entry.recipe_id.clone())
            };
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                selected,
                true,
                title,
                &detail,
                &ui_format(
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
            );
            list_y += 64.0;
        }

        let selected_entry = entries[selected_index];
        draw_text(
            ui_copy("overlay_selected_record"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &ui_format(
                "overlay_output",
                &[("item", data.item_name(&selected_entry.output_item_id))],
            ),
            x + 410.0,
            y + 156.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &ui_format(
                "overlay_archive_quality",
                &[
                    ("quality", &selected_entry.quality_score.to_string()),
                    ("band", &selected_entry.quality_band),
                ],
            ),
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &ui_format(
                "overlay_archive_result",
                &[(
                    "result",
                    ui_copy(if selected_entry.stable {
                        "overlay_archive_result_stable"
                    } else {
                        "overlay_archive_result_unstable"
                    }),
                )],
            ),
            x + 410.0,
            y + 208.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &ui_format(
                "overlay_archive_catalyst",
                &[(
                    "item",
                    if selected_entry.catalyst_item_id.is_empty() {
                        ui_copy("overlay_archive_none")
                    } else {
                        data.item_name(&selected_entry.catalyst_item_id)
                    },
                )],
            ),
            x + 410.0,
            y + 232.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &ui_format(
                "overlay_archive_morph",
                &[(
                    "item",
                    if selected_entry.morph_output_item_id.is_empty() {
                        ui_copy("overlay_archive_none")
                    } else {
                        data.item_name(&selected_entry.morph_output_item_id)
                    },
                )],
            ),
            x + 410.0,
            y + 256.0,
            20.0,
            dark::TEXT_DIM,
        );
        if let Some(recipe) = data
            .recipes
            .iter()
            .find(|recipe| recipe.id == selected_entry.recipe_id)
        {
            draw_text(
                &ui_format(
                    "overlay_archive_mastery_now",
                    &[(
                        "stage",
                        crate::alchemy::mastery_stage(self.recipe_mastery_brews(&recipe.id)),
                    )],
                ),
                x + 410.0,
                y + 282.0,
                20.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &ui_format(
                    "overlay_archive_memory",
                    &[("text", &self.recipe_memory_meta(data, recipe))],
                ),
                x + 410.0,
                y + 306.0,
                18.0,
                dark::TEXT_DIM,
            );
            draw_wrapped_text(
                &self.recipe_memory_detail(data, recipe),
                x + 410.0,
                y + 332.0,
                w - 430.0,
                18.0,
                20.0,
                dark::TEXT_DIM,
            );
        }
    }

}
