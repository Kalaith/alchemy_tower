use super::GameplayState;
use crate::data::{GameData, PlanterStateEntry, StationDefinition};

#[path = "gameplay_planter_status_text.rs"]
mod status_text;

impl GameplayState {
    pub(super) fn harvest_planter(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        state: &mut PlanterStateEntry,
    ) {
        let harvest_amount = 2 + station.planter_yield_bonus + state.mutation_yield_bonus;
        let mutation_note = state.mutation_note.clone();
        *self
            .inventory
            .entry(state.planted_item_id.clone())
            .or_insert(0) += harvest_amount;
        self.note_inventory_observation(data, &state.planted_item_id);
        self.runtime.status_text =
            status_text::harvested(data, station, state, harvest_amount, &mutation_note);
        reset_planter_after_harvest(state);
    }

    pub(super) fn tend_or_report_planter(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        state: &mut PlanterStateEntry,
        mutation_candidate: Option<&(String, String)>,
    ) {
        if state.tended_day == self.world.day_index {
            self.report_planter_status(station, state);
            return;
        }

        state.tended_day = self.world.day_index;
        state.growth_days += 1;
        let mutation_text = self.apply_planter_mutation(data, state, mutation_candidate);
        let growth_target = planter_growth_target(station, state);
        if state.growth_days >= growth_target {
            state.ready = true;
            self.runtime.status_text = status_text::ripeness(station, mutation_text.as_deref());
        } else {
            self.runtime.status_text =
                status_text::tended(station, state, growth_target, mutation_text.as_deref());
        }
    }

    pub(super) fn plant_seed_in_planter(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        state: &mut PlanterStateEntry,
        item_id: String,
    ) {
        if let Some(amount) = self.inventory.get_mut(&item_id) {
            *amount -= 1;
        }
        self.inventory.retain(|_, amount| *amount > 0);
        state.planted_item_id = item_id.clone();
        state.planted_day = self.world.day_index;
        state.ready = false;
        state.tended_day = 0;
        state.growth_days = 0;
        state.mutation_formula_id.clear();
        state.mutation_yield_bonus = 0;
        state.mutation_growth_bonus_days = 0;
        state.mutation_note.clear();
        self.runtime.status_text = status_text::planted(data, station, &item_id);
    }
}

pub(super) fn planter_growth_target(station: &StationDefinition, state: &PlanterStateEntry) -> u32 {
    station
        .planter_harvest_days
        .max(1)
        .saturating_sub(state.mutation_growth_bonus_days)
        .max(1)
}

fn reset_planter_after_harvest(state: &mut PlanterStateEntry) {
    state.planted_item_id.clear();
    state.ready = false;
    state.growth_days = 0;
    state.tended_day = 0;
    state.mutation_formula_id.clear();
    state.mutation_yield_bonus = 0;
    state.mutation_growth_bonus_days = 0;
    state.mutation_note.clear();
}
