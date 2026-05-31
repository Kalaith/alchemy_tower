use crate::content::{ui_format, ui_text};

pub(super) fn talking_to(npc_name: &str) -> String {
    ui_format("gameplay_talking_to", &[("name", npc_name)])
}

pub(super) fn open_alchemy() -> String {
    ui_text().statuses.open_alchemy.clone()
}

pub(super) fn route_restored(warp_label: &str) -> String {
    ui_format("gameplay_route_restored", &[("label", warp_label)])
}

pub(super) fn repaired_access(warp_label: &str) -> String {
    ui_format("gameplay_repaired_access", &[("label", warp_label)])
}
