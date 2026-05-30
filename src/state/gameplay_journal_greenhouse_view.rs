use super::gameplay_support::planter_stage_label;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, StationKind};
use crate::view_models::journal::{JournalGreenhouseBedView, JournalGreenhouseTabView};

impl GameplayState {
    pub(super) fn journal_greenhouse_tab_view(&self, data: &GameData) -> JournalGreenhouseTabView {
        let beds = self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.kind == StationKind::Planter)
            .map(|station| JournalGreenhouseBedView {
                title: station.name.clone(),
                summary: self.journal_planter_summary(data, station),
            })
            .collect();

        JournalGreenhouseTabView {
            empty_text: ui_copy("overlay_greenhouse_empty").to_owned(),
            beds,
        }
    }

    fn journal_planter_summary(
        &self,
        data: &GameData,
        station: &crate::data::StationDefinition,
    ) -> String {
        self.progression
            .planter_states
            .get(&station.id)
            .map(|state| {
                if state.planted_item_id.is_empty() {
                    ui_copy("overlay_greenhouse_none").to_owned()
                } else if state.ready {
                    if state.mutation_note.is_empty() {
                        ui_format(
                            "overlay_planter_ready",
                            &[("item", data.item_name(&state.planted_item_id))],
                        )
                    } else {
                        ui_format(
                            "overlay_greenhouse_ready_meta",
                            &[
                                ("item", data.item_name(&state.planted_item_id)),
                                ("mutation", &state.mutation_note),
                            ],
                        )
                    }
                } else {
                    let growth_target = station
                        .planter_harvest_days
                        .max(1)
                        .saturating_sub(state.mutation_growth_bonus_days)
                        .max(1);
                    if state.mutation_note.is_empty() {
                        format!(
                            "{} ({})",
                            data.item_name(&state.planted_item_id),
                            planter_stage_label(state.growth_days, growth_target)
                        )
                    } else {
                        format!(
                            "{} ({}, {})",
                            data.item_name(&state.planted_item_id),
                            planter_stage_label(state.growth_days, growth_target),
                            state.mutation_note
                        )
                    }
                }
            })
            .unwrap_or_else(|| ui_copy("overlay_greenhouse_none").to_owned())
    }
}
