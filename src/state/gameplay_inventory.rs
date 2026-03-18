use super::*;
use crate::content::ui_format;
use crate::data::{ItemCategory, RecipeDefinition};
use std::collections::BTreeMap;

impl GameplayState {
    pub(super) fn brew_is_stable(&self, resolution: &crate::alchemy::BrewResolution<'_>) -> bool {
        resolution.process_match
            && resolution.minimum_quality_met
            && resolution.minimum_elements_met
    }

    pub(super) fn inventory_sort_label(&self) -> &'static str {
        self.ui.inventory_sort_mode.label()
    }

    pub(super) fn cycle_inventory_sort_mode(&mut self) {
        self.ui.inventory_sort_mode = self.ui.inventory_sort_mode.next();
        self.runtime.status_text = ui_format(
            "inventory_sort_status",
            &[("mode", self.inventory_sort_label())],
        );
    }

    pub(super) fn active_quest_reference_count(&self, data: &GameData, item_id: &str) -> usize {
        self.progression.started_quests
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
        self.progression.crafted_item_profiles
            .get(item_id)
            .map(|profile| ui_format("inventory_best", &[("band", &profile.best_quality_band)]))
            .or_else(|| {
                self.progression.field_journal
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
            parts.push(ui_format("inventory_ref_quest", &[("count", &quest_refs.to_string())]));
        }
        if recipe_refs > 0 {
            parts.push(ui_format("inventory_ref_recipe", &[("count", &recipe_refs.to_string())]));
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
            ui_format("inventory_catalyst_specific", &[("tag", &recipe.catalyst_tag)])
        };
        ui_format(
            "inventory_memory_meta",
            &[("mastery", &mastery.to_string()), ("best", &best), ("catalyst", &catalyst)],
        )
    }

    pub(super) fn recipe_memory_detail(
        &self,
        data: &GameData,
        recipe: &RecipeDefinition,
    ) -> String {
        let mut parts = vec![ui_format("inventory_memory_output", &[("item", data.item_name(&recipe.output_item_id))])];
        if !recipe.required_sequence.is_empty() {
            let sequence = recipe
                .required_sequence
                .iter()
                .map(|item_id| data.item_name(item_id))
                .collect::<Vec<_>>()
                .join(" -> ");
            parts.push(ui_format("inventory_memory_order", &[("sequence", &sequence)]));
        }
        if let Some(profile) = self.progression.crafted_item_profiles.get(&recipe.output_item_id) {
            if !profile.inherited_traits.is_empty() {
                parts.push(ui_format("inventory_memory_traits", &[("traits", &profile.inherited_traits.join(", "))]));
            }
        }
        if !recipe.morph_targets.is_empty() {
            parts.push(ui_format("inventory_memory_morph", &[]));
        }
        parts.join("  ")
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

    pub(super) fn alchemy_materials(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| {
                            item.category == ItemCategory::Ingredient
                                || item.category == ItemCategory::Catalyst
                        })
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, false);
        items
    }

    pub(super) fn reserved_count(&self, item_id: &str) -> u32 {
        self.alchemy.slots
            .iter()
            .filter(|slot| slot.as_deref() == Some(item_id))
            .count() as u32
            + u32::from(self.alchemy.catalyst.as_deref() == Some(item_id))
    }

    pub(super) fn fill_slot(&mut self, data: &GameData, items: &[String], slot: usize) {
        let Some(item_id) = items.get(self.alchemy.index) else {
            return;
        };
        let Some(item) = data.item(item_id) else {
            return;
        };
        if item.category != ItemCategory::Ingredient {
            self.runtime.status_text = self.unavailable_state_text(&format!(
                "{}",
                ui_format("inventory_fill_slot_catalyst", &[("name", &item.name)])
            ));
            return;
        }
        let total = self.inventory.get(item_id).copied().unwrap_or_default();
        let reserved = self.reserved_count(item_id)
            - u32::from(self.alchemy.slots[slot].as_deref() == Some(item_id));
        if total <= reserved {
            self.runtime.status_text = self.unavailable_state_text(&format!(
                "{}",
                ui_format("inventory_no_more_ready", &[("name", data.item_name(item_id))])
            ));
            return;
        }
        self.alchemy.slots[slot] = Some(item_id.clone());
        self.runtime.status_text = ui_format(
            "inventory_added_slot",
            &[("item", data.item_name(item_id)), ("slot", &(slot + 1).to_string())],
        );
    }

    pub(super) fn fill_catalyst(&mut self, data: &GameData, items: &[String]) {
        let Some(item_id) = items.get(self.alchemy.index) else {
            return;
        };
        let Some(item) = data.item(item_id) else {
            return;
        };
        if item.category != ItemCategory::Catalyst {
            self.runtime.status_text = self.unavailable_state_text(&format!(
                "{}",
                ui_format("inventory_fill_catalyst_invalid", &[("name", &item.name)])
            ));
            return;
        }
        let total = self.inventory.get(item_id).copied().unwrap_or_default();
        let reserved = self.reserved_count(item_id)
            - u32::from(self.alchemy.catalyst.as_deref() == Some(item_id));
        if total <= reserved {
            self.runtime.status_text =
                self.unavailable_state_text(&ui_format("inventory_no_more_ready", &[("name", &item.name)]));
            return;
        }
        self.alchemy.catalyst = Some(item_id.clone());
        self.runtime.status_text = ui_format("inventory_prepared_catalyst", &[("name", &item.name)]);
    }

    pub(super) fn selected_items(&self) -> Vec<String> {
        self.alchemy.slots
            .iter()
            .filter_map(|item_id| item_id.clone())
            .collect()
    }

    pub(super) fn selected_catalyst(&self) -> Option<&str> {
        self.alchemy.catalyst.as_deref()
    }

    pub(super) fn alchemy_timing(&self) -> &'static str {
        ALCHEMY_TIMINGS[self.alchemy.timing_index]
    }

    pub(super) fn save_last_brew_setup(&mut self) {
        self.runtime.last_brew_setup = Some(SavedAlchemySetup {
            heat: self.alchemy.heat,
            stirs: self.alchemy.stirs,
            timing_index: self.alchemy.timing_index,
            slots: self.alchemy.slots.clone(),
            catalyst: self.alchemy.catalyst.clone(),
        });
    }

    pub(super) fn repeat_last_brew_setup(&mut self, data: &GameData) {
        let Some(setup) = self.runtime.last_brew_setup.clone() else {
            self.runtime.status_text = ui_format("alchemy_repeat_none", &[]);
            return;
        };

        let mut needed = BTreeMap::<String, u32>::new();
        for item_id in setup.slots.iter().flatten() {
            *needed.entry(item_id.clone()).or_insert(0) += 1;
        }
        if let Some(item_id) = &setup.catalyst {
            *needed.entry(item_id.clone()).or_insert(0) += 1;
        }

        for (item_id, required) in &needed {
            let available = self.inventory.get(item_id).copied().unwrap_or_default();
            if available < *required {
                self.runtime.status_text = self.unavailable_state_text(&ui_format(
                    "alchemy_repeat_missing",
                    &[("name", data.item_name(item_id)), ("count", &required.to_string())],
                ));
                return;
            }
        }

        self.alchemy.heat = setup.heat;
        self.alchemy.stirs = setup.stirs;
        self.alchemy.timing_index = setup.timing_index.min(ALCHEMY_TIMINGS.len().saturating_sub(1));
        self.alchemy.slots = setup.slots;
        self.alchemy.catalyst = setup.catalyst;
        self.runtime.status_text = ui_format("alchemy_repeat_loaded", &[]);
    }

    pub(super) fn recipe_mastery_brews(&self, recipe_id: &str) -> u32 {
        self.progression.recipe_mastery
            .get(recipe_id)
            .copied()
            .unwrap_or_default()
    }

    pub(super) fn preview_mastery_brews(
        &self,
        data: &GameData,
        station: &StationDefinition,
        selected: &[String],
    ) -> u32 {
        crate::alchemy::match_recipe(data, station, selected)
            .map(|recipe| self.recipe_mastery_brews(&recipe.id))
            .unwrap_or_default()
    }

    pub(super) fn current_item_quality_snapshot(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<(u32, String)> {
        let item = data.item(item_id)?;
        let mut quality = item.quality;
        let mut variant_name = String::new();
        for variant in &item.wild_variants {
            if variant
                .required_conditions
                .iter()
                .all(|condition| self.condition_matches(condition))
            {
                quality += variant.quality_bonus;
                variant_name = variant.name.clone();
                break;
            }
        }
        Some((quality.min(100), variant_name))
    }

    pub(super) fn condition_matches(&self, condition: &str) -> bool {
        let condition = condition.to_ascii_lowercase();
        condition.contains(self.current_season())
            || condition.contains(self.current_weather())
            || condition.contains(self.current_time_window())
    }

    pub(super) fn preview_is_uncertain(
        &self,
        preview: &crate::alchemy::BrewResolution<'_>,
    ) -> bool {
        preview
            .recipe
            .map(|recipe| !recipe.morph_targets.is_empty() && self.selected_catalyst().is_some())
            .unwrap_or(false)
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

    fn sort_item_ids(&self, data: &GameData, items: &mut [String], selling: bool) {
        items.sort_by(|left, right| self.compare_item_ids(data, left, right, selling));
    }

    fn compare_item_ids(
        &self,
        data: &GameData,
        left: &str,
        right: &str,
        selling: bool,
    ) -> std::cmp::Ordering {
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
    ) -> std::cmp::Ordering {
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
    ) -> std::cmp::Ordering {
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

    pub(super) fn brew_selected(&mut self, data: &GameData, station: &StationDefinition) {
        let selected = self.selected_items();
        if selected.is_empty() {
            self.runtime.status_text = narrative_text().statuses.cauldron_empty.clone();
            return;
        }
        self.save_last_brew_setup();
        let resolution = resolve_brew(
            data,
            station,
            &selected,
            self.selected_catalyst(),
            self.alchemy.heat,
            self.alchemy.stirs,
            self.alchemy_timing(),
            self.preview_mastery_brews(data, station, &selected),
        );
        let stable_brew = self.brew_is_stable(&resolution);
        let brew_feedback_color = if resolution.recipe.is_none() {
            Color::from_rgba(196, 162, 255, 255)
        } else if stable_brew {
            Color::from_rgba(188, 255, 220, 255)
        } else {
            Color::from_rgba(255, 214, 132, 255)
        };
        self.trigger_world_feedback(
            vec2(station.position[0], station.position[1]),
            brew_feedback_color,
            stable_brew || resolution.recipe.is_none(),
            if stable_brew { 1.9 } else { 1.4 },
        );
        self.trigger_camera_shake(if stable_brew { 0.12 } else { 0.08 }, if stable_brew { 3.6 } else { 2.0 });
        for item_id in &selected {
            if let Some(amount) = self.inventory.get_mut(item_id) {
                *amount -= 1;
            }
        }
        if let Some(item_id) = self.selected_catalyst().map(str::to_owned) {
            if let Some(amount) = self.inventory.get_mut(&item_id) {
                *amount -= 1;
            }
        }
        self.inventory.retain(|_, amount| *amount > 0);
        *self
            .inventory
            .entry(resolution.output_item_id.clone())
            .or_insert(0) += resolution.output_amount;
        self.progression.total_brews += 1;
        self.record_experiment_log(&resolution);
        let previous_profile = self
            .progression
            .crafted_item_profiles
            .get(&resolution.output_item_id)
            .cloned();
        self.record_crafted_item_profile(data, &resolution.output_item_id, &resolution);
        if let Some(recipe) = resolution.recipe {
            let previous_mastery = self.recipe_mastery_brews(&recipe.id);
            let stable_brew = self.brew_is_stable(&resolution);
            let mastery_improved = if stable_brew {
                let mastery = self.progression.recipe_mastery.entry(recipe.id.clone()).or_insert(0);
                *mastery += 1;
                *mastery > previous_mastery
            } else {
                false
            };
            let current_mastery_stage = crate::alchemy::mastery_stage(
                self.recipe_mastery_brews(&recipe.id),
            );
            let recipe_discovered = self.progression.known_recipes.insert(recipe.id.clone());
            if recipe_discovered {
                self.push_event_toast(
                    ui_format("inventory_recipe_logged", &[("name", &recipe.name)]),
                    Color::from_rgba(176, 226, 255, 255),
                );
                self.runtime.status_text = ui_format(
                    "inventory_discovered_status",
                    &[
                        ("recipe", &recipe.name),
                        ("quality", resolution.quality_band),
                        ("mastery", current_mastery_stage),
                        ("traits", &resolution.inherited_traits.join(", ")),
                    ],
                );
            } else {
                self.runtime.status_text = ui_format(
                    "inventory_brewed_status",
                    &[
                        ("item", data.item_name(&resolution.output_item_id)),
                        ("amount", &resolution.output_amount.to_string()),
                        ("quality", resolution.quality_band),
                        (
                            "result",
                            if stable_brew {
                                "Stable synthesis."
                            } else {
                                "Imperfect process."
                            },
                        ),
                        ("mastery", current_mastery_stage),
                    ],
                );
            }
            if mastery_improved {
                self.push_event_toast(
                    ui_format(
                        "inventory_mastery_improved",
                        &[("name", &recipe.name), ("stage", current_mastery_stage)],
                    ),
                    Color::from_rgba(255, 230, 170, 255),
                );
            }
        } else {
            self.runtime.status_text = ui_format(
                "inventory_collapse_status",
                &[
                    ("quality", resolution.quality_band),
                    ("item", data.item_name(&resolution.output_item_id)),
                    ("reasons", &resolution.failure_reasons.join(" ")),
                ],
            );
        }
        let current_profile = self.progression.crafted_item_profiles.get(&resolution.output_item_id);
        let improved_best = current_profile
            .zip(previous_profile.as_ref())
            .map(|(current, previous)| current.best_quality_score > previous.best_quality_score)
            .unwrap_or(current_profile.is_some());
        if improved_best {
            if let Some(profile) = current_profile {
                self.push_event_toast(
                    ui_format(
                        "inventory_new_best",
                        &[
                            ("item", data.item_name(&resolution.output_item_id)),
                            ("band", &profile.best_quality_band),
                        ],
                    ),
                    Color::from_rgba(188, 255, 220, 255),
                );
            }
        }
        if self.progression.total_brews == 10 {
            self.push_event_toast(
                &ui_format("inventory_greenhouse_unlock", &[]),
                Color::from_rgba(200, 255, 200, 255),
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(200, 255, 200, 255),
                true,
                2.1,
            );
            self.trigger_camera_shake(0.2, 5.4);
            self.runtime.status_text = narrative_text().statuses.greenhouse_unlock.clone();
        }
        self.alchemy.stirs = 0;
        self.alchemy.timing_index = 0;
        self.alchemy.slots = [None, None, None];
        self.alchemy.catalyst = None;
    }

    pub(super) fn record_crafted_item_profile(
        &mut self,
        data: &GameData,
        item_id: &str,
        resolution: &crate::alchemy::BrewResolution<'_>,
    ) {
        let effect_kinds = data
            .item(item_id)
            .map(|item| {
                item.effects
                    .iter()
                    .map(|effect| effect.kind.to_string())
                    .collect()
            })
            .unwrap_or_else(Vec::new);
        let entry = self
            .progression
            .crafted_item_profiles
            .entry(item_id.to_owned())
            .or_insert_with(|| CraftedItemProfileEntry {
                item_id: item_id.to_owned(),
                best_quality_score: 0,
                best_quality_band: "Crude".to_owned(),
                inherited_traits: Vec::new(),
                effect_kinds: effect_kinds.clone(),
            });
        if resolution.quality_score >= entry.best_quality_score {
            entry.best_quality_score = resolution.quality_score;
            entry.best_quality_band = resolution.quality_band.to_owned();
            entry.inherited_traits = resolution.inherited_traits.clone();
        }
        if entry.effect_kinds.is_empty() {
            entry.effect_kinds = effect_kinds;
        }
    }

    pub(super) fn record_experiment_log(
        &mut self,
        resolution: &crate::alchemy::BrewResolution<'_>,
    ) {
        self.progression.experiment_log.push(ExperimentLogEntry {
            recipe_id: resolution
                .recipe
                .map(|recipe| recipe.id.clone())
                .unwrap_or_default(),
            output_item_id: resolution.output_item_id.clone(),
            quality_score: resolution.quality_score,
            quality_band: resolution.quality_band.to_owned(),
            stable: resolution.process_match
                && resolution.minimum_quality_met
                && resolution.minimum_elements_met,
            catalyst_item_id: self.selected_catalyst().unwrap_or_default().to_owned(),
            morph_output_item_id: resolution.morph_output_item_id.clone().unwrap_or_default(),
            day_index: self.world.day_index,
        });
        if self.progression.experiment_log.len() > 60 {
            let excess = self.progression.experiment_log.len() - 60;
            self.progression.experiment_log.drain(0..excess);
        }
    }

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
            self.runtime.status_text =
                ui_format("inventory_not_enough_coins", &[("item", data.item_name(item_id))]);
            return;
        }
        self.coins -= price;
        *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        self.runtime.status_text = ui_format("inventory_bought", &[("item", data.item_name(item_id))]);
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
            self.push_event_toast(
                ui_format(
                    "inventory_sold_safe",
                    &[("name", &item.name), ("price", &price.to_string())],
                ),
                Color::from_rgba(255, 214, 132, 255),
            );
        }
    }
}



