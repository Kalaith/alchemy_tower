use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::ui::draw_wrapped_text;
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, draw_text, Color};
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_journal_routes_tab(&self, data: &GameData, x: f32, y: f32, w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_known_routes"),
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut route_y = y + 168.0;
        for route in &data.gathering_routes {
            draw_text(&route.name, x + 20.0, route_y, 22.0, dark::TEXT_BRIGHT);
            route_y += 22.0;
            draw_text(&route.description, x + 20.0, route_y, 18.0, dark::TEXT_DIM);
            route_y += 28.0;
            if route_y > y + h - 40.0 {
                break;
            }
        }

        draw_text(
            ui_copy("overlay_herb_memories"),
            x + 420.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut entry_y = y + 168.0;
        let herb_memories = self.herb_memories(data);
        if herb_memories.is_empty() {
            draw_text(
                ui_copy("journal_memory_no_herbs"),
                x + 420.0,
                entry_y,
                22.0,
                dark::TEXT_DIM,
            );
        } else {
            for entry in herb_memories {
                draw_text(
                    data.item_name(&entry.item_id),
                    x + 420.0,
                    entry_y,
                    22.0,
                    dark::TEXT_BRIGHT,
                );
                entry_y += 22.0;
                draw_text(
                    &ui_format(
                        "journal_memory_state_line",
                        &[("state", ui_copy(self.herb_memory_state_key(&entry.item_id)))],
                    ),
                    x + 420.0,
                    entry_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                entry_y += 20.0;
                let route_label = if entry.learned {
                    data.route(&entry.learned_route_id)
                        .map(|route| route.name.as_str())
                        .unwrap_or_else(|| ui_copy("journal_memory_unknown_place"))
                } else {
                    data.route(&entry.first_seen_route_id)
                        .map(|route| route.name.as_str())
                        .unwrap_or_else(|| ui_copy("journal_memory_unknown_place"))
                };
                let route_copy_key = if entry.learned {
                    "journal_memory_learned_at"
                } else {
                    "journal_memory_observed_at"
                };
                draw_text(
                    &ui_format(route_copy_key, &[("route", route_label)]),
                    x + 420.0,
                    entry_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                entry_y += 20.0;
                draw_wrapped_text(
                    &self.journal_herb_summary(data, &entry.item_id),
                    x + 420.0,
                    entry_y,
                    w - 440.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                entry_y += 40.0;
                let conditions = if entry.learned {
                    self.learned_gathering_conditions(data, &entry.item_id)
                        .unwrap_or_else(|| ui_copy("journal_memory_conditions_unknown").to_owned())
                } else {
                    ui_copy("journal_memory_conditions_unknown").to_owned()
                };
                draw_wrapped_text(
                    &conditions,
                    x + 420.0,
                    entry_y,
                    w - 440.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                entry_y += 28.0;
                if entry.best_quality > 0 {
                    draw_text(
                        &ui_format(
                            "journal_memory_best_specimen",
                            &[
                                ("quality", &entry.best_quality.to_string()),
                                ("band", &entry.best_quality_band),
                            ],
                        ),
                        x + 420.0,
                        entry_y,
                        18.0,
                        dark::TEXT_DIM,
                    );
                    entry_y += 20.0;
                }
                if !entry.variant_name.is_empty() {
                    draw_text(
                        &ui_format(
                            "journal_memory_variant",
                            &[("variant", &entry.variant_name)],
                        ),
                        x + 420.0,
                        entry_y,
                        18.0,
                        dark::TEXT_DIM,
                    );
                    entry_y += 20.0;
                }
                if entry.learned && !entry.note.is_empty() {
                    draw_wrapped_text(
                        &entry.note,
                        x + 420.0,
                        entry_y,
                        w - 440.0,
                        16.0,
                        18.0,
                        dark::TEXT_DIM,
                    );
                    entry_y += 30.0;
                }
                if entry_y > y + h - 170.0 {
                    break;
                }
            }
        }
        draw_text(
            ui_copy("overlay_progress_routes"),
            x + 20.0,
            y + h - 156.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_rectangle(
            x + 20.0,
            y + h - 140.0,
            w - 40.0,
            96.0,
            Color::from_rgba(38, 40, 50, 255),
        );
        draw_rectangle_lines(x + 20.0, y + h - 140.0, w - 40.0, 96.0, 2.0, dark::ACCENT);
        let locked_warps = self.locked_warps(data);
        if locked_warps.is_empty() {
            draw_text(
                ui_copy("overlay_routes_all_restored"),
                x + 34.0,
                y + h - 108.0,
                20.0,
                dark::TEXT_DIM,
            );
        } else {
            let mut unlock_y = y + h - 108.0;
            for warp in locked_warps.into_iter().take(2) {
                draw_wrapped_text(
                    &format!("{}: {}", warp.label, self.warp_lock_text(data, warp)),
                    x + 34.0,
                    unlock_y,
                    w - 68.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                unlock_y += 34.0;
            }
        }
    }

}
