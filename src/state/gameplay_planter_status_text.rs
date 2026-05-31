use super::super::gameplay_support::planter_stage_label;
use crate::content::ui_format;
use crate::data::{GameData, PlanterStateEntry, StationDefinition};

pub(super) fn harvested(
    data: &GameData,
    station: &StationDefinition,
    state: &PlanterStateEntry,
    harvest_amount: u32,
    mutation_note: &str,
) -> String {
    if mutation_note.is_empty() {
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
                ("mutation", mutation_note),
            ],
        )
    }
}

pub(super) fn ripeness(station: &StationDefinition, mutation_text: Option<&str>) -> String {
    if let Some(text) = mutation_text {
        ui_format(
            "gameplay_planter_ripeness_mutation",
            &[("station", &station.name), ("mutation", text)],
        )
    } else {
        ui_format("gameplay_tending_ripeness", &[("station", &station.name)])
    }
}

pub(super) fn tended(
    station: &StationDefinition,
    state: &PlanterStateEntry,
    growth_target: u32,
    mutation_text: Option<&str>,
) -> String {
    let stage = planter_stage_label(state.growth_days, growth_target);
    if let Some(text) = mutation_text {
        ui_format(
            "gameplay_planter_tended_mutation",
            &[
                ("station", &station.name),
                ("stage", stage),
                ("mutation", text),
            ],
        )
    } else {
        ui_format(
            "gameplay_planter_tended",
            &[("station", &station.name), ("stage", stage)],
        )
    }
}

pub(super) fn planted(
    data: &GameData,
    station: &StationDefinition,
    item_id: &str,
) -> String {
    ui_format(
        "gameplay_planted",
        &[
            ("item", data.item_name(item_id)),
            ("station", &station.name),
        ],
    )
}
