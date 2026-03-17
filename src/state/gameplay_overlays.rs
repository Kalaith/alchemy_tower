use super::*;

impl GameplayState {
    pub(super) fn draw_shop_overlay(&self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            return;
        };
        self.draw_overlay_backdrop();
        let x = 160.0;
        let y = 88.0;
        let w = screen_width() - 320.0;
        let h = screen_height() - 176.0;
        draw_panel(x, y, w, h, &station.name);
        self.draw_overlay_subtitle(
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
        if entries.is_empty() {
            let empty_text = if self.ui.shop_buy_tab {
                self.unavailable_state_text("Nothing is stocked right now.")
            } else {
                self.unavailable_state_text("Nothing safe to sell right now.")
            };
            self.draw_state_banner(x + 20.0, row_y - 16.0, w - 40.0, &empty_text, false);
        } else {
            for (index, (item_id, price, enabled)) in entries.iter().enumerate() {
                let selected = index == self.ui.shop_index;
                let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                let subtitle = data
                    .item(item_id)
                    .map(|item| item.description.as_str())
                    .unwrap_or("");
                let detail = if self.ui.shop_buy_tab {
                    subtitle.to_owned()
                } else {
                    let context = self.inventory_reference_summary(data, item_id);
                    if context.is_empty() {
                        subtitle.to_owned()
                    } else {
                        format!("{subtitle}  {context}")
                    }
                };
                let meta = if self.ui.shop_buy_tab {
                    self.item_card_meta(data, item_id, amount, &format!("buy {}c", price))
                } else {
                    let sell_tag = if self.sell_is_safe(data, item_id) {
                        format!("sell {}c  safe", price)
                    } else {
                        format!("sell {}c", price)
                    };
                    self.item_card_meta(data, item_id, amount, &sell_tag)
                };
                self.draw_selection_card(
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
        self.draw_overlay_footer(
            x,
            y,
            w,
            h,
            "Left/Right switch tab  |  Up/Down select  |  Enter confirm  |  Esc close",
        );
    }

    pub(super) fn draw_rune_overlay(&self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            return;
        };
        self.draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, &station.name);
        self.draw_overlay_subtitle(x, y, &ui_text().overlays.rune_subtitle);
        let recipes = self.available_rune_recipes(data, station);
        let mut row_y = y + 96.0;
        if recipes.is_empty() {
            self.draw_state_banner(
                x + 20.0,
                row_y - 16.0,
                w - 40.0,
                &self.unavailable_state_text("Bring a potion and a matching rune here."),
                false,
            );
        } else {
            for (index, recipe) in recipes.iter().enumerate() {
                let selected = index == self.ui.rune_index;
                self.draw_selection_card(
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
                    &format!("rune {}", data.item_name(&recipe.rune_item_id)),
                );
                row_y += 64.0;
            }
        }
        self.draw_overlay_footer(x, y, w, h, "Up/Down select  |  Enter confirm  |  Esc close");
    }

    pub(super) fn draw_archive_overlay(&self, data: &GameData) {
        self.draw_overlay_backdrop();
        let x = 150.0;
        let y = 70.0;
        let w = screen_width() - 300.0;
        let h = screen_height() - 140.0;
        draw_panel(x, y, w, h, "Archives");
        self.draw_overlay_subtitle(x, y, &ui_text().overlays.archive_subtitle);
        self.draw_archive_tabs(x, y, w);

        match ARCHIVE_TABS[self.ui.archive_tab] {
            "Timeline" => self.draw_archive_timeline_section(x, y, w, h),
            "Experiments" => self.draw_archive_experiments_section(data, x, y, w, h),
            "Mastery" => self.draw_archive_mastery_section(data, x, y, w, h),
            "Morphs" => self.draw_archive_morphs_section(data, x, y, w, h),
            "Disassembly" => self.draw_archive_disassembly_section(data, x, y, w, h),
            _ => self.draw_archive_duplication_section(data, x, y, w, h),
        }
        self.draw_overlay_footer(x, y, w, h, &self.archive_footer_text());
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

        draw_text("Tower Status", x + 500.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let status_lines = [
            format!("Brews completed: {}", self.progression.total_brews),
            format!("Known recipes: {}", self.progression.known_recipes.len()),
            format!("Recorded experiments: {}", self.progression.experiment_log.len()),
            format!("Unlocked routes: {}", self.progression.unlocked_warps.len()),
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
        self.draw_state_banner(
            x + 500.0,
            y + h - 120.0,
            w - 520.0,
            &reconstruction,
            !self.can_reconstruct_archive(),
        );
    }

    fn draw_archive_experiments_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text("Experiment History", x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        if self.progression.experiment_log.is_empty() {
            self.draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &self.unavailable_state_text("No experiments logged yet."),
                false,
            );
            return;
        }

        let mut list_y = y + 154.0;
        for (index, entry) in self.progression.experiment_log.iter().rev().take(6).enumerate() {
            let selected = index == self.ui.archive_index;
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
            self.draw_selection_card(
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

        let selected_entry = self
            .progression
            .experiment_log
            .iter()
            .rev()
            .nth(self.ui.archive_index)
            .unwrap_or_else(|| self.progression.experiment_log.last().expect("experiment log not empty"));
        draw_text("Selected Record", x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        draw_text(
            &format!("Output: {}", data.item_name(&selected_entry.output_item_id)),
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
    }

    fn draw_archive_mastery_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text("Recipe Mastery", x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let recipes = self.mastery_recipes(data);
        if recipes.is_empty() {
            self.draw_state_banner(
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
            self.draw_selection_card(
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
        draw_text("Mastery Detail", x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
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
        draw_wrapped_text(
            &recipe.lore_note,
            x + 410.0,
            y + 270.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
    }

    fn draw_archive_morphs_section(&self, data: &GameData, x: f32, y: f32, w: f32, _h: f32) {
        draw_text("Morph Previews", x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let recipes = self.morph_recipes(data);
        if recipes.is_empty() {
            self.draw_state_banner(
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
            self.draw_selection_card(
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
        draw_text("Branch Detail", x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let mut detail_y = y + 154.0;
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
                format!("quality {}", morph.minimum_quality),
                format!("heat {}", morph.required_heat),
                format!("stirs {}", morph.required_stirs),
                if morph.catalyst_tag.is_empty() {
                    "catalyst any".to_owned()
                } else {
                    format!("catalyst {}", morph.catalyst_tag)
                },
                if morph.required_timing.is_empty() {
                    "timing any".to_owned()
                } else {
                    format!("timing {}", morph.required_timing)
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
        draw_text("Disassembly", x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let recipes = self.available_disassembly_recipes(data);
        if recipes.is_empty() {
            self.draw_state_banner(
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
            self.draw_selection_card(
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
        draw_text("Recovered Inputs", x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
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
        draw_text("Duplication", x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        let items = self.duplication_candidates(data);
        if items.is_empty() {
            self.draw_state_banner(
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
            self.draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                index == selected_index,
                self.can_duplicate_item(data, item_id),
                data.item_name(item_id),
                &item.description,
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
        draw_text("Duplication Cost", x + 410.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
        draw_text(
            &format!("Target: {}", item.name),
            x + 410.0,
            y + 156.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &format!("Coins: {}", item.base_value + u32::from(item.rarity) * 10),
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
            "Experiments" | "Mastery" | "Morphs" => {
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
            &format!("Now: {}", self.npc_now_hint(data, npc)),
            x + 20.0,
            y + 34.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &format!("Later: {}", self.npc_later_hint(data, npc)),
            x + 20.0,
            y + 54.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &format!("Usually: {}", self.npc_usual_hint(data, npc)),
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

    pub(super) fn draw_field_journal(&self, data: &GameData) {
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
                rect.x + 12.0,
                rect.y + 20.0,
                18.0,
                if selected {
                    dark::TEXT_BRIGHT
                } else {
                    dark::TEXT_DIM
                },
            );
        }

        match tabs.get(self.ui.journal_tab).copied().unwrap_or("Routes") {
            "Routes" => self.draw_journal_routes_tab(data, x, y, w, h),
            "Notes" => self.draw_journal_notes_tab(x, y, w, h),
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

    fn draw_journal_routes_tab(&self, data: &GameData, x: f32, y: f32, _w: f32, h: f32) {
        draw_text("Known Routes", x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
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
            return;
        }
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
                    "{} / {} / {} / {}",
                    route_name, entry.season, entry.weather, entry.time_window
                ),
                x + 420.0,
                entry_y,
                18.0,
                dark::TEXT_DIM,
            );
            entry_y += 20.0;
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
            if entry_y > y + h - 40.0 {
                break;
            }
        }
    }

    fn draw_journal_notes_tab(&self, x: f32, y: f32, w: f32, h: f32) {
        draw_text("Tower Notes", x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
        let mut note_y = y + 168.0;
        for milestone in self.progression.journal_milestones.iter().rev().take(10) {
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
        draw_text("Brew Ledger", x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
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
                            format!("ready: {}", data.item_name(&state.planted_item_id))
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
        draw_text("Town Rapport", x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
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
                &format!("Now: {}", self.npc_now_hint(data, npc)),
                x + 20.0,
                rapport_y,
                17.0,
                dark::TEXT_DIM,
            );
            rapport_y += 18.0;
            draw_text(
                &format!("Later: {}", self.npc_later_hint(data, npc)),
                x + 20.0,
                rapport_y,
                17.0,
                dark::TEXT_DIM,
            );
            rapport_y += 18.0;
            draw_wrapped_text(
                &format!("Usually: {}", self.npc_usual_hint(data, npc)),
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
        self.draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, "Town Requests");
        self.draw_overlay_subtitle(x, y, &ui_text().overlays.quest_board_subtitle);
        let available = self.available_board_quests(data);
        let mut row_y = y + 92.0;
        if available.is_empty() {
            self.draw_state_banner(
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
                    self.draw_selection_card(
                        x + 20.0,
                        row_y - 24.0,
                        w - 40.0,
                        58.0,
                        selected,
                        true,
                        &quest.title,
                        &giver_hint,
                        &format!("reward {}c", quest.reward_coins),
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
        self.draw_overlay_footer(x, y, w, h, "Up/Down select  |  Enter confirm  |  Esc close");
    }

    pub(super) fn draw_alchemy_overlay(&self, data: &GameData) {
        self.draw_overlay_backdrop();
        let x = 80.0;
        let y = 64.0;
        let w = screen_width() - 160.0;
        let h = screen_height() - 128.0;
        draw_panel(x, y, w, h, "Alchemy");
        self.draw_overlay_subtitle(x, y, &ui_text().overlays.alchemy_subtitle);

        let items = self.alchemy_materials(data);
        draw_text("Materials", x + 20.0, y + 84.0, 28.0, dark::TEXT_BRIGHT);
        let mut iy = y + 82.0;
        if items.is_empty() {
            self.draw_state_banner(
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
                self.draw_selection_card(
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
                            "ready {}  {}",
                            ready,
                            self.inventory_reference_summary(data, item_id)
                        ),
                    ),
                );
                iy += 58.0;
            }
        }

        draw_text("Slots", x + 340.0, y + 84.0, 28.0, dark::TEXT_BRIGHT);
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
                &format!("Slot {}", slot + 1),
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
        }
        draw_rectangle(
            x + 760.0,
            y + 120.0,
            160.0,
            100.0,
            Color::from_rgba(38, 40, 50, 255),
        );
        draw_rectangle_lines(x + 760.0, y + 120.0, 160.0, 100.0, 2.0, dark::ACCENT);
        draw_text("Catalyst", x + 776.0, y + 146.0, 22.0, dark::TEXT_BRIGHT);
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

        draw_text("Preview", x + 340.0, y + 240.0, 28.0, dark::TEXT_BRIGHT);
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
                "Add ingredients to the cauldron.",
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
            let preview_title = if known && !preview_uncertain {
                format!("Known result: {}", data.item_name(&preview.output_item_id))
            } else {
                "Uncertain result".to_owned()
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
                    y + 392.0,
                    18.0,
                    dark::TEXT_DIM,
                );
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
                    y + 414.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                if !preview.failure_reasons.is_empty() {
                    draw_wrapped_text(
                        &format!("Risk: {}", preview.failure_reasons.join(" ")),
                        x + 360.0,
                        y + 436.0,
                        w - 392.0,
                        18.0,
                        20.0,
                        dark::TEXT_DIM,
                    );
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
                    } else if known && preview.process_match && preview.minimum_quality_met {
                        recipe.description.clone()
                    } else if !preview.process_match {
                        "The ingredients are right, but the technique will distort the brew."
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
                y + 484.0
            } else {
                y + 438.0
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
            y + 330.0,
            28.0,
            dark::TEXT_BRIGHT,
        );
        let mut ky = y + 362.0;
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

        self.draw_overlay_footer(
            x,
            y,
            w,
            h,
            "Up/Down select  |  Left/Right heat  |  1/2/3 fill  F catalyst  |  Enter confirm  |  Esc close",
        );
    }
}


