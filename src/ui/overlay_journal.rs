use super::GameplayState;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, StationKind};
use crate::ui::{draw_panel, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_field_journal(&self, data: &GameData, art: &ArtAssets) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 150),
        );
        let x = 120.0;
        let y = 72.0;
        let w = screen_width() - 240.0;
        let h = screen_height() - 144.0;
        draw_panel(x, y, w, h, ui_copy("overlay_journal_title"));
        let close_rect = self.journal_close_rect();
        let close_hovered = close_rect.contains(mouse_position().into());
        draw_rectangle(
            close_rect.x,
            close_rect.y,
            close_rect.w,
            close_rect.h,
            if close_hovered {
                dark::ACCENT
            } else {
                Color::from_rgba(38, 40, 50, 255)
            },
        );
        draw_rectangle_lines(
            close_rect.x,
            close_rect.y,
            close_rect.w,
            close_rect.h,
            2.0,
            if close_hovered { WHITE } else { dark::ACCENT },
        );
        draw_text(
            ui_copy("overlay_close"),
            close_rect.x + 18.0,
            close_rect.y + 19.0,
            18.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &ui_format(
                "overlay_current_conditions",
                &[
                    ("season", self.current_season()),
                    ("weather", self.current_weather()),
                ],
            ),
            x + 20.0,
            y + 50.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        let tabs = self.journal_tabs();
        for (index, tab) in tabs.iter().enumerate() {
            let selected = index == self.ui.journal_tab;
            let rect = self.journal_tab_rect(index, tabs.len());
            let hovered = rect.contains(mouse_position().into());
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if selected || hovered {
                    dark::ACCENT
                } else {
                    Color::from_rgba(38, 40, 50, 255)
                },
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                2.0,
                if selected || hovered {
                    WHITE
                } else {
                    dark::ACCENT
                },
            );
            draw_text(
                tab,
                rect.x + 34.0,
                rect.y + 20.0,
                18.0,
                if selected {
                    dark::TEXT_BRIGHT
                } else {
                    dark::TEXT_DIM
                },
            );
            if let Some(texture) = art.journal_tab_by_label(tab) {
                draw_texture_centered(
                    texture,
                    vec2(rect.x + 18.0, rect.y + 14.0),
                    vec2(18.0, 18.0),
                    WHITE,
                );
            }
        }

        let greenhouse_unlocked = self
            .progression
            .completed_quests
            .contains("entry_to_greenhouse");
        match self.ui.journal_tab {
            0 => self.draw_journal_routes_tab(data, x, y, w, h),
            1 => self.draw_journal_notes_tab(data, x, y, w, h),
            2 => self.draw_journal_brews_tab(data, x, y, w, h),
            3 if greenhouse_unlocked => self.draw_journal_greenhouse_tab(data, x, y, w, h),
            _ => self.draw_journal_rapport_tab(data, x, y, w, h),
        }
        draw_text(
            ui_copy("overlay_journal_footer"),
            x + 20.0,
            y + h - 20.0,
            18.0,
            dark::TEXT_DIM,
        );
    }

    fn draw_journal_notes_tab(&self, data: &GameData, x: f32, y: f32, w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_tower_notes"),
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            ui_copy("overlay_progress_active"),
            x + 20.0,
            y + 168.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_rectangle(
            x + 20.0,
            y + 182.0,
            w - 40.0,
            72.0,
            Color::from_rgba(38, 40, 50, 255),
        );
        draw_rectangle_lines(x + 20.0, y + 182.0, w - 40.0, 72.0, 2.0, dark::ACCENT);
        let active_summary = self
            .active_quest_summary(data)
            .unwrap_or_else(|| self.next_goal_summary(data));
        draw_wrapped_text(
            &active_summary,
            x + 34.0,
            y + 206.0,
            w - 68.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );

        draw_text(
            ui_copy("overlay_progress_milestones"),
            x + 20.0,
            y + 286.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        let mut milestone_y = y + 318.0;
        for (label, detail, ready) in self.milestone_status_lines() {
            draw_text(
                &format!(
                    "{} [{}]",
                    label,
                    if ready {
                        ui_copy("overlay_progress_ready")
                    } else {
                        ui_copy("overlay_progress_locked")
                    }
                ),
                x + 20.0,
                milestone_y,
                20.0,
                dark::TEXT_BRIGHT,
            );
            milestone_y += 20.0;
            draw_wrapped_text(
                &detail,
                x + 20.0,
                milestone_y,
                w - 40.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            milestone_y += 34.0;
        }

        let mut note_y = y + 448.0;
        for milestone in self.progression.journal_milestones.iter().rev().take(5) {
            draw_text(&milestone.title, x + 20.0, note_y, 22.0, dark::TEXT_BRIGHT);
            note_y += 22.0;
            draw_wrapped_text(
                &milestone.text,
                x + 20.0,
                note_y,
                w - 40.0,
                18.0,
                20.0,
                dark::TEXT_DIM,
            );
            note_y += 52.0;
            if note_y > y + h - 40.0 {
                break;
            }
        }
    }

    fn draw_journal_greenhouse_tab(&self, data: &GameData, x: f32, y: f32, _w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_greenhouse_beds"),
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut bed_y = y + 168.0;
        let mut any_planter = false;
        for station in self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.kind == StationKind::Planter)
        {
            any_planter = true;
            let summary = self
                .progression
                .planter_states
                .get(&station.id)
                .map(|state| {
                    if state.planted_item_id.is_empty() {
                        ui_copy("overlay_greenhouse_none").to_owned()
                    } else if state.ready {
                        if state.mutation_note.is_empty() {
                            ui_format(
                                "overlay_planter_ready",
                                &[("item", data.item_name(&state.planted_item_id))],
                            )
                        } else {
                            ui_format(
                                "overlay_greenhouse_ready_meta",
                                &[
                                    ("item", data.item_name(&state.planted_item_id)),
                                    ("mutation", &state.mutation_note),
                                ],
                            )
                        }
                    } else {
                        let growth_target = station
                            .planter_harvest_days
                            .max(1)
                            .saturating_sub(state.mutation_growth_bonus_days)
                            .max(1);
                        if state.mutation_note.is_empty() {
                            format!(
                                "{} ({})",
                                data.item_name(&state.planted_item_id),
                                planter_stage_label(state.growth_days, growth_target)
                            )
                        } else {
                            format!(
                                "{} ({}, {})",
                                data.item_name(&state.planted_item_id),
                                planter_stage_label(state.growth_days, growth_target),
                                state.mutation_note
                            )
                        }
                    }
                })
                .unwrap_or_else(|| ui_copy("overlay_greenhouse_none").to_owned());
            draw_text(&station.name, x + 20.0, bed_y, 22.0, dark::TEXT_BRIGHT);
            bed_y += 22.0;
            draw_text(&summary, x + 20.0, bed_y, 18.0, dark::TEXT_DIM);
            bed_y += 30.0;
            if bed_y > y + h - 40.0 {
                break;
            }
        }
        if !any_planter {
            draw_text(
                ui_copy("overlay_greenhouse_empty"),
                x + 20.0,
                bed_y,
                20.0,
                dark::TEXT_DIM,
            );
        }
    }

    fn draw_journal_rapport_tab(&self, data: &GameData, x: f32, y: f32, w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_town_rapport"),
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut rapport_y = y + 168.0;
        for npc in &data.npcs {
            let rapport = self
                .progression
                .relationships
                .get(&npc.id)
                .copied()
                .unwrap_or_default();
            draw_text(
                &ui_format(
                    "overlay_rapport_line",
                    &[
                        ("name", &npc.name),
                        (
                            "role",
                            if npc.role.is_empty() {
                                ui_copy("overlay_rapport_empty")
                            } else {
                                npc.role.as_str()
                            },
                        ),
                        ("value", &rapport.to_string()),
                    ],
                ),
                x + 20.0,
                rapport_y,
                20.0,
                dark::TEXT_BRIGHT,
            );
            rapport_y += 20.0;
            draw_text(
                &ui_format("overlay_now", &[("text", &self.npc_now_hint(data, npc))]),
                x + 20.0,
                rapport_y,
                17.0,
                dark::TEXT_DIM,
            );
            rapport_y += 18.0;
            draw_text(
                &ui_format(
                    "overlay_later",
                    &[("text", &self.npc_later_hint(data, npc))],
                ),
                x + 20.0,
                rapport_y,
                17.0,
                dark::TEXT_DIM,
            );
            rapport_y += 18.0;
            draw_wrapped_text(
                &ui_format(
                    "overlay_usually",
                    &[("text", &self.npc_usual_hint(data, npc))],
                ),
                x + 20.0,
                rapport_y,
                w - 40.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            rapport_y += 34.0;
            if rapport_y > y + h - 40.0 {
                break;
            }
        }
    }

}
