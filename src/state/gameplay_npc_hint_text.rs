use crate::content::{ui_copy, ui_format};

pub(super) fn somewhere() -> &'static str {
    ui_copy("npc_hint_somewhere")
}

pub(super) fn unknown() -> &'static str {
    ui_copy("npc_hint_unknown")
}

pub(super) fn travelling(area_name: &str, target_name: &str) -> String {
    ui_format(
        "npc_travelling",
        &[("area", area_name), ("target", target_name)],
    )
}

pub(super) fn here_now(area_name: &str, time_window: &str) -> String {
    ui_format(
        "npc_hint_here_now",
        &[("area", area_name), ("time", time_window)],
    )
}

pub(super) fn routine_unclear() -> String {
    ui_copy("npc_hint_routine_unclear").to_owned()
}

pub(super) fn later(area_name: &str, time_window: &str) -> String {
    ui_format(
        "npc_hint_later",
        &[("area", area_name), ("time", time_window)],
    )
}

pub(super) fn usual(time_window: &str, area_name: &str) -> String {
    ui_format(
        "npc_hint_usual",
        &[("time", time_window), ("area", area_name)],
    )
}

pub(super) fn quest_location(now: &str, later: &str) -> String {
    ui_format("npc_quest_location", &[("now", now), ("later", later)])
}

pub(super) fn quest_location_fallback() -> String {
    ui_copy("npc_quest_location_fallback").to_owned()
}
