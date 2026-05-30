use super::gameplay_planter_actions::planter_growth_target;
use super::gameplay_support::planter_stage_label;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, PlanterStateEntry, StationDefinition};

impl GameplayState {
    pub(super) fn planter_prompt_text(&self, station: &StationDefinition) -> String {
        self.progression
            .planter_states
            .get(&station.id)
            .map(|planter| {
                if planter.ready {
                    ui_copy("world_prompt_planter_harvest").to_owned()
                } else if planter.planted_item_id.is_empty() {
                    ui_copy("world_prompt_planter_plant").to_owned()
                } else if planter.tended_day != self.world.day_index {
                    ui_copy("world_prompt_planter_tend").to_owned()
                } else {
                    ui_copy("world_prompt_planter_check").to_owned()
                }
            })
            .unwrap_or_else(|| ui_copy("world_prompt_planter_plant").to_owned())
    }

    pub(super) fn report_missing_planter_seed(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
    ) {
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
    }

    pub(super) fn report_planter_status(
        &mut self,
        station: &StationDefinition,
        state: &PlanterStateEntry,
    ) {
        let growth_target = planter_growth_target(station, state);
        let days_left = growth_target.saturating_sub(state.growth_days);
        self.runtime.status_text = ui_format(
            "gameplay_planter_status",
            &[
                ("station", &station.name),
                ("stage", planter_stage_label(state.growth_days, growth_target)),
                ("days", &days_left.to_string()),
            ],
        );
    }
}
