use super::GameplayState;
use crate::data::{GameData, StationKind};
use macroquad::prelude::{is_key_pressed, KeyCode};

impl GameplayState {
    pub(super) fn handle_shop_inputs(&mut self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            self.clear_overlay();
            return;
        };
        if station.kind != StationKind::Shop {
            self.clear_overlay();
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
        if is_key_pressed(KeyCode::V) {
            self.cycle_inventory_sort_mode();
            self.ui.shop_index = 0;
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
}
