use crate::content::{ui_copy, ui_format};

pub(super) fn day_begin(weather: &str, season: &str) -> String {
    ui_format(
        "day_begin_status",
        &[("weather", weather), ("season", season)],
    )
}

pub(super) fn fainted_home() -> String {
    ui_copy("gameplay_fainted_home").to_owned()
}

pub(super) fn slept_until(time: &str) -> String {
    ui_format("gameplay_slept_until", &[("time", time)])
}
