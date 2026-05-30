use super::gameplay_support::rgba;
use super::gameplay_feedback_types::{FieldDiscoveryFeedback, GatherFeedback};
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, GatherNodeDefinition};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn node_is_available(&self, node: &GatherNodeDefinition) -> bool {
        self.world.available_nodes.contains(&node.id)
    }

    pub(super) fn gather_attempt_status(
        &self,
        _data: &GameData,
        node: &GatherNodeDefinition,
    ) -> String {
        if self.item_has_field_notes(&node.item_id) {
            ui_format("gather_attempt_known", &[("name", &node.name)])
        } else {
            ui_format("gather_attempt_none", &[])
        }
    }

    pub(super) fn trigger_gather_feedback(
        &mut self,
        data: &GameData,
        node: &GatherNodeDefinition,
        discovery: &FieldDiscoveryFeedback,
    ) {
        let center = vec2(node.position[0], node.position[1]);
        let base_color = rgba(node.color);
        let emphasis = discovery.variant_discovered || discovery.improved_quality;
        let duration = if emphasis { 0.8 } else { 0.45 };
        self.runtime.gather_feedbacks.push(GatherFeedback {
            position: center,
            remaining_seconds: duration,
            color: base_color,
            emphasis,
            burst_scale: if emphasis { 1.35 } else { 1.0 },
        });

        if discovery.new_note {
            let route_name = data
                .route(&node.route_id)
                .map(|route| route.name.as_str())
                .unwrap_or(ui_copy("gather_fallback_notes"));
            self.push_gather_toast(
                ui_format("gather_toast_journal", &[("route", route_name)]),
                Color::from_rgba(176, 226, 255, 255),
            );
        }
        if discovery.improved_quality {
            self.push_gather_toast(
                ui_format("gather_toast_quality", &[("name", &node.name)]),
                Color::from_rgba(255, 228, 150, 255),
            );
        }
        if discovery.variant_discovered {
            self.push_gather_toast(
                ui_format("gather_toast_variant", &[("name", &node.name)]),
                Color::from_rgba(188, 255, 220, 255),
            );
        }
        if emphasis {
            self.runtime.gather_pause_seconds = self.runtime.gather_pause_seconds.max(0.08);
            self.trigger_camera_shake(0.08, 2.4);
        }
    }

    pub(super) fn push_gather_toast(&mut self, text: String, color: Color) {
        self.push_event_toast(text, color);
    }

    pub(super) fn gather_status_text(
        &self,
        data: &GameData,
        node: &GatherNodeDefinition,
        discovery: &FieldDiscoveryFeedback,
    ) -> String {
        let route_name = data
            .route(&node.route_id)
            .map(|route| route.name.as_str())
            .unwrap_or(ui_copy("gather_fallback_route"));
        if discovery.variant_discovered {
            ui_format(
                "gather_status_variant",
                &[("name", &node.name), ("route", route_name)],
            )
        } else if discovery.improved_quality {
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
}
