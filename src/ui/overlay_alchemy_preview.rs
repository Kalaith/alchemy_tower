use super::{draw_overlay_section_box, draw_overlay_section_title, GameplayState};
use crate::alchemy::resolve_brew;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::ui::draw_wrapped_text;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_alchemy_preview_panel(&self, data: &GameData, x: f32, y: f32, w: f32) {
        draw_overlay_section_title(x + 340.0, y + 240.0, ui_copy("overlay_preview"), None);
        draw_overlay_section_box(x + 340.0, y + 256.0, w - 360.0, 210.0);
        let selected = self.selected_items();
        if selected.is_empty() {
            draw_text(
                ui_copy("overlay_preview_empty"),
                x + 360.0,
                y + 296.0,
                22.0,
                dark::TEXT_DIM,
            );
        } else if let Some(station) = self.nearby_station(data) {
            let preview = resolve_brew(
                data,
                station,
                &selected,
                self.selected_catalyst(),
                self.alchemy.heat,
                self.alchemy.stirs,
                self.alchemy_timing(),
                self.preview_mastery_brews(data, station, &selected),
            );
            let known = preview
                .recipe
                .map(|recipe| self.progression.known_recipes.contains(&recipe.id))
                .unwrap_or(false);
            let preview_uncertain = known && self.preview_is_uncertain(&preview);
            let stable_preview = self.brew_is_stable(&preview);
            let preview_title = if preview.recipe.is_none() {
                ui_copy("overlay_alchemy_preview_unknown_salvage").to_owned()
            } else if known && stable_preview && !preview_uncertain {
                ui_format(
                    "overlay_known_result",
                    &[("item", data.item_name(&preview.output_item_id))],
                )
            } else if !known {
                ui_copy("overlay_alchemy_preview_unlogged").to_owned()
            } else if preview_uncertain {
                ui_copy("overlay_alchemy_preview_uncertain").to_owned()
            } else if !preview.process_match {
                ui_copy("overlay_alchemy_preview_unstable_process").to_owned()
            } else if !preview.minimum_elements_met {
                ui_copy("overlay_alchemy_preview_element_shortfall").to_owned()
            } else if !preview.minimum_quality_met {
                ui_copy("overlay_alchemy_preview_quality_shortfall").to_owned()
            } else {
                ui_copy("overlay_alchemy_preview_imperfect").to_owned()
            };
            draw_text(
                &preview_title,
                x + 360.0,
                y + 296.0,
                24.0,
                dark::TEXT_BRIGHT,
            );
            draw_text(
                &ui_format(
                    "overlay_alchemy_output",
                    &[
                        ("item", data.item_name(&preview.output_item_id)),
                        ("amount", &preview.output_amount.to_string()),
                    ],
                ),
                x + 360.0,
                y + 326.0,
                22.0,
                dark::TEXT,
            );
            draw_text(
                &ui_format(
                    "overlay_alchemy_quality_forecast",
                    &[
                        ("quality", &preview.quality_score.to_string()),
                        ("band", &preview.quality_band),
                        ("mastery", &preview.mastery_stage),
                    ],
                ),
                x + 360.0,
                y + 348.0,
                18.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &ui_format(
                    "overlay_alchemy_traits",
                    &[(
                        "traits",
                        &if preview.inherited_traits.is_empty() {
                            ui_copy("overlay_alchemy_traits_none").to_owned()
                        } else {
                            preview.inherited_traits.join(", ")
                        },
                    )],
                ),
                x + 360.0,
                y + 370.0,
                18.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &ui_format(
                    "overlay_alchemy_read",
                    &[(
                        "text",
                        ui_copy(if preview.recipe.is_none() {
                            "overlay_alchemy_read_fallback"
                        } else if !known {
                            "overlay_alchemy_read_unlogged"
                        } else if stable_preview {
                            "overlay_alchemy_read_stable"
                        } else {
                            "overlay_alchemy_read_degraded"
                        }),
                    )],
                ),
                x + 360.0,
                y + 392.0,
                18.0,
                dark::TEXT_DIM,
            );
            let mut process_y = y + 416.0;
            if let Some(recipe) = preview.recipe {
                draw_text(
                    &ui_format(
                        "overlay_alchemy_requirements",
                        &[
                            ("heat", &recipe.required_heat.to_string()),
                            ("stirs", &recipe.required_stirs.to_string()),
                            (
                                "timing",
                                if recipe.required_timing.is_empty() {
                                    ui_copy("overlay_any")
                                } else {
                                    recipe.required_timing.as_str()
                                },
                            ),
                            (
                                "process",
                                ui_copy(if preview.process_match {
                                    "overlay_archive_state_stable"
                                } else {
                                    "overlay_archive_state_unstable"
                                }),
                            ),
                            (
                                "quality",
                                ui_copy(if preview.minimum_quality_met {
                                    "overlay_pass"
                                } else {
                                    "overlay_fail"
                                }),
                            ),
                            (
                                "elements",
                                ui_copy(if preview.minimum_elements_met {
                                    "overlay_pass"
                                } else {
                                    "overlay_fail"
                                }),
                            ),
                        ],
                    ),
                    x + 360.0,
                    process_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                process_y += 22.0;
                draw_text(
                    &ui_format(
                        "overlay_alchemy_process_flags",
                        &[
                            (
                                "catalyst",
                                ui_copy(if preview.catalyst_match {
                                    "overlay_match"
                                } else {
                                    "overlay_miss"
                                }),
                            ),
                            (
                                "timing",
                                ui_copy(if preview.timing_match {
                                    "overlay_match"
                                } else {
                                    "overlay_miss"
                                }),
                            ),
                            (
                                "sequence",
                                ui_copy(if preview.sequence_match {
                                    "overlay_match"
                                } else {
                                    "overlay_miss"
                                }),
                            ),
                            (
                                "room",
                                ui_copy(if preview.room_bonus_applied {
                                    "overlay_active"
                                } else {
                                    "overlay_inactive"
                                }),
                            ),
                        ],
                    ),
                    x + 360.0,
                    process_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                process_y += 24.0;
                if !preview.failure_reasons.is_empty() {
                    draw_text(
                        ui_copy("overlay_alchemy_instability_points"),
                        x + 360.0,
                        process_y,
                        18.0,
                        dark::TEXT_BRIGHT,
                    );
                    process_y += 20.0;
                    for reason in preview.failure_reasons.iter().take(3) {
                        draw_text(
                            &format!("- {reason}"),
                            x + 372.0,
                            process_y,
                            18.0,
                            dark::TEXT_DIM,
                        );
                        process_y += 20.0;
                    }
                }
            }
            let detail = preview
                .recipe
                .map(|recipe| {
                    if let Some(morph_output_item_id) = &preview.morph_output_item_id {
                        ui_format(
                            "overlay_alchemy_morph_ready",
                            &[
                                ("recipe", &recipe.name),
                                ("item", data.item_name(morph_output_item_id)),
                            ],
                        )
                    } else if let Some(morph_hint) = &preview.morph_hint {
                        morph_hint.clone()
                    } else if known && preview_uncertain {
                        ui_copy("overlay_alchemy_unknown_catalyst_branch").to_owned()
                    } else if known && stable_preview {
                        recipe.description.clone()
                    } else if !preview.process_match {
                        ui_copy("overlay_alchemy_distort").to_owned()
                    } else if !preview.minimum_elements_met {
                        ui_copy("overlay_alchemy_missing_elements").to_owned()
                    } else if !preview.minimum_quality_met {
                        ui_copy("overlay_alchemy_missing_quality").to_owned()
                    } else {
                        ui_copy("overlay_alchemy_not_proven").to_owned()
                    }
                })
                .unwrap_or_else(|| ui_copy("overlay_alchemy_collapse").to_owned());
            let detail_y = if preview.recipe.is_some() && !preview.failure_reasons.is_empty() {
                process_y + 8.0
            } else {
                process_y + 4.0
            };
            draw_wrapped_text(
                &detail,
                x + 360.0,
                detail_y,
                w - 392.0,
                18.0,
                20.0,
                dark::TEXT_DIM,
            );
        }

    }

}
