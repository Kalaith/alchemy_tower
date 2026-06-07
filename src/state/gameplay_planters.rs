use super::GameplayState;
use crate::data::{GameData, ItemCategory, PlanterStateEntry, StationDefinition};

impl GameplayState {
    pub(super) fn interact_with_planter(&mut self, data: &GameData, station: &StationDefinition) {
        let existing_state = self.progression.planter_states.get(&station.id).cloned();
        let candidate = existing_state
            .as_ref()
            .filter(|state| state.planted_item_id.is_empty())
            .and_then(|_| self.planter_seed_choice(data, station));
        let mutation_candidate = existing_state.as_ref().and_then(|state| {
            (!state.planted_item_id.is_empty()
                && !state.ready
                && state.mutation_formula_id.is_empty())
            .then(|| self.planter_mutation_candidate(data, &state.planted_item_id))
            .flatten()
        });
        let mut state = self
            .progression
            .planter_states
            .remove(&station.id)
            .unwrap_or(PlanterStateEntry {
                station_id: station.id.clone(),
                planted_item_id: String::new(),
                planted_day: self.world.day_index,
                ready: false,
                tended_day: 0,
                growth_days: 0,
                mutation_formula_id: String::new(),
                mutation_yield_bonus: 0,
                mutation_growth_bonus_days: 0,
                mutation_note: String::new(),
            });
        if state.ready && !state.planted_item_id.is_empty() {
            self.harvest_planter(data, station, &mut state);
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }
        if !state.planted_item_id.is_empty() {
            self.tend_or_report_planter(data, station, &mut state, mutation_candidate.as_ref());
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }

        let Some(item_id) = candidate else {
            self.report_missing_planter_seed(data, station);
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        };
        self.plant_seed_in_planter(data, station, &mut state, item_id);
        self.progression
            .planter_states
            .insert(station.id.clone(), state);
    }

    pub(super) fn planter_seed_choice(
        &self,
        data: &GameData,
        station: &StationDefinition,
    ) -> Option<String> {
        self.inventory
            .iter()
            .find(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| {
                            item.category == ItemCategory::Ingredient
                                && item.rarity >= 2
                                && (station.planter_seed_ids.is_empty()
                                    || station.planter_seed_ids.iter().any(|seed| seed == *item_id))
                        })
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
    }
}
