use super::GameplayState;
use crate::data::{GameData, ItemCategory};

impl GameplayState {
    pub(super) fn sorted_inventory_items(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, false);
        items
    }

    pub(super) fn sell_price(&self, data: &GameData, item_id: &str) -> u32 {
        let Some(item) = data.item(item_id) else {
            return 0;
        };
        if item.category == ItemCategory::Potion {
            item.base_value + (item.base_value / 4).max(1)
        } else {
            item.base_value
        }
    }

    pub(super) fn quick_potions(&self, data: &GameData) -> Vec<String> {
        let mut potions = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| item.category == ItemCategory::Potion)
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        potions.sort_by(|left, right| {
            let left_value = data.item(left).map(|item| item.base_value).unwrap_or(0);
            let right_value = data.item(right).map(|item| item.base_value).unwrap_or(0);
            right_value.cmp(&left_value).then(left.cmp(right))
        });
        potions
    }

    pub(super) fn sell_candidates(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .sorted_inventory_items(data)
            .into_iter()
            .filter(|item_id| self.active_quest_reference_count(data, item_id) == 0)
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, true);
        items
    }
}
