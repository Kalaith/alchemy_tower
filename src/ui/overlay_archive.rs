use super::{
    archive_tab_label, draw_overlay_section_box, draw_overlay_tab, GameplayState, ARCHIVE_TABS,
};
use crate::content::{ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::ui::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_selection_card, draw_state_banner, draw_wrapped_text,
};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_archive_overlay(&self, data: &GameData) {
        draw_overlay_backdrop();
        let x = 150.0;
        let y = 70.0;
        let w = screen_width() - 300.0;
        let h = screen_height() - 140.0;
        draw_panel(x, y, w, h, ui_copy("overlay_archive_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.archive_subtitle);
        self.draw_archive_tabs(x, y, w);

        match ARCHIVE_TABS[self.ui.archive_tab] {
            "timeline" => self.draw_archive_timeline_section(x, y, w, h),
            "experiments" => self.draw_archive_experiments_section(data, x, y, w, h),
            "mastery" => self.draw_archive_mastery_section(data, x, y, w, h),
            "morphs" => self.draw_archive_morphs_section(data, x, y, w, h),
            "disassembly" => self.draw_archive_disassembly_section(data, x, y, w, h),
            _ => self.draw_archive_duplication_section(data, x, y, w, h),
        }
        draw_overlay_footer(x, y, w, h, &self.archive_footer_text());
    }

    fn draw_archive_tabs(&self, x: f32, y: f32, w: f32) {
        for (index, tab) in ARCHIVE_TABS.iter().enumerate() {
            let rect = Rect::new(x + 20.0 + index as f32 * 148.0, y + 54.0, 136.0, 30.0);
            let selected = index == self.ui.archive_tab;
            draw_overlay_tab(rect, archive_tab_label(tab), selected);
            if rect.x + rect.w > x + w - 20.0 {
                break;
            }
        }
    }

    fn draw_archive_timeline_section(&self, x: f32, y: f32, w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_archive_section_timeline"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut row_y = y + 152.0;
        for milestone in self.progression.journal_milestones.iter().rev().take(7) {
            draw_text(&milestone.title, x + 20.0, row_y, 20.0, dark::TEXT_BRIGHT);
            row_y += 20.0;
            draw_wrapped_text(
                &milestone.text,
                x + 20.0,
                row_y,
                430.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            row_y += 40.0;
        }

        draw_text(
            ui_copy("overlay_tower_status"),
            x + 500.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let status_lines = [
            ui_format(
                "overlay_brews_completed",
                &[("count", &self.progression.total_brews.to_string())],
            ),
            ui_format(
                "overlay_known_recipes",
                &[("count", &self.progression.known_recipes.len().to_string())],
            ),
            ui_format(
                "overlay_recorded_experiments",
                &[("count", &self.progression.experiment_log.len().to_string())],
            ),
            ui_format(
                "overlay_unlocked_routes",
                &[("count", &self.progression.unlocked_warps.len().to_string())],
            ),
        ];
        let mut status_y = y + 156.0;
        for line in status_lines {
            draw_text(&line, x + 500.0, status_y, 20.0, dark::TEXT_DIM);
            status_y += 24.0;
        }
        let reconstruction = if self.can_reconstruct_archive() {
            narrative_text()
                .statuses
                .archive_reconstruction_ready
                .clone()
        } else {
            self.locked_state_text(&narrative_text().statuses.archive_reconstruction_missing)
        };
        draw_state_banner(
            x + 500.0,
            y + h - 120.0,
            w - 520.0,
            &reconstruction,
            !self.can_reconstruct_archive(),
        );
    }

    fn draw_archive_mastery_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text(
            ui_copy("overlay_recipe_mastery"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let recipes = self.mastery_recipes(data);
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text(ui_copy("overlay_archive_empty_mastery")),
                false,
            );
            return;
        }

        let selected_index = self.ui.archive_index.min(recipes.len().saturating_sub(1));
        let mut list_y = y + 154.0;
        for (index, recipe) in recipes.iter().take(6).enumerate() {
            let mastery = self.recipe_mastery_brews(&recipe.id);
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                index == selected_index,
                true,
                &recipe.name,
                &recipe.description,
                &format!(
                    "{}  {}",
                    crate::alchemy::mastery_stage(mastery),
                    self.recipe_memory_meta(data, recipe)
                ),
            );
            list_y += 64.0;
        }

        let recipe = recipes[selected_index];
        let mastery = self.recipe_mastery_brews(&recipe.id);
        draw_text(
            ui_copy("overlay_mastery_detail"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(&recipe.name, x + 410.0, y + 156.0, 24.0, dark::TEXT_BRIGHT);
        draw_text(
            &ui_format(
                "overlay_archive_mastery_stage",
                &[
                    ("stage", crate::alchemy::mastery_stage(mastery)),
                    ("count", &mastery.to_string()),
                ],
            ),
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        if let Some(profile) = self
            .progression
            .crafted_item_profiles
            .get(&recipe.output_item_id)
        {
            draw_text(
                &ui_format(
                    "overlay_archive_best_result",
                    &[
                        ("quality", &profile.best_quality_score.to_string()),
                        ("band", &profile.best_quality_band),
                    ],
                ),
                x + 410.0,
                y + 210.0,
                20.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &ui_format(
                    "overlay_archive_traits_carried",
                    &[(
                        "traits",
                        &if profile.inherited_traits.is_empty() {
                            ui_copy("overlay_archive_none").to_owned()
                        } else {
                            profile.inherited_traits.join(", ")
                        },
                    )],
                ),
                x + 410.0,
                y + 236.0,
                20.0,
                dark::TEXT_DIM,
            );
        }
        if let Some(entry) = self
            .progression
            .experiment_log
            .iter()
            .rev()
            .find(|entry| entry.recipe_id == recipe.id)
        {
            draw_text(
                &ui_format(
                    "overlay_archive_last_attempt",
                    &[
                        ("day", &(entry.day_index + 1).to_string()),
                        ("band", &entry.quality_band),
                    ],
                ),
                x + 410.0,
                y + 262.0,
                20.0,
                dark::TEXT_DIM,
            );
        }
        draw_wrapped_text(
            &recipe.lore_note,
            x + 410.0,
            y + 292.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
    }

    fn draw_archive_disassembly_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text(
            ui_copy("overlay_disassembly"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let recipes = self.available_disassembly_recipes(data);
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text(ui_copy("overlay_archive_empty_disassembly")),
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
                data.item_name(&recipe.output_item_id),
                &recipe.name,
                &ui_format(
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
            );
            list_y += 64.0;
        }

        let recipe = recipes[selected_index];
        draw_text(
            ui_copy("overlay_recovered_inputs"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut detail_y = y + 156.0;
        for ingredient in &recipe.ingredients {
            draw_text(
                &ui_format(
                    "overlay_input_amount",
                    &[
                        ("item", data.item_name(&ingredient.item_id)),
                        ("amount", &ingredient.amount.to_string()),
                    ],
                ),
                x + 410.0,
                detail_y,
                22.0,
                dark::TEXT_DIM,
            );
            detail_y += 24.0;
        }
        draw_wrapped_text(
            ui_copy("overlay_archive_disassembly_help"),
            x + 410.0,
            detail_y + 18.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
    }

    fn draw_archive_duplication_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text(
            ui_copy("overlay_duplication"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let items = self.duplication_candidates(data);
        if items.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text(ui_copy("overlay_archive_empty_duplication")),
                false,
            );
            return;
        }

        let selected_index = self.ui.archive_index.min(items.len().saturating_sub(1));
        let mut list_y = y + 154.0;
        for (index, item_id) in items.iter().take(6).enumerate() {
            let item = data
                .item(item_id)
                .expect("duplication candidate should exist");
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                index == selected_index,
                self.can_duplicate_item(data, item_id),
                data.item_name(item_id),
                &self.inventory_reference_summary(data, item_id),
                &ui_format(
                    "overlay_archive_owned_cost",
                    &[
                        (
                            "count",
                            &self
                                .inventory
                                .get(item_id)
                                .copied()
                                .unwrap_or_default()
                                .to_string(),
                        ),
                        (
                            "cost",
                            &(item.base_value + u32::from(item.rarity) * 10).to_string(),
                        ),
                    ],
                ),
            );
            list_y += 64.0;
        }

        let item_id = &items[selected_index];
        let item = data
            .item(item_id)
            .expect("duplication candidate should exist");
        draw_text(
            ui_copy("overlay_duplication_cost"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &ui_format("overlay_target", &[("item", &item.name)]),
            x + 410.0,
            y + 156.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &ui_format(
                "overlay_coins",
                &[(
                    "count",
                    &(item.base_value + u32::from(item.rarity) * 10).to_string(),
                )],
            ),
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &ui_format(
                "overlay_archive_duplication_catalyst",
                &[(
                    "item",
                    self.duplication_catalyst_item_id(data)
                        .as_deref()
                        .map(|id| data.item_name(id))
                        .unwrap_or(ui_copy("overlay_archive_duplication_catalyst_required")),
                )],
            ),
            x + 410.0,
            y + 210.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            ui_copy("overlay_archive_duplication_help"),
            x + 410.0,
            y + 244.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
    }

    fn archive_footer_text(&self) -> String {
        match ARCHIVE_TABS[self.ui.archive_tab] {
            "timeline" => ui_copy("overlay_archive_footer_timeline").to_owned(),
            "disassembly" | "duplication" => ui_copy("overlay_archive_footer_confirm").to_owned(),
            "experiments" => ui_copy("overlay_archive_footer_filter").to_owned(),
            "mastery" | "morphs" => ui_copy("overlay_archive_footer_browse").to_owned(),
            _ => ui_copy("overlay_archive_footer_close").to_owned(),
        }
    }

}
