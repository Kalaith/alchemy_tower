use super::GameplayState;
use crate::data::GameData;

#[path = "gameplay_inventory_transaction_text.rs"]
mod transaction_text;

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
        self.runtime.status_text = transaction_text::potion_used(&item.name);
    }

    pub(super) fn buy_item(&mut self, data: &GameData, item_id: &str, price: u32) {
        if self.coins < price {
            self.runtime.status_text = transaction_text::not_enough_coins(data, item_id);
            return;
        }
        self.coins -= price;
        *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        self.note_inventory_observation(data, item_id);
        self.runtime.status_text = transaction_text::bought(data, item_id);
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
        self.runtime.status_text = transaction_text::sold(&item.name, price);
        if self.sell_is_safe(data, item_id) {
            self.trigger_safe_sale_feedback(transaction_text::sold_safe(&item.name, price));
        }
    }
}
