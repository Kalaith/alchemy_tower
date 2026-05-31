use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, GatherNodeDefinition};

pub(super) fn attempt_status(has_field_notes: bool, node_name: &str) -> String {
    if has_field_notes {
        ui_format("gather_attempt_known", &[("name", node_name)])
    } else {
        ui_format("gather_attempt_none", &[])
    }
}

pub(super) fn journal_toast(data: &GameData, route_id: &str) -> String {
    let route_name = data
        .route(route_id)
        .map(|route| route.name.as_str())
        .unwrap_or(ui_copy("gather_fallback_notes"));
    ui_format("gather_toast_journal", &[("route", route_name)])
}

pub(super) fn quality_toast(node_name: &str) -> String {
    ui_format("gather_toast_quality", &[("name", node_name)])
}

pub(super) fn variant_toast(node_name: &str) -> String {
    ui_format("gather_toast_variant", &[("name", node_name)])
}

pub(super) fn status(
    data: &GameData,
    node: &GatherNodeDefinition,
    variant_discovered: bool,
    improved_quality: bool,
) -> String {
    let route_name = data
        .route(&node.route_id)
        .map(|route| route.name.as_str())
        .unwrap_or(ui_copy("gather_fallback_route"));
    if variant_discovered {
        ui_format(
            "gather_status_variant",
            &[("name", &node.name), ("route", route_name)],
        )
    } else if improved_quality {
        ui_format(
            "gather_status_best",
            &[("name", &node.name), ("route", route_name)],
        )
    } else {
        ui_format(
            "gather_status_collected",
            &[("name", &node.name), ("route", route_name)],
        )
    }
}
