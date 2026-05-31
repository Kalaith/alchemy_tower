use super::super::gameplay_support::planter_stage_label;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, PlanterStateEntry, StationDefinition};

pub(super) fn missing_seed(data: &GameData, station: &StationDefinition) -> String {
    if station.planter_seed_ids.is_empty() {
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
    }
}

pub(super) fn planter_status(
    station: &StationDefinition,
    state: &PlanterStateEntry,
    growth_target: u32,
    days_left: u32,
) -> String {
    ui_format(
        "gameplay_planter_status",
        &[
            ("station", &station.name),
            ("stage", planter_stage_label(state.growth_days, growth_target)),
            ("days", &days_left.to_string()),
        ],
    )
}
