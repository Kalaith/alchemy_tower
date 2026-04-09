use super::*;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_copy_optional, ui_format};
use crate::ui::{
    draw_action_button, draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle,
    draw_selection_card, draw_state_banner, draw_wrapped_text,
};

fn draw_overlay_section_title(x: f32, y: f32, title: &str, meta: Option<&str>) {
    draw_text(title, x, y, 24.0, dark::TEXT_BRIGHT);
    if let Some(meta) = meta {
        draw_text(meta, x + 208.0, y, 18.0, dark::TEXT_DIM);
    }
}

fn draw_overlay_section_box(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(16, 18, 26, 148));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(160, 170, 190, 40));
}

fn draw_overlay_tab(rect: Rect, label: &str, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::from_rgba(30, 34, 44, 220)
        } else {
            Color::from_rgba(16, 18, 26, 150)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        if selected {
            Color::from_rgba(255, 238, 196, 92)
        } else {
            Color::from_rgba(160, 170, 190, 56)
        },
    );
    let measured = measure_text(label, None, 18, 1.0);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + 21.0,
        18.0,
        if selected {
            Color::from_rgba(248, 242, 230, 255)
        } else {
            dark::TEXT_DIM
        },
    );
}

fn archive_tab_label(tab: &str) -> &'static str {
    ui_copy(match tab {
        "timeline" => "overlay_archive_tab_timeline",
        "experiments" => "overlay_archive_tab_experiments",
        "mastery" => "overlay_archive_tab_mastery",
        "morphs" => "overlay_archive_tab_morphs",
        "disassembly" => "overlay_archive_tab_disassembly",
        _ => "overlay_archive_tab_duplication",
    })
}

impl GameplayState {
    pub(super) fn draw_shop_overlay(&self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            return;
        };
        draw_overlay_backdrop();
        let x = 160.0;
        let y = 88.0;
        let w = screen_width() - 320.0;
        let h = screen_height() - 176.0;
        draw_panel(x, y, w, h, &station.name);
        draw_overlay_subtitle(
            x,
            y,
            &ui_text()
                .overlays
                .shop_subtitle
                .replace("{coins}", &self.coins.to_string()),
        );
        draw_overlay_tab(
            Rect::new(x + 20.0, y + 88.0, 112.0, 30.0),
            ui_copy("overlay_shop_buy_tab"),
            self.ui.shop_buy_tab,
        );
        draw_overlay_tab(
            Rect::new(x + 140.0, y + 88.0, 112.0, 30.0),
            ui_copy("overlay_shop_sell_tab"),
            !self.ui.shop_buy_tab,
        );
        draw_overlay_section_title(
            x + 20.0,
            y + 148.0,
            if self.ui.shop_buy_tab {
                ui_copy("overlay_shop_stock")
            } else {
                ui_copy("overlay_shop_sellable_stock")
            },
            Some(&ui_format(
                "overlay_sort_mode",
                &[("mode", self.inventory_sort_label())],
            )),
        );
        draw_overlay_section_box(x + 20.0, y + 162.0, w - 40.0, h - 224.0);

        let entries = if self.ui.shop_buy_tab {
            station
                .stock
                .iter()
                .map(|stock| {
                    (
                        stock.item_id.clone(),
                        stock.price,
                        self.coins >= stock.price,
                    )
                })
                .collect::<Vec<_>>()
        } else {
            self.sell_candidates(data)
                .into_iter()
                .map(|item_id| {
                    let price = self.sell_price(data, &item_id);
                    let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                    (item_id, price, amount > 0)
                })
                .collect::<Vec<_>>()
        };

        let mut row_y = y + 196.0;
        if !self.ui.shop_buy_tab {
            let safe_banner = entries
                .get(self.ui.shop_index)
                .map(|(item_id, _, _)| {
                    if self.sell_is_safe(data, item_id) {
                        ui_copy("overlay_safe_sell").to_owned()
                    } else {
                        ui_copy("overlay_keep_stock").to_owned()
                    }
                })
                .unwrap_or_else(|| ui_copy("overlay_safe_sell").to_owned());
            draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &safe_banner, false);
            row_y += 38.0;
        }
        if entries.is_empty() {
            let empty_text = if self.ui.shop_buy_tab {
                self.unavailable_state_text(ui_copy("overlay_shop_empty_buy"))
            } else {
                self.unavailable_state_text(ui_copy("overlay_shop_empty_sell"))
            };
            draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &empty_text, false);
        } else {
            for (index, (item_id, price, enabled)) in entries.iter().enumerate() {
                let selected = index == self.ui.shop_index;
                let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                let detail = if self.ui.shop_buy_tab {
                    self.inventory_reference_summary(data, item_id)
                } else {
                    let context = self.inventory_reference_summary(data, item_id);
                    context
                };
                let meta = if self.ui.shop_buy_tab {
                    self.item_card_meta(
                        data,
                        item_id,
                        amount,
                        &ui_format("overlay_buy_price", &[("price", &price.to_string())]),
                    )
                } else {
                    let sell_tag = if self.sell_is_safe(data, item_id) {
                        ui_format("overlay_sell_price_safe", &[("price", &price.to_string())])
                    } else {
                        ui_format("overlay_sell_price", &[("price", &price.to_string())])
                    };
                    self.item_card_meta(data, item_id, amount, &sell_tag)
                };
                draw_selection_card(
                    x + 32.0,
                    row_y - 24.0,
                    w - 64.0,
                    52.0,
                    selected,
                    *enabled,
                    data.item_name(item_id),
                    &detail,
                    &meta,
                );
                row_y += 60.0;
                if row_y > y + h - 40.0 {
                    break;
                }
            }
        }
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_format(
                "overlay_shop_footer",
                &[
                    ("switch", &input_bindings().shop.switch_tab),
                    ("select", &input_bindings().navigation.select),
                    ("confirm", &input_bindings().global.confirm),
                    ("close", &input_bindings().global.cancel),
                ],
            ),
        );
    }

    pub(super) fn draw_rune_overlay(&self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            return;
        };
        draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, &station.name);
        draw_overlay_subtitle(x, y, &ui_text().overlays.rune_subtitle);
        let recipes = self.available_rune_recipes(data, station);
        draw_overlay_section_title(x + 20.0, y + 124.0, ui_copy("overlay_rune_drafts"), None);
        draw_overlay_section_box(x + 20.0, y + 138.0, w - 40.0, h - 200.0);
        let mut row_y = y + 172.0;
        if recipes.is_empty() {
            draw_state_banner(
                x + 32.0,
                row_y - 16.0,
                w - 64.0,
                &self.unavailable_state_text(ui_copy("overlay_rune_empty")),
                false,
            );
        } else {
            for (index, recipe) in recipes.iter().enumerate() {
                let selected = index == self.ui.rune_index;
                draw_selection_card(
                    x + 32.0,
                    row_y - 24.0,
                    w - 64.0,
                    58.0,
                    selected,
                    true,
                    &format!(
                        "{} -> {}",
                        data.item_name(&recipe.input_item_id),
                        data.item_name(&recipe.output_item_id)
                    ),
                    &recipe.description,
                    &ui_format(
                        "overlay_rune_label",
                        &[("item", data.item_name(&recipe.rune_item_id))],
                    ),
                );
                row_y += 64.0;
            }
        }
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_copy("overlay_rune_footer")
                .replace("{select}", &input_bindings().navigation.select)
                .replace("{confirm}", &input_bindings().global.confirm)
                .replace("{close}", &input_bindings().global.cancel),
        );
    }

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

    fn draw_archive_experiments_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
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

    fn draw_archive_morphs_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
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
                    ui_format("overlay_condition_catalyst", &[("value", ui_copy("overlay_any"))])
                } else {
                    ui_format(
                        "overlay_condition_catalyst",
                        &[("value", &morph.catalyst_tag)],
                    )
                },
                if morph.required_timing.is_empty() {
                    ui_format("overlay_condition_timing", &[("value", ui_copy("overlay_any"))])
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
                            &self.inventory.get(item_id).copied().unwrap_or_default().to_string(),
                        ),
                        ("cost", &(item.base_value + u32::from(item.rarity) * 10).to_string()),
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

    pub(super) fn draw_ending_overlay(&self) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 180),
        );
        let x = 170.0;
        let y = 110.0;
        let w = screen_width() - 340.0;
        let h = screen_height() - 220.0;
        draw_panel(x, y, w, h, ui_copy("overlay_ending_title"));
        draw_wrapped_text(
            &narrative_text().overlays.observatory_epilogue,
            x + 24.0,
            y + 60.0,
            w - 48.0,
            22.0,
            28.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &narrative_text().overlays.observatory_footer,
            x + 24.0,
            y + h - 24.0,
            18.0,
            dark::TEXT_DIM,
        );
    }

    pub(super) fn draw_dialogue_overlay(&self, data: &GameData) {
        let Some(npc_id) = self.dialogue_npc_id() else {
            return;
        };
        let Some(npc) = data.npc(npc_id) else {
            return;
        };
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 130),
        );
        let x = 180.0;
        let y = screen_height() - 286.0;
        let w = screen_width() - 360.0;
        let h = 226.0;
        draw_panel(x, y, w, h, &npc.name);
        draw_text(
            &ui_format("overlay_now", &[("text", &self.npc_now_hint(data, npc))]),
            x + 20.0,
            y + 34.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &ui_format(
                "overlay_later",
                &[("text", &self.npc_later_hint(data, npc))],
            ),
            x + 20.0,
            y + 54.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &ui_format(
                "overlay_usually",
                &[("text", &self.npc_usual_hint(data, npc))],
            ),
            x + 20.0,
            y + 72.0,
            w - 40.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        let text = self.current_dialogue_text(data, npc);
        draw_wrapped_text(
            &text,
            x + 20.0,
            y + 104.0,
            w - 40.0,
            20.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &self.current_dialogue_footer(data, npc),
            x + 20.0,
            y + h - 28.0,
            20.0,
            dark::TEXT_DIM,
        );
    }

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

    fn draw_journal_routes_tab(&self, data: &GameData, x: f32, y: f32, w: f32, h: f32) {
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

    fn draw_journal_brews_tab(&self, data: &GameData, x: f32, y: f32, _w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_potion_memories"),
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut brew_y = y + 168.0;
        let potion_memories = self.potion_memories(data);
        if potion_memories.is_empty() {
            draw_text(
                ui_copy("journal_memory_no_potions"),
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_DIM,
            );
            return;
        }
        for entry in potion_memories {
            draw_text(
                data.item_name(&entry.item_id),
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_BRIGHT,
            );
            brew_y += 20.0;
            draw_text(
                &self.journal_potion_state_line(entry),
                x + 20.0,
                brew_y,
                17.0,
                dark::TEXT_DIM,
            );
            brew_y += 18.0;
            draw_wrapped_text(
                &self.journal_potion_recap(data, &entry.item_id),
                x + 20.0,
                brew_y,
                520.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            if let Some(profile) = self.progression.crafted_item_profiles.get(&entry.item_id) {
                let effects = if profile.effect_kinds.is_empty() {
                    ui_copy("journal_memory_effects_none").to_owned()
                } else {
                    profile.effect_kinds.join(", ")
                };
                draw_text(
                    &ui_format("journal_memory_effects", &[("effects", &effects)]),
                    x + 580.0,
                    brew_y - 2.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                if !profile.inherited_traits.is_empty() {
                    draw_text(
                        &ui_format(
                            "inventory_memory_traits",
                            &[("traits", &profile.inherited_traits.join(", "))],
                        ),
                        x + 580.0,
                        brew_y + 20.0,
                        18.0,
                        dark::TEXT_DIM,
                    );
                }
            }
            brew_y += 40.0;
            if entry.best_quality_score > 0 {
                draw_text(
                    &ui_format(
                        "journal_memory_best_brew",
                        &[
                            ("quality", &entry.best_quality_score.to_string()),
                            ("band", &entry.best_quality_band),
                        ],
                    ),
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if !entry.last_recipe_id.is_empty() {
                let recipe_name = data
                    .recipes
                    .iter()
                    .find(|recipe| recipe.id == entry.last_recipe_id)
                    .map(|recipe| recipe.name.as_str())
                    .unwrap_or(entry.last_recipe_id.as_str());
                draw_text(
                    &ui_format("journal_memory_formula", &[("formula", recipe_name)]),
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if entry.successful_brews > 0 {
                draw_text(
                    &ui_format(
                        "journal_memory_successful_brews",
                        &[("count", &entry.successful_brews.to_string())],
                    ),
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if brew_y > y + h - 40.0 {
                break;
            }
        }
    }

    fn journal_herb_summary(&self, data: &GameData, item_id: &str) -> String {
        let key = format!("journal_herb_summary_{item_id}");
        ui_copy_optional(&key)
            .map(str::to_owned)
            .or_else(|| data.item(item_id).map(|item| item.description.clone()))
            .unwrap_or_else(|| data.item_name(item_id).to_owned())
    }

    fn journal_potion_recap(&self, data: &GameData, item_id: &str) -> String {
        let key = format!("journal_potion_recap_{item_id}");
        ui_copy_optional(&key)
            .map(str::to_owned)
            .or_else(|| data.item(item_id).map(|item| item.description.clone()))
            .unwrap_or_else(|| data.item_name(item_id).to_owned())
    }

    fn journal_potion_state_line(&self, entry: &PotionMemoryEntry) -> String {
        let mut parts = Vec::new();
        if entry.seen {
            parts.push(ui_copy("journal_memory_state_seen").to_owned());
        }
        if entry.learned {
            parts.push(ui_copy("journal_memory_state_learned").to_owned());
        }
        if entry.successful_brews > 0 {
            parts.push(ui_copy("journal_memory_state_brewed").to_owned());
        }
        ui_format(
            "journal_memory_state_line",
            &[("state", &parts.join("  |  "))],
        )
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

    pub(super) fn draw_quest_board_overlay(&self, data: &GameData) {
        draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, ui_copy("overlay_quest_board_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.quest_board_subtitle);
        let available = self.available_board_quests(data);
        draw_overlay_section_title(x + 20.0, y + 122.0, ui_copy("overlay_quest_available"), None);
        draw_overlay_section_box(x + 20.0, y + 136.0, w - 40.0, 232.0);
        let mut row_y = y + 168.0;
        if available.is_empty() {
            draw_state_banner(
                x + 32.0,
                row_y - 16.0,
                w - 64.0,
                &self.unavailable_state_text(ui_copy("overlay_quest_none_available")),
                false,
            );
        } else {
            for (index, quest_id) in available.iter().enumerate() {
                let selected = index == self.ui.shop_index;
                if let Some(quest) = data.quest(quest_id) {
                    let giver_hint = self.quest_location_hint(data, quest);
                    draw_selection_card(
                        x + 32.0,
                        row_y - 24.0,
                        w - 64.0,
                        58.0,
                        selected,
                        true,
                        &quest.title,
                        &giver_hint,
                        &ui_format(
                            "overlay_reward",
                            &[("coins", &quest.reward_coins.to_string())],
                        ),
                    );
                }
                row_y += 64.0;
            }
        }
        draw_text(
            ui_copy("overlay_quest_locked"),
            x + 20.0,
            y + h - 200.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_overlay_section_box(x + 20.0, y + h - 186.0, w - 40.0, 54.0);
        let locked = data
            .quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| !self.quest_is_available(quest))
            .collect::<Vec<_>>();
        let locked_text = if locked.is_empty() {
            ui_copy("overlay_none").to_owned()
        } else {
            locked
                .iter()
                .map(|quest| {
                    format!(
                        "{}: {}",
                        quest.title,
                        self.locked_state_text(&self.quest_unlock_summary(quest))
                    )
                })
                .collect::<Vec<_>>()
                .join("  ")
        };
        draw_wrapped_text(
            &locked_text,
            x + 32.0,
            y + h - 164.0,
            w - 64.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            ui_copy("overlay_quest_active"),
            x + 20.0,
            y + h - 120.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_overlay_section_box(x + 20.0, y + h - 106.0, w - 40.0, 52.0);
        let active_orders = self
            .progression
            .started_quests
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .map(|quest| quest.title.clone())
            .collect::<Vec<_>>();
        let active_text = if active_orders.is_empty() {
            ui_copy("overlay_none").to_owned()
        } else {
            active_orders.join(", ")
        };
        draw_wrapped_text(
            &active_text,
            x + 32.0,
            y + h - 84.0,
            w - 64.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_copy("overlay_rune_footer")
                .replace("{select}", &input_bindings().navigation.select)
                .replace("{confirm}", &input_bindings().global.confirm)
                .replace("{close}", &input_bindings().global.cancel),
        );
    }

    pub(super) fn draw_alchemy_overlay(&self, data: &GameData, art: &ArtAssets) {
        draw_overlay_backdrop();
        let x = 80.0;
        let y = 64.0;
        let w = screen_width() - 160.0;
        let h = screen_height() - 128.0;
        draw_panel(x, y, w, h, ui_copy("overlay_alchemy_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.alchemy_subtitle);
        if let Some(texture) = art.effect("brew_bubble_effect") {
            let alpha = 0.55 + ((get_time() as f32 * 2.4).sin() * 0.5 + 0.5) * 0.25;
            draw_texture_centered(
                texture,
                vec2(x + w - 54.0, y + 44.0),
                vec2(42.0, 42.0),
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }

        let items = self.alchemy_materials(data);
        draw_overlay_section_title(
            x + 20.0,
            y + 84.0,
            ui_copy("overlay_materials"),
            Some(&ui_format(
                "overlay_sort_mode",
                &[("mode", self.inventory_sort_label())],
            )),
        );
        draw_overlay_section_box(x + 18.0, y + 98.0, 286.0, 162.0);
        let mut iy = y + 82.0;
        if items.is_empty() {
            draw_state_banner(
                x + 30.0,
                iy - 12.0,
                262.0,
                &self.unavailable_state_text(ui_copy("overlay_alchemy_empty_materials")),
                false,
            );
        } else {
            for (index, item_id) in items.iter().enumerate() {
                let selected = index == self.alchemy.index;
                let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                let ready = amount.saturating_sub(self.reserved_count(item_id));
                let subtitle = data
                    .item(item_id)
                    .map(|item| item.description.as_str())
                    .unwrap_or("");
                draw_selection_card(
                    x + 30.0,
                    iy - 24.0,
                    262.0,
                    52.0,
                    selected,
                    ready > 0,
                    data.item_name(item_id),
                    subtitle,
                    &self.item_card_meta(
                        data,
                        item_id,
                        amount,
                        &ui_format(
                            "overlay_materials_meta",
                            &[
                                ("ready", &ready.to_string()),
                                ("reserved", &self.reserved_count(item_id).to_string()),
                                ("reference", &self.inventory_reference_summary(data, item_id)),
                            ],
                        ),
                    ),
                );
                iy += 58.0;
            }
        }

        draw_overlay_section_title(x + 20.0, y + 270.0, ui_copy("overlay_alchemy_controls"), None);
        draw_overlay_section_box(x + 18.0, y + 284.0, 286.0, 74.0);
        draw_wrapped_text(
            &ui_format(
                "overlay_alchemy_controls_line1",
                &[
                    ("browse", &input_bindings().navigation.select),
                    ("heat", &input_bindings().alchemy.heat),
                ],
            ),
            x + 32.0,
            y + 304.0,
            262.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &ui_format(
                "overlay_alchemy_controls_line2",
                &[
                    ("fill", &input_bindings().alchemy.fill_slots),
                    ("catalyst", &input_bindings().alchemy.catalyst),
                ],
            ),
            x + 32.0,
            y + 334.0,
            262.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );

        draw_overlay_section_title(x + 340.0, y + 84.0, ui_copy("overlay_slots"), None);
        draw_overlay_section_box(x + 340.0, y + 98.0, w - 360.0, 134.0);
        draw_text(
            &ui_format(
                "overlay_alchemy_process",
                &[
                    ("heat", &self.alchemy.heat.to_string()),
                    ("stirs", &self.alchemy.stirs.to_string()),
                    ("timing", self.alchemy_timing()),
                ],
            ),
            x + 340.0,
            y + 106.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_action_button(Rect::new(x + 520.0, y + 88.0, 28.0, 28.0), "-", 0.0);
        draw_action_button(Rect::new(x + 552.0, y + 88.0, 28.0, 28.0), "+", 0.0);
        draw_action_button(
            Rect::new(x + 612.0, y + 88.0, 92.0, 28.0),
            ui_copy("overlay_alchemy_stir_button"),
            0.0,
        );
        draw_action_button(
            Rect::new(x + 716.0, y + 88.0, 156.0, 28.0),
            ui_copy("overlay_alchemy_timing_button"),
            0.0,
        );
        for slot in 0..SLOT_COUNT {
            let sx = x + 340.0 + slot as f32 * 140.0;
            draw_rectangle(
                sx,
                y + 120.0,
                120.0,
                100.0,
                Color::from_rgba(28, 32, 42, 255),
            );
            draw_rectangle(
                sx,
                y + 120.0,
                4.0,
                100.0,
                Color::from_rgba(176, 226, 255, 96),
            );
            draw_rectangle_lines(
                sx,
                y + 120.0,
                120.0,
                100.0,
                1.5,
                Color::from_rgba(160, 170, 190, 58),
            );
            draw_text(
                &ui_format("overlay_slot_label", &[("slot", &(slot + 1).to_string())]),
                sx + 16.0,
                y + 146.0,
                22.0,
                dark::TEXT_BRIGHT,
            );
            draw_text(
                self.alchemy.slots[slot]
                    .as_deref()
                    .map(|id| data.item_name(id))
                    .unwrap_or(ui_copy("overlay_alchemy_empty_slot")),
                sx + 12.0,
                y + 188.0,
                20.0,
                dark::TEXT,
            );
            draw_text(
                if self.alchemy.slots[slot].is_some() {
                    ui_copy("overlay_slot_click_clear")
                } else {
                    ui_copy("overlay_slot_click_fill")
                },
                sx + 12.0,
                y + 206.0,
                16.0,
                dark::TEXT_DIM,
            );
        }
        draw_rectangle(
            x + 760.0,
            y + 120.0,
            160.0,
            100.0,
            Color::from_rgba(28, 32, 42, 255),
        );
        draw_rectangle(
            x + 760.0,
            y + 120.0,
            4.0,
            100.0,
            Color::from_rgba(255, 214, 132, 96),
        );
        draw_rectangle_lines(
            x + 760.0,
            y + 120.0,
            160.0,
            100.0,
            1.5,
            Color::from_rgba(160, 170, 190, 58),
        );
        draw_text(
            ui_copy("overlay_catalyst"),
            x + 776.0,
            y + 146.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            self.alchemy
                .catalyst
                .as_deref()
                .map(|id| data.item_name(id))
                .unwrap_or(ui_copy("overlay_alchemy_empty_slot")),
            x + 772.0,
            y + 188.0,
            20.0,
            dark::TEXT,
        );
        draw_text(
            if self.alchemy.catalyst.is_some() {
                ui_copy("overlay_catalyst_click_clear")
            } else {
                ui_copy("overlay_catalyst_click_assign")
            },
            x + 772.0,
            y + 206.0,
            16.0,
            dark::TEXT_DIM,
        );

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

        draw_overlay_section_title(x + 20.0, y + 392.0, ui_copy("overlay_alchemy_known_formulae"), None);
        draw_overlay_section_box(x + 18.0, y + 406.0, 286.0, 142.0);
        let mut ky = y + 424.0;
        let mut any_known = false;
        for recipe in &data.recipes {
            if self.progression.known_recipes.contains(&recipe.id) {
                any_known = true;
                draw_text(&recipe.name, x + 20.0, ky, 22.0, dark::TEXT_BRIGHT);
                ky += 22.0;
                draw_text(
                    &self.recipe_memory_meta(data, recipe),
                    x + 20.0,
                    ky,
                    18.0,
                    dark::TEXT_DIM,
                );
                ky += 20.0;
                draw_wrapped_text(
                    &self.recipe_memory_detail(data, recipe),
                    x + 20.0,
                    ky,
                    286.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                ky += 36.0;
                draw_wrapped_text(
                    &recipe.lore_note,
                    x + 20.0,
                    ky,
                    286.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                ky += 32.0;
            }
        }
        if !any_known {
            draw_text(
                ui_copy("overlay_alchemy_no_formulae"),
                x + 32.0,
                ky,
                20.0,
                dark::TEXT_DIM,
            );
        }

        draw_action_button(
            Rect::new(x + 20.0, y + 368.0, 82.0, 28.0),
            ui_copy("overlay_alchemy_sort_button"),
            0.0,
        );
        draw_action_button(
            Rect::new(x + 114.0, y + 368.0, 82.0, 28.0),
            ui_copy("overlay_alchemy_clear_button"),
            0.0,
        );
        draw_action_button(
            Rect::new(x + 208.0, y + 368.0, 90.0, 28.0),
            ui_copy("overlay_alchemy_repeat_button"),
            0.0,
        );
        draw_rectangle(
            x + 310.0,
            y + 368.0,
            90.0,
            28.0,
            Color::from_rgba(38, 58, 46, 210),
        );
        draw_rectangle_lines(
            x + 310.0,
            y + 368.0,
            90.0,
            28.0,
            1.5,
            Color::from_rgba(188, 255, 220, 96),
        );
        draw_text(
            ui_copy("overlay_alchemy_brew_button"),
            x + 338.0,
            y + 388.0,
            18.0,
            dark::TEXT_BRIGHT,
        );

        draw_overlay_footer(x, y, w, h, ui_copy("overlay_alchemy_mouse_footer"));
    }
}
