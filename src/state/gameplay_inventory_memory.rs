use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, ItemCategory, PotionMemoryEntry, RecipeDefinition};

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
            .or_insert_with(|| PotionMemoryEntry {
                item_id: item_id.to_owned(),
                first_seen_day: self.world.day_index,
                seen: true,
                learned: false,
                learned_day: 0,
                successful_brews: 0,
                best_quality_score: 0,
                best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
                last_recipe_id: String::new(),
            });
        if !entry.seen {
            entry.seen = true;
            entry.first_seen_day = self.world.day_index;
        }
    }

    pub(super) fn ensure_potion_memory_learned(&mut self, item_id: &str, recipe_id: Option<&str>) {
        self.ensure_potion_memory_seen(item_id);
        let entry = self
            .progression
            .potion_memories
            .get_mut(item_id)
            .expect("potion memory should exist after ensure");
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
        let entry = self
            .progression
            .potion_memories
            .get_mut(item_id)
            .expect("potion memory should exist after learn");
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

    pub(super) fn rebuild_memory_state(&mut self, data: &GameData) {
        let inventory_items = self.inventory.keys().cloned().collect::<Vec<_>>();
        for item_id in inventory_items {
            self.note_inventory_observation(data, &item_id);
        }

        if self.progression.potion_memories.is_empty() {
            for entry in self.progression.experiment_log.clone() {
                if data
                    .item(&entry.output_item_id)
                    .map(|item| item.category == ItemCategory::Potion)
                    .unwrap_or(false)
                {
                    self.record_potion_result_memory(
                        &entry.output_item_id,
                        (!entry.recipe_id.is_empty()).then_some(entry.recipe_id.as_str()),
                        entry.stable,
                        entry.quality_score,
                        &entry.quality_band,
                    );
                }
            }
        }

        for recipe in &data.recipes {
            if self.progression.known_recipes.contains(&recipe.id) {
                self.ensure_potion_memory_learned(&recipe.output_item_id, Some(&recipe.id));
            }
        }

        let crafted_profiles = self
            .progression
            .crafted_item_profiles
            .values()
            .cloned()
            .collect::<Vec<_>>();
        for profile in crafted_profiles {
            if data
                .item(&profile.item_id)
                .map(|item| item.category == ItemCategory::Potion)
                .unwrap_or(false)
            {
                self.ensure_potion_memory_learned(&profile.item_id, None);
                if let Some(memory) = self.progression.potion_memories.get_mut(&profile.item_id) {
                    if profile.best_quality_score >= memory.best_quality_score {
                        memory.best_quality_score = profile.best_quality_score;
                        memory.best_quality_band = profile.best_quality_band.clone();
                    }
                }
            }
        }
    }

    pub(super) fn recipe_memory_meta(&self, _data: &GameData, recipe: &RecipeDefinition) -> String {
        let mastery = self.recipe_mastery_brews(&recipe.id);
        let best = self
            .progression
            .crafted_item_profiles
            .get(&recipe.output_item_id)
            .map(|profile| profile.best_quality_band.clone())
            .unwrap_or_else(|| ui_format("inventory_best_unlogged", &[]));
        let catalyst = if recipe.catalyst_tag.is_empty() {
            ui_format("inventory_catalyst_any", &[])
        } else {
            ui_format(
                "inventory_catalyst_specific",
                &[("tag", &recipe.catalyst_tag)],
            )
        };
        ui_format(
            "inventory_memory_meta",
            &[
                ("mastery", &mastery.to_string()),
                ("best", &best),
                ("catalyst", &catalyst),
            ],
        )
    }

    pub(super) fn recipe_memory_detail(
        &self,
        data: &GameData,
        recipe: &RecipeDefinition,
    ) -> String {
        let mut parts = vec![ui_format(
            "inventory_memory_output",
            &[("item", data.item_name(&recipe.output_item_id))],
        )];
        if !recipe.required_sequence.is_empty() {
            let sequence = recipe
                .required_sequence
                .iter()
                .map(|item_id| data.item_name(item_id))
                .collect::<Vec<_>>()
                .join(" -> ");
            parts.push(ui_format(
                "inventory_memory_order",
                &[("sequence", &sequence)],
            ));
        }
        if let Some(profile) = self
            .progression
            .crafted_item_profiles
            .get(&recipe.output_item_id)
        {
            if !profile.inherited_traits.is_empty() {
                parts.push(ui_format(
                    "inventory_memory_traits",
                    &[("traits", &profile.inherited_traits.join(", "))],
                ));
            }
        }
        if !recipe.morph_targets.is_empty() {
            parts.push(ui_format("inventory_memory_morph", &[]));
        }
        parts.join("  ")
    }
}
