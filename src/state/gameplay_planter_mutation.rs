use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, ItemCategory, PlanterStateEntry};
use macroquad::prelude::Color;

impl GameplayState {
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
            ui_format(
                "progression_planter_mutation",
                &[("item", data.item_name(&state.planted_item_id))],
            ),
            Color::from_rgba(188, 255, 220, 255),
            "best_quality",
        );

        Some(ui_format(
            "progression_planter_mutation_status",
            &[
                ("catalyst", data.item_name(catalyst_item_id)),
                ("strain", &formula.mutation_note),
            ],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{GameData, PlanterStateEntry};

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
        assert_eq!(
            state
                .inventory
                .get("glow_potion")
                .copied()
                .unwrap_or_default(),
            0
        );
    }
}
