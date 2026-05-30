use super::gameplay_support::planter_stage_label;
use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, MutationFormulaDefinition, PlanterStateEntry, StationDefinition};

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
        self.runtime.status_text = if mutation_note.is_empty() {
            ui_format(
                "gameplay_planter_harvested",
                &[
                    ("item", data.item_name(&state.planted_item_id)),
                    ("amount", &harvest_amount.to_string()),
                    ("station", &station.name),
                ],
            )
        } else {
            ui_format(
                "gameplay_planter_harvested_mutation",
                &[
                    ("item", data.item_name(&state.planted_item_id)),
                    ("amount", &harvest_amount.to_string()),
                    ("station", &station.name),
                    ("mutation", &mutation_note),
                ],
            )
        };
        reset_planter_after_harvest(state);
    }

    pub(super) fn tend_or_report_planter(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        state: &mut PlanterStateEntry,
        mutation_candidate: Option<&MutationFormulaDefinition>,
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
            self.runtime.status_text = if let Some(text) = mutation_text {
                ui_format(
                    "gameplay_planter_ripeness_mutation",
                    &[("station", &station.name), ("mutation", &text)],
                )
            } else {
                ui_format("gameplay_tending_ripeness", &[("station", &station.name)])
            };
        } else {
            self.runtime.status_text = if let Some(text) = mutation_text {
                ui_format(
                    "gameplay_planter_tended_mutation",
                    &[
                        ("station", &station.name),
                        ("stage", planter_stage_label(state.growth_days, growth_target)),
                        ("mutation", &text),
                    ],
                )
            } else {
                ui_format(
                    "gameplay_planter_tended",
                    &[
                        ("station", &station.name),
                        ("stage", planter_stage_label(state.growth_days, growth_target)),
                    ],
                )
            };
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
        self.runtime.status_text = ui_format(
            "gameplay_planted",
            &[
                ("item", data.item_name(&item_id)),
                ("station", &station.name),
            ],
        );
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
