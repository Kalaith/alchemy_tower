use super::{GameplayState, InventorySortMode};
use crate::content::ui_format;
use crate::data::{GameData, ItemCategory};
use std::cmp::Ordering;

impl GameplayState {
    pub(super) fn active_quest_reference_count(&self, data: &GameData, item_id: &str) -> usize {
        self.progression
            .started_quests
            .iter()
            .filter(|quest_id| {
                data.quest(quest_id)
                    .map(|quest| quest.required_item_id == item_id)
                    .unwrap_or(false)
            })
            .count()
    }

    pub(super) fn known_recipe_reference_count(&self, data: &GameData, item_id: &str) -> usize {
        data.recipes
            .iter()
            .filter(|recipe| self.progression.known_recipes.contains(&recipe.id))
            .filter(|recipe| {
                recipe.output_item_id == item_id
                    || recipe
                        .ingredients
                        .iter()
                        .any(|ingredient| ingredient.item_id == item_id)
            })
            .count()
    }

    pub(super) fn item_best_record_label(&self, item_id: &str) -> Option<String> {
        self.progression
            .crafted_item_profiles
            .get(item_id)
            .map(|profile| ui_format("inventory_best", &[("band", &profile.best_quality_band)]))
            .or_else(|| {
                self.progression
                    .herb_memories
                    .get(item_id)
                    .map(|entry| ui_format("inventory_best", &[("band", &entry.best_quality_band)]))
            })
    }

    pub(super) fn sell_is_safe(&self, data: &GameData, item_id: &str) -> bool {
        let quest_refs = self.active_quest_reference_count(data, item_id);
        let recipe_refs = self.known_recipe_reference_count(data, item_id);
        let category = data
            .item(item_id)
            .map(|item| item.category)
            .unwrap_or(ItemCategory::Rune);
        quest_refs == 0 && recipe_refs == 0 && category != ItemCategory::Potion
    }

    pub(super) fn inventory_badges(&self, data: &GameData, item_id: &str) -> Vec<String> {
        let quest_refs = self.active_quest_reference_count(data, item_id);
        let recipe_refs = self.known_recipe_reference_count(data, item_id);
        let mut badges = Vec::new();
        if quest_refs > 0 {
            badges.push(ui_format("inventory_badge_quest", &[]));
        }
        if recipe_refs > 0 {
            badges.push(ui_format("inventory_badge_recipe", &[]));
        }
        if self.item_best_record_label(item_id).is_some() {
            badges.push(ui_format("inventory_badge_best", &[]));
        }
        if self.sell_is_safe(data, item_id) {
            badges.push(ui_format("inventory_badge_safe", &[]));
        }
        badges
    }

    pub(super) fn inventory_reference_summary(&self, data: &GameData, item_id: &str) -> String {
        let quest_refs = self.active_quest_reference_count(data, item_id);
        let recipe_refs = self.known_recipe_reference_count(data, item_id);
        let mut parts = Vec::new();
        if quest_refs > 0 {
            parts.push(ui_format(
                "inventory_ref_quest",
                &[("count", &quest_refs.to_string())],
            ));
        }
        if recipe_refs > 0 {
            parts.push(ui_format(
                "inventory_ref_recipe",
                &[("count", &recipe_refs.to_string())],
            ));
        }
        if let Some(best_label) = self.item_best_record_label(item_id) {
            parts.push(best_label);
        }
        let reserved = self.reserved_count(item_id);
        if reserved > 0 {
            parts.push(ui_format(
                "inventory_ref_reserved",
                &[("count", &reserved.to_string())],
            ));
        }
        if self.sell_is_safe(data, item_id) {
            parts.push(ui_format("inventory_ref_safe", &[]));
        }
        let badges = self.inventory_badges(data, item_id);
        if !badges.is_empty() {
            parts.push(format!(
                "[{}]",
                badges
                    .iter()
                    .map(|badge| badge.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        parts.join("  ")
    }

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
            .filter(|item_id| {
                !self.progression.started_quests.iter().any(|quest_id| {
                    data.quest(quest_id)
                        .map(|quest| quest.required_item_id == *item_id)
                        .unwrap_or(false)
                })
            })
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, true);
        items
    }

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
