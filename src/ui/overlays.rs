use super::*;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::ui::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_selection_card,
    draw_state_banner, draw_wrapped_text,
};

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
        draw_text(
            if self.ui.shop_buy_tab { "Buy <" } else { "Buy" },
            x + 20.0,
            y + 88.0,
            24.0,
            if self.ui.shop_buy_tab {
                dark::ACCENT
            } else {
                dark::TEXT_DIM
            },
        );
        draw_text(
            if self.ui.shop_buy_tab { "Sell" } else { "Sell <" },
            x + 150.0,
            y + 88.0,
            24.0,
            if self.ui.shop_buy_tab {
                dark::TEXT_DIM
            } else {
                dark::ACCENT
            },
        );
        draw_text(
            &ui_format("overlay_sort_mode", &[("mode", self.inventory_sort_label())]),
            x + w - 180.0,
            y + 88.0,
            20.0,
            dark::TEXT_DIM,
        );

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

        let mut row_y = y + 128.0;
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
            draw_state_banner(x + 20.0, row_y - 16.0, w - 40.0, &safe_banner, false);
            row_y += 38.0;
        }
        if entries.is_empty() {
            let empty_text = if self.ui.shop_buy_tab {
                self.unavailable_state_text("Nothing is stocked right now.")
            } else {
                self.unavailable_state_text("Nothing safe to sell right now.")
            };
            draw_state_banner(x + 20.0, row_y - 16.0, w - 40.0, &empty_text, false);
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
                    self.item_card_meta(data, item_id, amount, &ui_format("overlay_buy_price", &[("price", &price.to_string())]))
                } else {
                    let sell_tag = if self.sell_is_safe(data, item_id) {
                        ui_format("overlay_sell_price_safe", &[("price", &price.to_string())])
                    } else {
                        ui_format("overlay_sell_price", &[("price", &price.to_string())])
                    };
                    self.item_card_meta(data, item_id, amount, &sell_tag)
                };
                draw_selection_card(
                    x + 20.0,
                    row_y - 24.0,
                    w - 40.0,
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
            &format!(
                "{} switch tab  |  {} select  |  V sort  |  {} confirm  |  {} close",
                input_bindings().shop.switch_tab,
                input_bindings().navigation.select,
                input_bindings().global.confirm,
                input_bindings().global.cancel,
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
        let mut row_y = y + 96.0;
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                row_y - 16.0,
                w - 40.0,
                &self.unavailable_state_text("Bring a potion and a matching rune here."),
                false,
            );
        } else {
            for (index, recipe) in recipes.iter().enumerate() {
                let selected = index == self.ui.rune_index;
                draw_selection_card(
                    x + 20.0,
                    row_y - 24.0,
                    w - 40.0,
                    58.0,
                    selected,
                    true,
                    &format!(
                        "{} -> {}",
                        data.item_name(&recipe.input_item_id),
                        data.item_name(&recipe.output_item_id)
                    ),
                    &recipe.description,
                    &ui_format("overlay_rune_label", &[("item", data.item_name(&recipe.rune_item_id))]),
                );
                row_y += 64.0;
            }
        }
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &format!(
                "{} select  |  {} confirm  |  {} close",
                input_bindings().navigation.select,
                input_bindings().global.confirm,
                input_bindings().global.cancel,
            ),
        );
    }

    pub(super) fn draw_archive_overlay(&self, data: &GameData) {
        draw_overlay_backdrop();
        let x = 150.0;
        let y = 70.0;
        let w = screen_width() - 300.0;
        let h = screen_height() - 140.0;
        draw_panel(x, y, w, h, "Archives");
        draw_overlay_subtitle(x, y, &ui_text().overlays.archive_subtitle);
        self.draw_archive_tabs(x, y, w);

        match ARCHIVE_TABS[self.ui.archive_tab] {
            "Timeline" => self.draw_archive_timeline_section(x, y, w, h),
            "Experiments" => self.draw_archive_experiments_section(data, x, y, w, h),
            "Mastery" => self.draw_archive_mastery_section(data, x, y, w, h),
            "Morphs" => self.draw_archive_morphs_section(data, x, y, w, h),
            "Disassembly" => self.draw_archive_disassembly_section(data, x, y, w, h),
            _ => self.draw_archive_duplication_section(data, x, y, w, h),
        }
        draw_overlay_footer(x, y, w, h, &self.archive_footer_text());
    }

    fn draw_archive_tabs(&self, x: f32, y: f32, w: f32) {
        for (index, tab) in ARCHIVE_TABS.iter().enumerate() {
            let rect = Rect::new(x + 20.0 + index as f32 * 148.0, y + 54.0, 136.0, 28.0);
            let selected = index == self.ui.archive_tab;
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if selected {
                    dark::ACCENT
                } else {
                    Color::from_rgba(38, 40, 50, 255)
                },
            );
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, dark::ACCENT);
            draw_text(
                tab,
                rect.x + 10.0,
                rect.y + 20.0,
                18.0,
                if selected {
                    dark::TEXT_BRIGHT
                } else {
                    dark::TEXT_DIM
                },
            );
            if rect.x + rect.w > x + w - 20.0 {
                break;
            }
        }
    }

    fn draw_archive_timeline_section(&self, x: f32, y: f32, w: f32, h: f32) {
        draw_text(
            "Recovered Milestones",
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

        draw_text(ui_copy("overlay_tower_status"), x + 500.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let status_lines = [
            ui_format("overlay_brews_completed", &[("count", &self.progression.total_brews.to_string())]),
            ui_format("overlay_known_recipes", &[("count", &self.progression.known_recipes.len().to_string())]),
            ui_format("overlay_recorded_experiments", &[("count", &self.progression.experiment_log.len().to_string())]),
            ui_format("overlay_unlocked_routes", &[("count", &self.progression.unlocked_warps.len().to_string())]),
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
        draw_text(ui_copy("overlay_experiment_history"), x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let entries = self.archive_experiment_entries(data);
        draw_text(
            &ui_format("overlay_filter", &[("mode", self.archive_experiment_filter_label())]),
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
                &self.unavailable_state_text("No experiments logged yet."),
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
                &[("page", &(page + 1).to_string()), ("pages", &page_count.to_string())],
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
                "Unknown recipe".to_owned()
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
                &format!(
                    "day {}  {} {}",
                    entry.day_index + 1,
                    entry.quality_band,
                    if entry.stable { "stable" } else { "unstable" }
                ),
            );
            list_y += 64.0;
        }

        let selected_entry = entries[selected_index];
        draw_text(ui_copy("overlay_selected_record"), x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        draw_text(
            &ui_format("overlay_output", &[("item", data.item_name(&selected_entry.output_item_id))]),
            x + 410.0,
            y + 156.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &format!(
                "Quality: {} ({})",
                selected_entry.quality_score, selected_entry.quality_band
            ),
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &format!(
                "Result: {}",
                if selected_entry.stable {
                    "stable synthesis"
                } else {
                    "unstable salvage"
                }
            ),
            x + 410.0,
            y + 208.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &format!(
                "Catalyst: {}",
                if selected_entry.catalyst_item_id.is_empty() {
                    "none".to_owned()
                } else {
                    data.item_name(&selected_entry.catalyst_item_id).to_owned()
                }
            ),
            x + 410.0,
            y + 232.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &format!(
                "Morph: {}",
                if selected_entry.morph_output_item_id.is_empty() {
                    "none".to_owned()
                } else {
                    data.item_name(&selected_entry.morph_output_item_id).to_owned()
                }
            ),
            x + 410.0,
            y + 256.0,
            20.0,
            dark::TEXT_DIM,
        );
        if let Some(recipe) = data.recipes.iter().find(|recipe| recipe.id == selected_entry.recipe_id) {
            draw_text(
                &format!(
                    "Mastery now: {}",
                    crate::alchemy::mastery_stage(self.recipe_mastery_brews(&recipe.id))
                ),
                x + 410.0,
                y + 282.0,
                20.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &format!(
                    "Memory: {}",
                    self.recipe_memory_meta(data, recipe)
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
        draw_text(ui_copy("overlay_recipe_mastery"), x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let recipes = self.mastery_recipes(data);
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text("No mastered formulas recorded yet."),
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
        draw_text(ui_copy("overlay_mastery_detail"), x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        draw_text(&recipe.name, x + 410.0, y + 156.0, 24.0, dark::TEXT_BRIGHT);
        draw_text(
            &format!(
                "Stage: {}  Successful brews: {}",
                crate::alchemy::mastery_stage(mastery),
                mastery
            ),
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        if let Some(profile) = self.progression.crafted_item_profiles.get(&recipe.output_item_id) {
            draw_text(
                &format!(
                    "Best result: {} ({})",
                    profile.best_quality_score, profile.best_quality_band
                ),
                x + 410.0,
                y + 210.0,
                20.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &format!(
                    "Traits carried: {}",
                    if profile.inherited_traits.is_empty() {
                        "none".to_owned()
                    } else {
                        profile.inherited_traits.join(", ")
                    }
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
                &format!(
                    "Last logged attempt: day {} as {}",
                    entry.day_index + 1,
                    entry.quality_band
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
        draw_text(ui_copy("overlay_morph_previews"), x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let recipes = self.morph_recipes(data);
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text("No branching formulas recovered yet."),
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
                &format!("{} branch(es)", recipe.morph_targets.len()),
            );
            list_y += 64.0;
        }

        let recipe = recipes[selected_index];
        draw_text(ui_copy("overlay_branch_detail"), x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        if let Some(entry) = self
            .progression
            .experiment_log
            .iter()
            .rev()
            .find(|entry| entry.recipe_id == recipe.id && !entry.morph_output_item_id.is_empty())
        {
            draw_text(
                &format!(
                    "Last morph logged: day {} -> {}",
                    entry.day_index + 1,
                    data.item_name(&entry.morph_output_item_id)
                ),
                x + 410.0,
                y + 146.0,
                20.0,
                dark::TEXT_DIM,
            );
        }
        let mut detail_y = y + 176.0;
        for morph in &recipe.morph_targets {
            let discovered = self.progression.crafted_item_profiles.contains_key(&morph.output_item_id);
            draw_text(
                &format!(
                    "{}{}",
                    data.item_name(&morph.output_item_id),
                    if discovered { " [logged]" } else { "" }
                ),
                x + 410.0,
                detail_y,
                22.0,
                dark::TEXT_BRIGHT,
            );
            detail_y += 22.0;
            let conditions = [
                ui_format("overlay_condition_quality", &[("value", &morph.minimum_quality.to_string())]),
                ui_format("overlay_condition_heat", &[("value", &morph.required_heat.to_string())]),
                ui_format("overlay_condition_stirs", &[("value", &morph.required_stirs.to_string())]),
                if morph.catalyst_tag.is_empty() {
                    ui_format("overlay_condition_catalyst", &[("value", "any")])
                } else {
                    ui_format("overlay_condition_catalyst", &[("value", &morph.catalyst_tag)])
                },
                if morph.required_timing.is_empty() {
                    ui_format("overlay_condition_timing", &[("value", "any")])
                } else {
                    ui_format("overlay_condition_timing", &[("value", &morph.required_timing)])
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
        draw_text(ui_copy("overlay_disassembly"), x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let recipes = self.available_disassembly_recipes(data);
        if recipes.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text("Bring a brewed item from a logged recipe."),
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
                &format!(
                    "owned {}",
                    self.inventory
                        .get(&recipe.output_item_id)
                        .copied()
                        .unwrap_or_default()
                ),
            );
            list_y += 64.0;
        }

        let recipe = recipes[selected_index];
        draw_text(ui_copy("overlay_recovered_inputs"), x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let mut detail_y = y + 156.0;
        for ingredient in &recipe.ingredients {
            draw_text(
                &format!("{} x{}", data.item_name(&ingredient.item_id), ingredient.amount),
                x + 410.0,
                detail_y,
                22.0,
                dark::TEXT_DIM,
            );
            detail_y += 24.0;
        }
        draw_wrapped_text(
            "Enter confirm will break one brewed item back into its authored recipe inputs.",
            x + 410.0,
            detail_y + 18.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
    }

    fn draw_archive_duplication_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text(ui_copy("overlay_duplication"), x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let items = self.duplication_candidates(data);
        if items.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text("Bring a potion, ingredient, or catalyst plus a starlight shard."),
                false,
            );
            return;
        }

        let selected_index = self.ui.archive_index.min(items.len().saturating_sub(1));
        let mut list_y = y + 154.0;
        for (index, item_id) in items.iter().take(6).enumerate() {
            let item = data.item(item_id).expect("duplication candidate should exist");
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                index == selected_index,
                self.can_duplicate_item(data, item_id),
                data.item_name(item_id),
                &self.inventory_reference_summary(data, item_id),
                &format!(
                    "owned {}  cost {}c",
                    self.inventory.get(item_id).copied().unwrap_or_default(),
                    item.base_value + u32::from(item.rarity) * 10
                ),
            );
            list_y += 64.0;
        }

        let item_id = &items[selected_index];
        let item = data.item(item_id).expect("duplication candidate should exist");
        draw_text(ui_copy("overlay_duplication_cost"), x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        draw_text(
            &ui_format("overlay_target", &[("item", &item.name)]),
            x + 410.0,
            y + 156.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &ui_format("overlay_coins", &[("count", &(item.base_value + u32::from(item.rarity) * 10).to_string())]),
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &format!(
                "Catalyst: {}",
                self.duplication_catalyst_item_id(data)
                    .as_deref()
                    .map(|id| data.item_name(id))
                    .unwrap_or("starlight shard required")
            ),
            x + 410.0,
            y + 210.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            "Enter confirm will consume one starlight catalyst and the listed coin cost to create one additional copy.",
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
            "Timeline" => "Left/Right section  |  Enter reconstruct  |  Esc close".to_owned(),
            "Disassembly" | "Duplication" => {
                "Left/Right section  |  Up/Down select  |  Enter confirm  |  Esc close"
                    .to_owned()
            }
            "Experiments" => {
                "Left/Right section  |  Up/Down browse  |  F filter  |  Esc close".to_owned()
            }
            "Mastery" | "Morphs" => {
                "Left/Right section  |  Up/Down browse  |  Esc close".to_owned()
            }
            _ => "Left/Right section  |  Esc close".to_owned(),
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
        draw_panel(x, y, w, h, "Observatory");
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
        let Some(npc_id) = &self.ui.current_npc_id else {
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
            &ui_format("overlay_later", &[("text", &self.npc_later_hint(data, npc))]),
            x + 20.0,
            y + 54.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &ui_format("overlay_usually", &[("text", &self.npc_usual_hint(data, npc))]),
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
        draw_panel(x, y, w, h, "Field Journal");
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
            "Close",
            close_rect.x + 18.0,
            close_rect.y + 19.0,
            18.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &format!(
                "Current Conditions: {} / {}",
                self.current_season(),
                self.current_weather()
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
                draw_texture_centered(texture, vec2(rect.x + 18.0, rect.y + 14.0), vec2(18.0, 18.0), WHITE);
            }
        }

        match tabs.get(self.ui.journal_tab).copied().unwrap_or("Routes") {
            "Routes" => self.draw_journal_routes_tab(data, x, y, w, h),
            "Notes" => self.draw_journal_notes_tab(data, x, y, w, h),
            "Brews" => self.draw_journal_brews_tab(data, x, y, w, h),
            "Greenhouse" => self.draw_journal_greenhouse_tab(data, x, y, w, h),
            _ => self.draw_journal_rapport_tab(data, x, y, w, h),
        }
        draw_text(
            "Click tabs  |  Left/Right switch tab  |  Esc or J to close",
            x + 20.0,
            y + h - 20.0,
            18.0,
            dark::TEXT_DIM,
        );
    }

    fn draw_journal_routes_tab(&self, data: &GameData, x: f32, y: f32, w: f32, h: f32) {
        draw_text(ui_copy("overlay_known_routes"), x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
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
            "Recorded Specimens",
            x + 420.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut entry_y = y + 168.0;
            if self.progression.field_journal.is_empty() {
                draw_text(
                    "No specimens recorded yet.",
                    x + 420.0,
                entry_y,
                22.0,
                dark::TEXT_DIM,
            );
            } else {
                for entry in self.progression.field_journal.values() {
                    let route_name = data
                        .route(&entry.route_id)
                        .map(|route| route.name.as_str())
                        .unwrap_or("Unknown Route");
                    draw_text(
                        data.item_name(&entry.item_id),
                        x + 420.0,
                        entry_y,
                        22.0,
                        dark::TEXT_BRIGHT,
                    );
                    entry_y += 22.0;
                    draw_text(
                        &format!(
                            "Observed at {} / {} / {} / {}",
                            route_name, entry.season, entry.weather, entry.time_window
                        ),
                        x + 420.0,
                        entry_y,
                        18.0,
                        dark::TEXT_DIM,
                    );
                    entry_y += 20.0;
                    if let Some(conditions) = self.learned_gathering_conditions(data, &entry.item_id) {
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
                    }
                    let quality_line = if entry.variant_name.is_empty() {
                        format!(
                            "Best quality: {} ({})",
                            entry.best_quality, entry.best_quality_band
                        )
                    } else {
                        format!(
                            "Best quality: {} ({})  Variant: {}",
                            entry.best_quality, entry.best_quality_band, entry.variant_name
                        )
                    };
                    draw_text(&quality_line, x + 420.0, entry_y, 18.0, dark::TEXT_DIM);
                    entry_y += 20.0;
                    draw_text(&entry.note, x + 420.0, entry_y, 18.0, dark::TEXT_DIM);
                    entry_y += 28.0;
                    if entry_y > y + h - 160.0 {
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
                "All current tower routes are restored.",
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
        draw_text(ui_copy("overlay_tower_notes"), x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
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
        draw_text(ui_copy("overlay_brew_ledger"), x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
        let mut brew_y = y + 168.0;
        let mut any_brew_note = false;
        for recipe in &data.recipes {
            if !self.progression.known_recipes.contains(&recipe.id) {
                continue;
            }
            any_brew_note = true;
            draw_text(&recipe.name, x + 20.0, brew_y, 20.0, dark::TEXT_BRIGHT);
            brew_y += 20.0;
            draw_text(
                &self.recipe_memory_meta(data, recipe),
                x + 20.0,
                brew_y,
                17.0,
                dark::TEXT_DIM,
            );
            brew_y += 18.0;
            draw_wrapped_text(
                &self.recipe_memory_detail(data, recipe),
                x + 20.0,
                brew_y,
                520.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            draw_wrapped_text(
                &recipe.lore_note,
                x + 580.0,
                brew_y - 18.0,
                280.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            brew_y += 40.0;
            if brew_y > y + h - 40.0 {
                break;
            }
        }
        if !any_brew_note {
            draw_text(
                "No formula notes recorded yet.",
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_DIM,
            );
        }
    }

    fn draw_journal_greenhouse_tab(&self, data: &GameData, x: f32, y: f32, _w: f32, h: f32) {
        draw_text(
            "Greenhouse Beds",
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
                        "empty".to_owned()
                    } else if state.ready {
                        if state.mutation_note.is_empty() {
                            ui_format("overlay_planter_ready", &[("item", data.item_name(&state.planted_item_id))])
                        } else {
                            format!(
                                "ready: {} ({})",
                                data.item_name(&state.planted_item_id),
                                state.mutation_note
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
                .unwrap_or("empty".to_owned());
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
                "No greenhouse beds are available yet.",
                x + 20.0,
                bed_y,
                20.0,
                dark::TEXT_DIM,
            );
        }
    }

    fn draw_journal_rapport_tab(&self, data: &GameData, x: f32, y: f32, w: f32, h: f32) {
        draw_text(ui_copy("overlay_town_rapport"), x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
        let mut rapport_y = y + 168.0;
        for npc in &data.npcs {
            let rapport = self.progression.relationships.get(&npc.id).copied().unwrap_or_default();
            draw_text(
                &format!(
                    "{} [{}] rapport {}",
                    npc.name,
                    if npc.role.is_empty() {
                        "townsfolk"
                    } else {
                        npc.role.as_str()
                    },
                    rapport
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
                &ui_format("overlay_later", &[("text", &self.npc_later_hint(data, npc))]),
                x + 20.0,
                rapport_y,
                17.0,
                dark::TEXT_DIM,
            );
            rapport_y += 18.0;
            draw_wrapped_text(
                &ui_format("overlay_usually", &[("text", &self.npc_usual_hint(data, npc))]),
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
        draw_panel(x, y, w, h, "Town Requests");
        draw_overlay_subtitle(x, y, &ui_text().overlays.quest_board_subtitle);
        let available = self.available_board_quests(data);
        let mut row_y = y + 92.0;
        if available.is_empty() {
            draw_state_banner(
                x + 20.0,
                row_y - 16.0,
                w - 40.0,
                &self.unavailable_state_text("No new board requests."),
                false,
            );
        } else {
            for (index, quest_id) in available.iter().enumerate() {
                let selected = index == self.ui.shop_index;
                if let Some(quest) = data.quest(quest_id) {
                    let giver_hint = self.quest_location_hint(data, quest);
                    draw_selection_card(
                        x + 20.0,
                        row_y - 24.0,
                        w - 40.0,
                        58.0,
                        selected,
                        true,
                        &quest.title,
                        &giver_hint,
                        &ui_format("overlay_reward", &[("coins", &quest.reward_coins.to_string())]),
                    );
                }
                row_y += 64.0;
            }
        }
        draw_text(
            "Locked Requests",
            x + 20.0,
            y + h - 200.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        let locked = data
            .quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| !self.quest_is_available(quest))
            .collect::<Vec<_>>();
        let locked_text = if locked.is_empty() {
            "None".to_owned()
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
            x + 20.0,
            y + h - 172.0,
            w - 40.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            "Active Orders",
            x + 20.0,
            y + h - 120.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        let active_orders = self
            .progression
            .started_quests
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .map(|quest| quest.title.clone())
            .collect::<Vec<_>>();
        let active_text = if active_orders.is_empty() {
            "None".to_owned()
        } else {
            active_orders.join(", ")
        };
        draw_wrapped_text(
            &active_text,
            x + 20.0,
            y + h - 90.0,
            w - 40.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &format!(
                "{} select  |  {} confirm  |  {} close",
                input_bindings().navigation.select,
                input_bindings().global.confirm,
                input_bindings().global.cancel,
            ),
        );
    }

    pub(super) fn draw_alchemy_overlay(&self, data: &GameData, art: &ArtAssets) {
        draw_overlay_backdrop();
        let x = 80.0;
        let y = 64.0;
        let w = screen_width() - 160.0;
        let h = screen_height() - 128.0;
        draw_panel(x, y, w, h, "Alchemy");
        draw_overlay_subtitle(x, y, &ui_text().overlays.alchemy_subtitle);
        if let Some(texture) = art.effect("brew_bubble_effect") {
            let alpha = 0.55 + ((get_time() as f32 * 2.4).sin() * 0.5 + 0.5) * 0.25;
            draw_texture_centered(texture, vec2(x + w - 54.0, y + 44.0), vec2(42.0, 42.0), Color::new(1.0, 1.0, 1.0, alpha));
        }

        let items = self.alchemy_materials(data);
        draw_text(ui_copy("overlay_materials"), x + 20.0, y + 84.0, 28.0, dark::TEXT_BRIGHT);
        draw_text(
            &ui_format("overlay_sort_mode", &[("mode", self.inventory_sort_label())]),
            x + 182.0,
            y + 84.0,
            18.0,
            dark::TEXT_DIM,
        );
        let mut iy = y + 82.0;
        if items.is_empty() {
            draw_state_banner(
                x + 18.0,
                iy - 12.0,
                286.0,
                &self.unavailable_state_text("No ingredients prepared."),
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
                    x + 18.0,
                    iy - 24.0,
                    286.0,
                    52.0,
                    selected,
                    ready > 0,
                    data.item_name(item_id),
                    subtitle,
                    &self.item_card_meta(
                        data,
                        item_id,
                        amount,
                        &format!(
                            "ready {}  reserved {}  {}",
                            ready,
                            self.reserved_count(item_id),
                            self.inventory_reference_summary(data, item_id)
                        ),
                    ),
                );
                iy += 58.0;
            }
        }

        draw_text("Controls", x + 20.0, y + 270.0, 24.0, dark::TEXT_BRIGHT);
        draw_wrapped_text(
            &format!(
                "{} browse  {} adjust heat  S stir  T timing",
                input_bindings().navigation.select,
                input_bindings().alchemy.heat,
            ),
            x + 20.0,
            y + 298.0,
            286.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &format!(
                "{} fill slots  {} catalyst  Enter or B brew  Y repeat  V sort  C clear",
                input_bindings().alchemy.fill_slots,
                input_bindings().alchemy.catalyst,
            ),
            x + 20.0,
            y + 334.0,
            286.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );

        draw_text(ui_copy("overlay_slots"), x + 340.0, y + 84.0, 28.0, dark::TEXT_BRIGHT);
        draw_text(
            &format!(
                "Heat {}  Stirs {}  Timing {}",
                self.alchemy.heat,
                self.alchemy.stirs,
                self.alchemy_timing()
            ),
            x + 340.0,
            y + 106.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_rectangle(x + 520.0, y + 88.0, 28.0, 24.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 520.0, y + 88.0, 28.0, 24.0, 2.0, dark::ACCENT);
        draw_text("-", x + 530.0, y + 106.0, 22.0, dark::TEXT_BRIGHT);
        draw_rectangle(x + 552.0, y + 88.0, 28.0, 24.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 552.0, y + 88.0, 28.0, 24.0, 2.0, dark::ACCENT);
        draw_text("+", x + 560.0, y + 106.0, 22.0, dark::TEXT_BRIGHT);
        draw_rectangle(x + 612.0, y + 88.0, 92.0, 24.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 612.0, y + 88.0, 92.0, 24.0, 2.0, dark::ACCENT);
        draw_text(ui_copy("overlay_alchemy_stir_button"), x + 624.0, y + 106.0, 18.0, dark::TEXT_BRIGHT);
        draw_rectangle(x + 716.0, y + 88.0, 156.0, 24.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 716.0, y + 88.0, 156.0, 24.0, 2.0, dark::ACCENT);
        draw_text(ui_copy("overlay_alchemy_timing_button"), x + 734.0, y + 106.0, 18.0, dark::TEXT_BRIGHT);
        for slot in 0..SLOT_COUNT {
            let sx = x + 340.0 + slot as f32 * 140.0;
            draw_rectangle(
                sx,
                y + 120.0,
                120.0,
                100.0,
                Color::from_rgba(38, 40, 50, 255),
            );
            draw_rectangle_lines(sx, y + 120.0, 120.0, 100.0, 2.0, dark::ACCENT);
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
                    .unwrap_or("Empty"),
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
            Color::from_rgba(38, 40, 50, 255),
        );
        draw_rectangle_lines(x + 760.0, y + 120.0, 160.0, 100.0, 2.0, dark::ACCENT);
        draw_text(ui_copy("overlay_catalyst"), x + 776.0, y + 146.0, 22.0, dark::TEXT_BRIGHT);
        draw_text(
            self.alchemy.catalyst
                .as_deref()
                .map(|id| data.item_name(id))
                .unwrap_or("Empty"),
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

        draw_text(ui_copy("overlay_preview"), x + 340.0, y + 240.0, 28.0, dark::TEXT_BRIGHT);
        draw_rectangle(
            x + 340.0,
            y + 256.0,
            w - 360.0,
            210.0,
            Color::from_rgba(38, 40, 50, 255),
        );
        draw_rectangle_lines(x + 340.0, y + 256.0, w - 360.0, 210.0, 2.0, dark::ACCENT);
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
                "Unknown salvage".to_owned()
            } else if known && stable_preview && !preview_uncertain {
                ui_format("overlay_known_result", &[("item", data.item_name(&preview.output_item_id))])
            } else if !known {
                "Unlogged formula".to_owned()
            } else if preview_uncertain {
                "Known base, uncertain branch".to_owned()
            } else if !preview.process_match {
                "Known formula, unstable process".to_owned()
            } else if !preview.minimum_elements_met {
                "Known formula, element shortfall".to_owned()
            } else if !preview.minimum_quality_met {
                "Known formula, quality shortfall".to_owned()
            } else {
                "Known formula, imperfect setup".to_owned()
            };
            draw_text(
                &preview_title,
                x + 360.0,
                y + 296.0,
                24.0,
                dark::TEXT_BRIGHT,
            );
            draw_text(
                &format!(
                    "Output: {} x{}",
                    data.item_name(&preview.output_item_id),
                    preview.output_amount
                ),
                x + 360.0,
                y + 326.0,
                22.0,
                dark::TEXT,
            );
            draw_text(
                &format!(
                    "Quality forecast: {} ({})  Mastery: {}",
                    preview.quality_score, preview.quality_band, preview.mastery_stage
                ),
                x + 360.0,
                y + 348.0,
                18.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &format!(
                    "Traits: {}",
                    if preview.inherited_traits.is_empty() {
                        "none".to_owned()
                    } else {
                        preview.inherited_traits.join(", ")
                    }
                ),
                x + 360.0,
                y + 370.0,
                18.0,
                dark::TEXT_DIM,
            );
            draw_text(
                &format!(
                    "Read: {}",
                    if preview.recipe.is_none() {
                        "fallback salvage"
                    } else if !known {
                        "formula matches, but not yet logged"
                    } else if stable_preview {
                        "stable result if brewed now"
                    } else {
                        "result is known, but the current setup will degrade it"
                    }
                ),
                x + 360.0,
                y + 392.0,
                18.0,
                dark::TEXT_DIM,
            );
            let mut process_y = y + 416.0;
            if let Some(recipe) = preview.recipe {
                draw_text(
                    &format!(
                        "Needs heat {} stir {} timing {}. Process: {} / quality: {} / elements: {}.",
                        recipe.required_heat,
                        recipe.required_stirs,
                        if recipe.required_timing.is_empty() {
                            "any"
                        } else {
                            recipe.required_timing.as_str()
                        },
                        if preview.process_match { "stable" } else { "unstable" },
                        if preview.minimum_quality_met { "pass" } else { "fail" },
                        if preview.minimum_elements_met { "pass" } else { "fail" }
                    ),
                    x + 360.0,
                    process_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                process_y += 22.0;
                draw_text(
                    &format!(
                        "Catalyst: {}  Timing: {}  Sequence: {}  Room bonus: {}",
                        if preview.catalyst_match {
                            "match"
                        } else {
                            "miss"
                        },
                        if preview.timing_match {
                            "match"
                        } else {
                            "miss"
                        },
                        if preview.sequence_match {
                            "match"
                        } else {
                            "miss"
                        },
                        if preview.room_bonus_applied {
                            "active"
                        } else {
                            "inactive"
                        }
                    ),
                    x + 360.0,
                    process_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                process_y += 24.0;
                if !preview.failure_reasons.is_empty() {
                    draw_text("Instability points:", x + 360.0, process_y, 18.0, dark::TEXT_BRIGHT);
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
                        format!(
                            "Morph ready: {} will branch into {}.",
                            recipe.name,
                            data.item_name(morph_output_item_id)
                        )
                    } else if let Some(morph_hint) = &preview.morph_hint {
                        morph_hint.clone()
                    } else if known && preview_uncertain {
                        "Known base recipe, unknown catalyst interaction.".to_owned()
                    } else if known && stable_preview {
                        recipe.description.clone()
                    } else if !preview.process_match {
                        "The ingredients are right, but the technique will distort the brew."
                            .to_owned()
                    } else if !preview.minimum_elements_met {
                        "The base formula is right, but the mixture is missing enough elemental weight."
                            .to_owned()
                    } else if !preview.minimum_quality_met {
                        "The formula is correct, but the current quality will not stabilize it."
                            .to_owned()
                    } else {
                        "The ingredients resonate, but the formula is not yet proven.".to_owned()
                    }
                })
                .unwrap_or("These ingredients will collapse into murky sludge.".to_owned());
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

        draw_text(
            "Known Formulae",
            x + 20.0,
            y + 392.0,
            28.0,
            dark::TEXT_BRIGHT,
        );
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
                "No formulae recorded yet.",
                x + 20.0,
                ky,
                20.0,
                dark::TEXT_DIM,
            );
        }

        draw_rectangle(x + 20.0, y + 368.0, 82.0, 28.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 20.0, y + 368.0, 82.0, 28.0, 2.0, dark::ACCENT);
        draw_text(ui_copy("overlay_alchemy_sort_button"), x + 44.0, y + 388.0, 18.0, dark::TEXT_BRIGHT);
        draw_rectangle(x + 114.0, y + 368.0, 82.0, 28.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 114.0, y + 368.0, 82.0, 28.0, 2.0, dark::ACCENT);
        draw_text(ui_copy("overlay_alchemy_clear_button"), x + 136.0, y + 388.0, 18.0, dark::TEXT_BRIGHT);
        draw_rectangle(x + 208.0, y + 368.0, 90.0, 28.0, Color::from_rgba(52, 56, 70, 255));
        draw_rectangle_lines(x + 208.0, y + 368.0, 90.0, 28.0, 2.0, dark::ACCENT);
        draw_text(ui_copy("overlay_alchemy_repeat_button"), x + 224.0, y + 388.0, 18.0, dark::TEXT_BRIGHT);
        draw_rectangle(x + 310.0, y + 368.0, 90.0, 28.0, Color::from_rgba(82, 110, 82, 255));
        draw_rectangle_lines(x + 310.0, y + 368.0, 90.0, 28.0, 2.0, Color::from_rgba(188, 255, 220, 255));
        draw_text(ui_copy("overlay_alchemy_brew_button"), x + 338.0, y + 388.0, 18.0, dark::TEXT_BRIGHT);

        draw_overlay_footer(
            x,
            y,
            w,
            h,
            ui_copy("overlay_alchemy_mouse_footer"),
        );
    }
}


