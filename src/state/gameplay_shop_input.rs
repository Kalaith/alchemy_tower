use super::GameplayState;
use crate::data::{GameData, StationKind};
use crate::input::{
    confirm_pressed, select_next_pressed, select_previous_pressed, sort_pressed,
    switch_next_pressed, switch_previous_pressed,
};

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

        if switch_previous_pressed() || switch_next_pressed() {
            self.ui.shop_buy_tab = !self.ui.shop_buy_tab;
            self.ui.shop_index = 0;
        }
        if select_previous_pressed() {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if select_next_pressed() {
            let max_index = if self.ui.shop_buy_tab {
                station.stock.len().saturating_sub(1)
            } else {
                self.sell_candidates(data).len().saturating_sub(1)
            };
            self.ui.shop_index = (self.ui.shop_index + 1).min(max_index);
        }
        if sort_pressed() {
            self.cycle_inventory_sort_mode();
            self.ui.shop_index = 0;
        }
        if confirm_pressed() {
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
