use super::gameplay_feedback_types::{FieldDiscoveryFeedback, GatherFeedback};
use super::GameplayState;
use crate::data::{GameData, GatherNodeDefinition};

#[path = "gameplay_gathering_text.rs"]
mod gathering_text;

impl GameplayState {
    pub(super) fn node_is_available(&self, node: &GatherNodeDefinition) -> bool {
        self.world.available_nodes.contains(&node.id)
    }

    pub(super) fn gather_attempt_status(
        &self,
        _data: &GameData,
        node: &GatherNodeDefinition,
    ) -> String {
        gathering_text::attempt_status(self.item_has_field_notes(&node.item_id), &node.name)
    }

    pub(super) fn trigger_gather_feedback(
        &mut self,
        data: &GameData,
        node: &GatherNodeDefinition,
        discovery: &FieldDiscoveryFeedback,
    ) {
        let center = [node.position[0], node.position[1]];
        let base_color = feedback_color(node.color);
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
            self.trigger_gather_journal_toast(gathering_text::journal_toast(data, &node.route_id));
        }
        if discovery.improved_quality {
            self.trigger_gather_quality_toast(gathering_text::quality_toast(&node.name));
        }
        if discovery.variant_discovered {
            self.trigger_gather_variant_toast(gathering_text::variant_toast(&node.name));
        }
        if emphasis {
            self.runtime.gather_pause_seconds = self.runtime.gather_pause_seconds.max(0.08);
            self.trigger_camera_shake(0.08, 2.4);
        }
    }

    pub(super) fn gather_status_text(
        &self,
        data: &GameData,
        node: &GatherNodeDefinition,
        discovery: &FieldDiscoveryFeedback,
    ) -> String {
        gathering_text::status(
            data,
            node,
            discovery.variant_discovered,
            discovery.improved_quality,
        )
    }
}

fn feedback_color(values: [u8; 4]) -> [f32; 4] {
    [
        values[0] as f32 / 255.0,
        values[1] as f32 / 255.0,
        values[2] as f32 / 255.0,
        values[3] as f32 / 255.0,
    ]
}
