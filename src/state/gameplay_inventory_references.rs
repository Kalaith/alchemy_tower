use super::GameplayState;
use crate::data::{GameData, ItemCategory};

#[path = "gameplay_inventory_reference_text.rs"]
mod reference_text;

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
            .map(|profile| reference_text::best_record(&profile.best_quality_band))
            .or_else(|| {
                self.progression
                    .herb_memories
                    .get(item_id)
                    .map(|entry| reference_text::best_record(&entry.best_quality_band))
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
            badges.push(reference_text::quest_badge());
        }
        if recipe_refs > 0 {
            badges.push(reference_text::recipe_badge());
        }
        if self.item_best_record_label(item_id).is_some() {
            badges.push(reference_text::best_badge());
        }
        if self.sell_is_safe(data, item_id) {
            badges.push(reference_text::safe_badge());
        }
        badges
    }

    pub(super) fn inventory_reference_summary(&self, data: &GameData, item_id: &str) -> String {
        let quest_refs = self.active_quest_reference_count(data, item_id);
        let recipe_refs = self.known_recipe_reference_count(data, item_id);
        let mut parts = Vec::new();
        if quest_refs > 0 {
            parts.push(reference_text::quest_reference(quest_refs));
        }
        if recipe_refs > 0 {
            parts.push(reference_text::recipe_reference(recipe_refs));
        }
        if let Some(best_label) = self.item_best_record_label(item_id) {
            parts.push(best_label);
        }
        let reserved = self.reserved_count(item_id);
        if reserved > 0 {
            parts.push(reference_text::reserved_reference(reserved));
        }
        if self.sell_is_safe(data, item_id) {
            parts.push(reference_text::safe_reference());
        }
        let badges = self.inventory_badges(data, item_id);
        if !badges.is_empty() {
            parts.push(reference_text::badge_summary(&badges));
        }
        reference_text::reference_summary(&parts)
    }
}
