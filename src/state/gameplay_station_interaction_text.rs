use crate::content::{ui_copy, ui_format, ui_text};

pub(super) fn opened_station(station_name: &str) -> String {
    ui_format("gameplay_opened_station", &[("name", station_name)])
}

pub(super) fn reading_quest_board() -> String {
    ui_text().statuses.reading_quest_board.clone()
}

pub(super) fn observatory_aligned() -> String {
    ui_format("gameplay_observatory_aligned", &[])
}

pub(super) fn observatory_locked() -> String {
    ui_copy("observatory_locked").to_owned()
}
