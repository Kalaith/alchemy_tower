use crate::content::ui_format;
use crate::data::{GameData, StationDefinition};

pub(super) fn accepts(data: &GameData, station: &StationDefinition) -> String {
    ui_format(
        "gameplay_habitat_accepts",
        &[
            ("station", &station.name),
            (
                "items",
                &station
                    .habitat_creature_ids
                    .iter()
                    .map(|item_id| data.item_name(item_id))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        ],
    )
}

pub(super) fn settled(data: &GameData, station: &StationDefinition, creature_id: &str) -> String {
    ui_format(
        "gameplay_habitat_settled",
        &[
            ("item", data.item_name(creature_id)),
            ("station", &station.name),
        ],
    )
}

pub(super) fn collected(
    data: &GameData,
    station_name: &str,
    output_item_id: &str,
    amount: u32,
) -> String {
    ui_format(
        "gameplay_habitat_collected",
        &[
            ("item", data.item_name(output_item_id)),
            ("amount", &amount.to_string()),
            ("station", station_name),
        ],
    )
}

pub(super) fn waiting(
    data: &GameData,
    creature_item_id: &str,
    output_item_id: &str,
    days_left: u32,
) -> String {
    ui_format(
        "gameplay_habitat_waiting",
        &[
            ("creature", data.item_name(creature_item_id)),
            ("days", &days_left.to_string()),
            ("output", data.item_name(output_item_id)),
        ],
    )
}
