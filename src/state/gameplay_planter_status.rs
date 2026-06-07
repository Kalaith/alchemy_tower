use super::gameplay_planter_actions::planter_growth_target;
use super::GameplayState;
use crate::data::{GameData, PlanterStateEntry, StationDefinition};

#[path = "gameplay_planter_report_text.rs"]
mod report_text;

impl GameplayState {
    pub(super) fn planter_prompt_text(&self, station: &StationDefinition) -> String {
        self.progression
            .planter_states
            .get(&station.id)
            .map(|planter| {
                if planter.ready {
                    self.interact_prompt_copy("world_prompt_planter_harvest", &[])
                } else if planter.planted_item_id.is_empty() {
                    self.interact_prompt_copy("world_prompt_planter_plant", &[])
                } else if planter.tended_day != self.world.day_index {
                    self.interact_prompt_copy("world_prompt_planter_tend", &[])
                } else {
                    self.interact_prompt_copy("world_prompt_planter_check", &[])
                }
            })
            .unwrap_or_else(|| self.interact_prompt_copy("world_prompt_planter_plant", &[]))
    }

    pub(super) fn report_missing_planter_seed(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
    ) {
        self.runtime.status_text = report_text::missing_seed(data, station);
    }

    pub(super) fn report_planter_status(
        &mut self,
        station: &StationDefinition,
        state: &PlanterStateEntry,
    ) {
        let growth_target = planter_growth_target(station, state);
        let days_left = growth_target.saturating_sub(state.growth_days);
        self.runtime.status_text =
            report_text::planter_status(station, state, growth_target, days_left);
    }
}
