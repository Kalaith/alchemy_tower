use super::gameplay_overlay_types::InventorySortMode;
use super::GameplayState;
use crate::data::GameData;
use std::cmp::Ordering;

impl GameplayState {
    pub(super) fn sort_item_ids(&self, data: &GameData, items: &mut [String], selling: bool) {
        items.sort_by(|left, right| self.compare_item_ids(data, left, right, selling));
    }

    fn compare_item_ids(
        &self,
        data: &GameData,
        left: &str,
        right: &str,
        selling: bool,
    ) -> Ordering {
        match self.ui.inventory_sort_mode {
            InventorySortMode::Priority => self.compare_item_priority(data, left, right, selling),
            InventorySortMode::Type => self.compare_item_type(data, left, right, selling),
            InventorySortMode::Name => data.item_name(left).cmp(data.item_name(right)),
        }
    }

    fn compare_item_priority(
        &self,
        data: &GameData,
        left: &str,
        right: &str,
        selling: bool,
    ) -> Ordering {
        let left_item = data.item(left);
        let right_item = data.item(right);
        let left_quest = self.active_quest_reference_count(data, left);
        let right_quest = self.active_quest_reference_count(data, right);
        let left_recipe = self.known_recipe_reference_count(data, left);
        let right_recipe = self.known_recipe_reference_count(data, right);
        let left_best = self.item_best_record_label(left).is_some();
        let right_best = self.item_best_record_label(right).is_some();
        let left_quality = left_item.map(|item| item.quality).unwrap_or_default();
        let right_quality = right_item.map(|item| item.quality).unwrap_or_default();
        let left_category = left_item.map(|item| item.category.as_str()).unwrap_or("");
        let right_category = right_item.map(|item| item.category.as_str()).unwrap_or("");
        let left_safe = self.sell_is_safe(data, left);
        let right_safe = self.sell_is_safe(data, right);

        if selling {
            right_safe
                .cmp(&left_safe)
                .then(right_quality.cmp(&left_quality))
                .then(data.item_name(left).cmp(data.item_name(right)))
        } else {
            right_quest
                .cmp(&left_quest)
                .then(right_recipe.cmp(&left_recipe))
                .then(right_best.cmp(&left_best))
                .then(right_quality.cmp(&left_quality))
                .then(left_category.cmp(right_category))
                .then(data.item_name(left).cmp(data.item_name(right)))
        }
    }

    fn compare_item_type(
        &self,
        data: &GameData,
        left: &str,
        right: &str,
        selling: bool,
    ) -> Ordering {
        let left_item = data.item(left);
        let right_item = data.item(right);
        let left_category = left_item.map(|item| item.category.as_str()).unwrap_or("");
        let right_category = right_item.map(|item| item.category.as_str()).unwrap_or("");
        let left_safe = self.sell_is_safe(data, left);
        let right_safe = self.sell_is_safe(data, right);

        if selling {
            right_safe
                .cmp(&left_safe)
                .then(left_category.cmp(right_category))
                .then(data.item_name(left).cmp(data.item_name(right)))
        } else {
            left_category
                .cmp(right_category)
                .then(data.item_name(left).cmp(data.item_name(right)))
        }
    }
}
