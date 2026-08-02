use crate::content::{ui_copy, ui_format};

pub(super) fn out_of(place: &str) -> String {
    ui_format("journal_formula_hint_ground", &[("place", place)])
}

pub(super) fn stocked_by(counter: &str) -> String {
    ui_format("journal_formula_hint_counter", &[("counter", counter)])
}

pub(super) fn brewed_not_picked() -> String {
    ui_copy("journal_formula_hint_bench").to_owned()
}

pub(super) fn only_the_bench(station: &str) -> String {
    ui_format("journal_formula_hint_ready", &[("station", station)])
}
