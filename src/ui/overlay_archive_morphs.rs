use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::ui::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_archive_morphs_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text(
            ui_copy("overlay_morph_previews"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let recipes = self.morph_recipes(data);
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text(ui_copy("overlay_archive_empty_morphs")),
                false,
            );
            return;
        }

        let selected_index = self.ui.archive_index.min(recipes.len().saturating_sub(1));
        let mut list_y = y + 154.0;
        for (index, recipe) in recipes.iter().take(6).enumerate() {
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                index == selected_index,
                true,
                &recipe.name,
                &recipe.description,
                &ui_format(
                    "overlay_archive_branches",
                    &[("count", &recipe.morph_targets.len().to_string())],
                ),
            );
            list_y += 64.0;
        }

        let recipe = recipes[selected_index];
        draw_text(
            ui_copy("overlay_branch_detail"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        if let Some(entry) =
            self.progression.experiment_log.iter().rev().find(|entry| {
                entry.recipe_id == recipe.id && !entry.morph_output_item_id.is_empty()
            })
        {
            draw_text(
                &ui_format(
                    "overlay_archive_last_morph",
                    &[
                        ("day", &(entry.day_index + 1).to_string()),
                        ("item", data.item_name(&entry.morph_output_item_id)),
                    ],
                ),
                x + 410.0,
                y + 146.0,
                20.0,
                dark::TEXT_DIM,
            );
        }
        let mut detail_y = y + 176.0;
        for morph in &recipe.morph_targets {
            let discovered = self
                .progression
                .crafted_item_profiles
                .contains_key(&morph.output_item_id);
            draw_text(
                &format!(
                    "{}{}",
                    data.item_name(&morph.output_item_id),
                    if discovered {
                        ui_copy("overlay_archive_logged_suffix")
                    } else {
                        ""
                    }
                ),
                x + 410.0,
                detail_y,
                22.0,
                dark::TEXT_BRIGHT,
            );
            detail_y += 22.0;
            let conditions = [
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
            .join("  |  ");
            draw_wrapped_text(
                &conditions,
                x + 410.0,
                detail_y,
                w - 430.0,
                18.0,
                20.0,
                dark::TEXT_DIM,
            );
            detail_y += 32.0;
        }
    }

}
