use crate::content::{ui_format, ui_text};

pub(super) fn closed_alchemy() -> String {
    ui_text().statuses.closed_alchemy.clone()
}

pub(super) fn removed_catalyst() -> String {
    ui_format("alchemy_removed_catalyst", &[])
}

pub(super) fn stirred(stir_count: u32) -> String {
    ui_format("alchemy_stirred", &[("count", &stir_count.to_string())])
}

pub(super) fn timing_set(timing: &str) -> String {
    ui_format("alchemy_timing_set", &[("timing", timing)])
}

pub(super) fn cleared() -> String {
    ui_format("alchemy_cleared", &[])
}
