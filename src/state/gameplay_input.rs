use super::*;
use crate::content::ui_format;

impl GameplayState {
    pub(super) fn handle_alchemy_inputs(&mut self, data: &GameData) {
        let Some(station) = self.nearby_station(data).cloned() else {
            self.alchemy.open = false;
            return;
        };
        if station.kind != StationKind::Alchemy {
            self.alchemy.open = false;
            return;
        }

        if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Escape) {
            self.alchemy.open = false;
            self.runtime.status_text = ui_text().statuses.closed_alchemy.clone();
            return;
        }

        let items = self.alchemy_materials(data);
        if items.is_empty() {
            return;
        }

        if is_key_pressed(KeyCode::Up) {
            self.alchemy.index = self.alchemy.index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            self.alchemy.index = (self.alchemy.index + 1).min(items.len().saturating_sub(1));
        }
        if is_key_pressed(KeyCode::Left) {
            self.alchemy.heat = (self.alchemy.heat - 1).max(1);
        }
        if is_key_pressed(KeyCode::Right) {
            self.alchemy.heat = (self.alchemy.heat + 1).min(3);
        }
        if is_key_pressed(KeyCode::S) {
            self.alchemy.stirs += 1;
            self.runtime.status_text =
                ui_format("alchemy_stirred", &[("count", &self.alchemy.stirs.to_string())]);
        }
        if is_key_pressed(KeyCode::T) {
            self.alchemy.timing_index = (self.alchemy.timing_index + 1) % ALCHEMY_TIMINGS.len();
            self.runtime.status_text =
                ui_format("alchemy_timing_set", &[("timing", self.alchemy_timing())]);
        }

        for (slot, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3]
            .iter()
            .enumerate()
        {
            if is_key_pressed(*key) {
                self.fill_slot(data, &items, slot);
            }
        }
        for (slot, key) in [KeyCode::Q, KeyCode::W, KeyCode::E].iter().enumerate() {
            if is_key_pressed(*key) {
                self.alchemy.slots[slot] = None;
            }
        }
        if is_key_pressed(KeyCode::F) {
            self.fill_catalyst(data, &items);
        }
        if is_key_pressed(KeyCode::R) {
            self.alchemy.catalyst = None;
            self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
        }
        if is_key_pressed(KeyCode::C) {
            self.alchemy.slots = [None, None, None];
            self.alchemy.catalyst = None;
            self.alchemy.stirs = 0;
            self.alchemy.timing_index = 0;
            self.runtime.status_text = ui_format("alchemy_cleared", &[]);
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::B) {
            self.brew_selected(data, &station);
        }
    }

    pub(super) fn handle_shop_inputs(&mut self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            self.ui.shop_open = false;
            return;
        };
        if station.kind != StationKind::Shop {
            self.ui.shop_open = false;
            return;
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) {
            self.ui.shop_buy_tab = !self.ui.shop_buy_tab;
            self.ui.shop_index = 0;
        }
        if is_key_pressed(KeyCode::Up) {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            let max_index = if self.ui.shop_buy_tab {
                station.stock.len().saturating_sub(1)
            } else {
                self.sell_candidates(data).len().saturating_sub(1)
            };
            self.ui.shop_index = (self.ui.shop_index + 1).min(max_index);
        }
        if is_key_pressed(KeyCode::Enter) {
            if self.ui.shop_buy_tab {
                if let Some(stock) = station.stock.get(self.ui.shop_index) {
                    self.buy_item(data, &stock.item_id, stock.price);
                }
            } else {
                let sellable = self.sell_candidates(data);
                if let Some(item_id) = sellable.get(self.ui.shop_index) {
                    self.sell_item(data, item_id);
                }
            }
        }

        let max_index = if self.ui.shop_buy_tab {
            station.stock.len().saturating_sub(1)
        } else {
            self.sell_candidates(data).len().saturating_sub(1)
        };
        self.ui.shop_index = self.ui.shop_index.min(max_index);
    }

    pub(super) fn handle_rune_inputs(&mut self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            self.ui.rune_open = false;
            return;
        };
        if station.kind != StationKind::RuneWorkshop {
            self.ui.rune_open = false;
            return;
        }
        let recipes = self.available_rune_recipes(data, station);
        if recipes.is_empty() {
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::E) {
                self.ui.rune_open = false;
            }
            return;
        }
        if is_key_pressed(KeyCode::Up) {
            self.ui.rune_index = self.ui.rune_index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            self.ui.rune_index = (self.ui.rune_index + 1).min(recipes.len().saturating_sub(1));
        }
        if is_key_pressed(KeyCode::Enter) {
            if let Some(recipe) = recipes.get(self.ui.rune_index) {
                self.apply_rune_recipe(data, recipe);
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.ui.rune_open = false;
            self.runtime.status_text = ui_text().statuses.closed_rune.clone();
        }
    }

    pub(super) fn handle_archive_inputs(&mut self, data: &GameData) {
        if is_key_pressed(KeyCode::Left) {
            self.ui.archive_tab = self.ui.archive_tab.saturating_sub(1);
            self.ui.archive_index = 0;
        }
        if is_key_pressed(KeyCode::Right) {
            self.ui.archive_tab =
                (self.ui.archive_tab + 1).min(ARCHIVE_TABS.len().saturating_sub(1));
            self.ui.archive_index = 0;
        }

        let selection_len = self.archive_selection_len(data);
        if selection_len > 0 {
            if is_key_pressed(KeyCode::Up) {
                self.ui.archive_index = self.ui.archive_index.saturating_sub(1);
            }
            if is_key_pressed(KeyCode::Down) {
                self.ui.archive_index =
                    (self.ui.archive_index + 1).min(selection_len.saturating_sub(1));
            }
        } else {
            self.ui.archive_index = 0;
        }

        if is_key_pressed(KeyCode::Enter) {
            match ARCHIVE_TABS[self.ui.archive_tab] {
                "Timeline" => {
                    if self.can_reconstruct_archive() {
                        let milestone = &narrative_text().milestones.archive_revelation;
                        self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
                        self.push_event_toast(
                            "Archive restored: timeline completed.",
                            Color::from_rgba(176, 226, 255, 255),
                        );
                        self.runtime.status_text =
                            narrative_text().statuses.archive_timeline_complete.clone();
                    } else {
                        self.runtime.status_text = narrative_text()
                            .statuses
                            .archive_timeline_incomplete
                            .clone();
                    }
                }
                "Disassembly" => {
                    let recipes = self.available_disassembly_recipes(data);
                    if let Some(recipe) = recipes.get(self.ui.archive_index).copied() {
                        self.disassemble_recipe(data, recipe);
                    }
                }
                "Duplication" => {
                    let items = self.duplication_candidates(data);
                    if let Some(item_id) = items.get(self.ui.archive_index) {
                        self.duplicate_item(data, item_id);
                    }
                }
                _ => {}
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.ui.archive_open = false;
            self.runtime.status_text = ui_text().statuses.closed_archive.clone();
        }
    }

    pub(super) fn handle_potion_inputs(&mut self, data: &GameData) {
        let potions = self.quick_potions(data);
        for (index, item_id) in potions.iter().take(3).enumerate() {
            let key = [KeyCode::Z, KeyCode::X, KeyCode::C][index];
            if is_key_pressed(key) {
                self.consume_potion(data, item_id);
                return;
            }
        }
    }
}


