use super::*;
use crate::content::ui_format;
use crate::data::ItemCategory;

#[allow(dead_code)]
impl GameplayState {
    pub(super) fn archive_experiment_entries<'a>(
        &'a self,
        data: &'a GameData,
    ) -> Vec<&'a ExperimentLogEntry> {
        let _ = data;
        self.progression
            .experiment_log
            .iter()
            .rev()
            .filter(|entry| match self.ui.archive_experiment_filter {
                ArchiveExperimentFilter::All => true,
                ArchiveExperimentFilter::Stable => entry.stable,
                ArchiveExperimentFilter::Unstable => !entry.stable,
            })
            .collect()
    }

    pub(super) fn archive_experiment_filter_label(&self) -> &'static str {
        self.ui.archive_experiment_filter.label()
    }

    pub(super) fn cycle_archive_experiment_filter(&mut self) {
        self.ui.archive_experiment_filter = self.ui.archive_experiment_filter.next();
        self.ui.archive_index = 0;
        self.runtime.status_text = ui_format(
            "archive_filter_status",
            &[("mode", self.archive_experiment_filter_label())],
        );
    }

    pub(super) fn mastery_recipes<'a>(&self, data: &'a GameData) -> Vec<&'a crate::data::RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| self.progression.known_recipes.contains(&recipe.id))
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| {
            self.recipe_mastery_brews(&right.id)
                .cmp(&self.recipe_mastery_brews(&left.id))
                .then(left.name.cmp(&right.name))
        });
        recipes
    }

    pub(super) fn morph_recipes<'a>(&self, data: &'a GameData) -> Vec<&'a crate::data::RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| !recipe.morph_targets.is_empty())
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| left.name.cmp(&right.name));
        recipes
    }

    pub(super) fn archive_selection_len(&self, data: &GameData) -> usize {
        match ARCHIVE_TABS[self.ui.archive_tab] {
            "Experiments" => self.archive_experiment_entries(data).len(),
            "Mastery" => self.mastery_recipes(data).len(),
            "Morphs" => self.morph_recipes(data).len(),
            "Disassembly" => self.available_disassembly_recipes(data).len(),
            "Duplication" => self.duplication_candidates(data).len(),
            _ => 0,
        }
    }

    pub(super) fn available_disassembly_recipes<'a>(
        &self,
        data: &'a GameData,
    ) -> Vec<&'a crate::data::RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| self.progression.known_recipes.contains(&recipe.id))
            .filter(|recipe| {
                self.inventory
                    .get(&recipe.output_item_id)
                    .copied()
                    .unwrap_or_default()
                    > 0
            })
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| left.name.cmp(&right.name));
        recipes
    }

    pub(super) fn disassemble_recipe(
        &mut self,
        data: &GameData,
        recipe: &crate::data::RecipeDefinition,
    ) {
        let Some(output_amount) = self.inventory.get_mut(&recipe.output_item_id) else {
            self.runtime.status_text = ui_format("progression_no_disassemble", &[("name", &recipe.name)]);
            return;
        };
        if *output_amount == 0 {
            self.runtime.status_text = ui_format("progression_no_disassemble", &[("name", &recipe.name)]);
            return;
        }

        *output_amount -= 1;
        if *output_amount == 0 {
            self.inventory.remove(&recipe.output_item_id);
        }

        let mut returned = Vec::new();
        for ingredient in &recipe.ingredients {
            *self.inventory.entry(ingredient.item_id.clone()).or_insert(0) += ingredient.amount;
            self.note_inventory_observation(data, &ingredient.item_id);
            returned.push(format!(
                "{} x{}",
                data.item_name(&ingredient.item_id),
                ingredient.amount
            ));
        }

        self.push_event_toast_with_icon(
            ui_format("progression_disassembly_toast", &[("name", &recipe.name)]),
            Color::from_rgba(214, 204, 170, 255),
            "recipe_logged",
        );
        self.runtime.status_text =
            ui_format("progression_disassembled", &[("name", &recipe.name), ("items", &returned.join(", "))]);
    }

    pub(super) fn duplication_candidates(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(duplication_item_allowed)
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            let left_cost = data
                .item(left)
                .map(duplication_cost)
                .unwrap_or(u32::MAX);
            let right_cost = data
                .item(right)
                .map(duplication_cost)
                .unwrap_or(u32::MAX);
            left_cost
                .cmp(&right_cost)
                .then(data.item_name(left).cmp(data.item_name(right)))
        });
        items
    }

    pub(super) fn can_duplicate_item(&self, data: &GameData, item_id: &str) -> bool {
        let Some(item) = data.item(item_id) else {
            return false;
        };
        duplication_item_allowed(item)
            && self.inventory.get(item_id).copied().unwrap_or_default() > 0
            && self.coins >= duplication_cost(item)
            && self.duplication_catalyst_item_id(data).is_some()
    }

    pub(super) fn duplicate_item(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            self.runtime.status_text = ui_format("progression_duplicate_unstable", &[]);
            return;
        };
        if !duplication_item_allowed(item) {
            self.runtime.status_text = ui_format("progression_duplicate_resists", &[("name", &item.name)]);
            return;
        }
        if self.inventory.get(item_id).copied().unwrap_or_default() == 0 {
            self.runtime.status_text = ui_format("progression_duplicate_none", &[("name", &item.name)]);
            return;
        }

        let cost = duplication_cost(item);
        if self.coins < cost {
            self.runtime.status_text = ui_format(
                "progression_duplicate_need_coins",
                &[("coins", &cost.saturating_sub(self.coins).to_string()), ("name", &item.name)],
            );
            return;
        }

        let Some(catalyst_item_id) = self.duplication_catalyst_item_id(data) else {
            self.runtime.status_text = ui_format("progression_duplicate_need_catalyst", &[]);
            return;
        };

        self.coins = self.coins.saturating_sub(cost);
        if let Some(amount) = self.inventory.get_mut(&catalyst_item_id) {
            *amount = amount.saturating_sub(1);
            if *amount == 0 {
                self.inventory.remove(&catalyst_item_id);
            }
        }
        *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        self.note_inventory_observation(data, item_id);

        self.push_event_toast_with_icon(
            ui_format("progression_duplicate_toast", &[("name", &item.name)]),
            Color::from_rgba(216, 182, 255, 255),
            "best_quality",
        );
        self.runtime.status_text = ui_format(
            "progression_duplicate_status",
            &[
                ("name", &item.name),
                ("cost", &cost.to_string()),
                ("catalyst", data.item_name(&catalyst_item_id)),
            ],
        );
    }

    pub(super) fn duplication_catalyst_item_id(&self, data: &GameData) -> Option<String> {
        self.inventory
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .filter_map(|(item_id, _)| {
                let item = data.item(item_id)?;
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == "starlight")
                    .then_some((item_id.clone(), item.quality))
            })
            .max_by(|left, right| left.1.cmp(&right.1).then(left.0.cmp(&right.0)))
            .map(|entry| entry.0)
    }

    pub(super) fn planter_mutation_candidate(
        &self,
        data: &GameData,
        planted_item_id: &str,
    ) -> Option<(String, String)> {
        for formula in data.mutation_formulas_for_seed(planted_item_id) {
            for (item_id, amount) in &self.inventory {
                if *amount == 0 || item_id == planted_item_id {
                    continue;
                }
                let Some(item) = data.item(item_id) else {
                    continue;
                };
                if item.category != ItemCategory::Potion {
                    continue;
                }
                let effect_match = formula.required_effect_kind.is_empty()
                    || item
                        .effects
                        .iter()
                        .any(|effect| effect.kind.as_str() == formula.required_effect_kind);
                if effect_match {
                    return Some((formula.id.clone(), item_id.clone()));
                }
            }
        }
        None
    }

    pub(super) fn apply_planter_mutation(
        &mut self,
        data: &GameData,
        state: &mut PlanterStateEntry,
        candidate: Option<&(String, String)>,
    ) -> Option<String> {
        if !state.mutation_formula_id.is_empty() {
            return None;
        }
        let Some((formula_id, catalyst_item_id)) = candidate else {
            return None;
        };
        let formula = data
            .mutation_formulas
            .iter()
            .find(|formula| formula.id == *formula_id)?;

        let amount = self.inventory.get_mut(catalyst_item_id)?;
        *amount = amount.saturating_sub(1);
        if *amount == 0 {
            self.inventory.remove(catalyst_item_id);
        }

        state.mutation_formula_id = formula.id.clone();
        state.mutation_yield_bonus = formula.yield_bonus;
        state.mutation_growth_bonus_days = formula.growth_bonus_days;
        state.mutation_note = formula.mutation_note.clone();

        self.push_event_toast_with_icon(
            ui_format("progression_planter_mutation", &[("item", data.item_name(&state.planted_item_id))]),
            Color::from_rgba(188, 255, 220, 255),
            "best_quality",
        );

        Some(ui_format(
            "progression_planter_mutation_status",
            &[("catalyst", data.item_name(catalyst_item_id)), ("strain", &formula.mutation_note)],
        ))
    }
}

fn duplication_item_allowed(item: &crate::data::ItemDefinition) -> bool {
    matches!(
        item.category,
        ItemCategory::Ingredient | ItemCategory::Catalyst | ItemCategory::Potion
    )
}

fn duplication_cost(item: &crate::data::ItemDefinition) -> u32 {
    item.base_value + u32::from(item.rarity) * 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disassembly_returns_recipe_inputs() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        let recipe = data
            .recipe_for_output("healing_draught")
            .expect("healing draught recipe should exist");

        state.progression.known_recipes.insert(recipe.id.clone());
        state.inventory.insert("healing_draught".to_owned(), 1);

        state.disassemble_recipe(&data, recipe);

        assert_eq!(
            state.inventory.get("healing_draught").copied().unwrap_or_default(),
            0
        );
        assert_eq!(state.inventory.get("sunleaf").copied().unwrap_or_default(), 1);
        assert_eq!(
            state.inventory.get("whisper_moss").copied().unwrap_or_default(),
            1
        );
    }

    #[test]
    fn duplication_consumes_catalyst_and_coins() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);

        state.coins = 99;
        state.inventory.insert("glow_potion".to_owned(), 1);
        state.inventory.insert("starlight_shard".to_owned(), 1);

        state.duplicate_item(&data, "glow_potion");

        assert_eq!(state.inventory.get("glow_potion").copied().unwrap_or_default(), 2);
        assert_eq!(
            state.inventory.get("starlight_shard").copied().unwrap_or_default(),
            0
        );
        assert_eq!(state.coins, 63);
    }

    #[test]
    fn planter_mutation_consumes_matching_potion_and_stores_bonus() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        let mut planter = PlanterStateEntry {
            station_id: "greenhouse_planter_east".to_owned(),
            planted_item_id: "moon_fern".to_owned(),
            planted_day: 0,
            ready: false,
            tended_day: 0,
            growth_days: 0,
            mutation_formula_id: String::new(),
            mutation_yield_bonus: 0,
            mutation_growth_bonus_days: 0,
            mutation_note: String::new(),
        };

        state.inventory.insert("glow_potion".to_owned(), 1);

        let candidate = state.planter_mutation_candidate(&data, "moon_fern");
        let text = state.apply_planter_mutation(&data, &mut planter, candidate.as_ref());

        assert!(text.is_some());
        assert_eq!(planter.mutation_formula_id, "moon_fern_glow_mutation");
        assert_eq!(planter.mutation_yield_bonus, 1);
        assert_eq!(planter.mutation_growth_bonus_days, 1);
        assert_eq!(state.inventory.get("glow_potion").copied().unwrap_or_default(), 0);
    }
}

