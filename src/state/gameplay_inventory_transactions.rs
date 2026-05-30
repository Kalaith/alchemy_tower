use super::GameplayState;
use crate::content::ui_format;
use crate::data::GameData;
use macroquad::prelude::Color;

impl GameplayState {
    pub(super) fn consume_potion(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            return;
        };
        let Some(amount) = self.inventory.get_mut(item_id) else {
            return;
        };
        if *amount == 0 {
            return;
        }
        *amount -= 1;
        if *amount == 0 {
            self.inventory.remove(item_id);
        }
        for effect in &item.effects {
            self.apply_effect(effect);
        }
        self.runtime.status_text = ui_format("inventory_used", &[("name", &item.name)]);
    }

    pub(super) fn buy_item(&mut self, data: &GameData, item_id: &str, price: u32) {
        if self.coins < price {
            self.runtime.status_text = ui_format(
                "inventory_not_enough_coins",
                &[("item", data.item_name(item_id))],
            );
            return;
        }
        self.coins -= price;
        *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        self.note_inventory_observation(data, item_id);
        self.runtime.status_text =
            ui_format("inventory_bought", &[("item", data.item_name(item_id))]);
    }

    pub(super) fn sell_item(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            return;
        };
        let price = self.sell_price(data, item_id);
        let Some(amount) = self.inventory.get_mut(item_id) else {
            return;
        };
        if *amount == 0 {
            return;
        }
        *amount -= 1;
        if *amount == 0 {
            self.inventory.remove(item_id);
        }
        self.coins += price;
        self.runtime.status_text = ui_format(
            "inventory_sold",
            &[("name", &item.name), ("price", &price.to_string())],
        );
        if self.sell_is_safe(data, item_id) {
            self.push_event_toast_with_icon(
                ui_format(
                    "inventory_sold_safe",
                    &[("name", &item.name), ("price", &price.to_string())],
                ),
                Color::from_rgba(255, 214, 132, 255),
                "best_quality",
            );
        }
    }
}
