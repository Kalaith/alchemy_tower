use super::GameplayState;
use crate::data::{GameData, ItemCategory, PotionMemoryEntry};

#[path = "gameplay_potion_memory_entries.rs"]
mod potion_memory_entries;

use self::potion_memory_entries::{empty_effects_text, new_seen_potion_memory};

pub(super) struct JournalPotionProfileSummary {
    pub(super) effects_text: String,
    pub(super) traits_text: Option<String>,
}

impl GameplayState {
    pub(super) fn note_inventory_observation(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            return;
        };
        match item.category {
            ItemCategory::Ingredient => self.ensure_herb_memory_seen(item_id, None),
            ItemCategory::Potion => self.ensure_potion_memory_seen(item_id),
            _ => {}
        }
    }

    pub(super) fn ensure_potion_memory_seen(&mut self, item_id: &str) {
        let entry = self
            .progression
            .potion_memories
            .entry(item_id.to_owned())
            .or_insert_with(|| new_seen_potion_memory(item_id, self.world.day_index));
        if !entry.seen {
            entry.seen = true;
            entry.first_seen_day = self.world.day_index;
        }
    }

    pub(super) fn ensure_potion_memory_learned(&mut self, item_id: &str, recipe_id: Option<&str>) {
        self.ensure_potion_memory_seen(item_id);
        let Some(entry) = self.progression.potion_memories.get_mut(item_id) else {
            return;
        };
        if !entry.learned {
            entry.learned = true;
            entry.learned_day = self.world.day_index;
        }
        if let Some(recipe_id) = recipe_id {
            if entry.last_recipe_id.is_empty() {
                entry.last_recipe_id = recipe_id.to_owned();
            }
        }
    }

    pub(super) fn record_potion_result_memory(
        &mut self,
        item_id: &str,
        recipe_id: Option<&str>,
        stable: bool,
        quality_score: u32,
        quality_band: &str,
    ) {
        self.ensure_potion_memory_learned(item_id, recipe_id);
        let Some(entry) = self.progression.potion_memories.get_mut(item_id) else {
            return;
        };
        if stable {
            entry.successful_brews += 1;
        }
        if quality_score >= entry.best_quality_score {
            entry.best_quality_score = quality_score;
            entry.best_quality_band = quality_band.to_owned();
        }
        if let Some(recipe_id) = recipe_id {
            entry.last_recipe_id = recipe_id.to_owned();
        }
    }

    pub(super) fn potion_memories<'a>(&'a self, data: &'a GameData) -> Vec<&'a PotionMemoryEntry> {
        let mut entries = self
            .progression
            .potion_memories
            .values()
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| {
            right
                .successful_brews
                .cmp(&left.successful_brews)
                .then(right.learned.cmp(&left.learned))
                .then(right.best_quality_score.cmp(&left.best_quality_score))
                .then(
                    data.item_name(&left.item_id)
                        .cmp(data.item_name(&right.item_id)),
                )
        });
        entries
    }

    pub(super) fn journal_potion_profile_summary(
        &self,
        item_id: &str,
    ) -> Option<JournalPotionProfileSummary> {
        let profile = self.progression.crafted_item_profiles.get(item_id)?;
        let effects_text = if profile.effect_kinds.is_empty() {
            empty_effects_text()
        } else {
            profile.effect_kinds.join(", ")
        };
        let traits_text = (!profile.inherited_traits.is_empty())
            .then(|| profile.inherited_traits.join(", "));
        Some(JournalPotionProfileSummary {
            effects_text,
            traits_text,
        })
    }
}
