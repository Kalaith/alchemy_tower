use super::gameplay_support::planter_stage_label;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
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
        let mut state =
            self.progression
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
            state.planted_item_id.clear();
            state.ready = false;
            state.growth_days = 0;
            state.tended_day = 0;
            state.mutation_formula_id.clear();
            state.mutation_yield_bonus = 0;
            state.mutation_growth_bonus_days = 0;
            state.mutation_note.clear();
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }
        if !state.planted_item_id.is_empty() {
            if state.tended_day != self.world.day_index {
                state.tended_day = self.world.day_index;
                state.growth_days += 1;
                let mutation_text =
                    self.apply_planter_mutation(data, &mut state, mutation_candidate.as_ref());
                let growth_target = station
                    .planter_harvest_days
                    .max(1)
                    .saturating_sub(state.mutation_growth_bonus_days)
                    .max(1);
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
                                (
                                    "stage",
                                    planter_stage_label(state.growth_days, growth_target),
                                ),
                                ("mutation", &text),
                            ],
                        )
                    } else {
                        ui_format(
                            "gameplay_planter_tended",
                            &[
                                ("station", &station.name),
                                (
                                    "stage",
                                    planter_stage_label(state.growth_days, growth_target),
                                ),
                            ],
                        )
                    };
                }
            } else {
                let growth_target = station
                    .planter_harvest_days
                    .max(1)
                    .saturating_sub(state.mutation_growth_bonus_days)
                    .max(1);
                let days_left = growth_target.saturating_sub(state.growth_days);
                self.runtime.status_text = ui_format(
                    "gameplay_planter_status",
                    &[
                        ("station", &station.name),
                        (
                            "stage",
                            planter_stage_label(state.growth_days, growth_target),
                        ),
                        ("days", &days_left.to_string()),
                    ],
                );
            }
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }

        let Some(item_id) = candidate else {
            self.runtime.status_text = if station.planter_seed_ids.is_empty() {
                ui_copy("gameplay_planter_need_rare").to_owned()
            } else {
                ui_format(
                    "gameplay_planter_accepts",
                    &[
                        ("station", &station.name),
                        (
                            "items",
                            &station
                                .planter_seed_ids
                                .iter()
                                .map(|item_id| data.item_name(item_id))
                                .collect::<Vec<_>>()
                                .join(", "),
                        ),
                    ],
                )
            };
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        };
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
