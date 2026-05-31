use super::GameplayState;
use crate::data::AreaDefinition;
use crate::view_models::hud::HudFeedbackView;
use macroquad::prelude::{vec2, Vec2};

impl GameplayState {
    pub(super) fn build_hud_feedbacks(&self, area: &AreaDefinition) -> Vec<HudFeedbackView> {
        let offset = self.camera_offset(area);
        self.runtime
            .gather_feedbacks
            .iter()
            .map(|feedback| {
                let life = feedback.remaining_seconds;
                let t = 1.0
                    - if feedback.emphasis {
                        life / 0.8
                    } else {
                        life / 0.45
                    };
                let radius = if feedback.emphasis {
                    (12.0 + t * 24.0) * feedback.burst_scale
                } else {
                    (10.0 + t * 16.0) * feedback.burst_scale
                };
                let alpha = (1.0 - t).clamp(0.0, 1.0);
                let color = [
                    feedback.color[0],
                    feedback.color[1],
                    feedback.color[2],
                    alpha,
                ];
                let screen_pos = offset + vec2(feedback.position[0], feedback.position[1]);
                let sparkle_points = std::array::from_fn(|index| {
                    let angle = t * 1.1 + index as f32 * (std::f32::consts::TAU / 8.0);
                    let sparkle =
                        vec2(angle.cos(), angle.sin()) * (radius + 4.0 + index as f32 * 1.6);
                    hud_point(screen_pos + sparkle)
                });

                HudFeedbackView {
                    position: hud_point(screen_pos),
                    radius,
                    color,
                    sparkle_points,
                    burst_scale: feedback.burst_scale,
                }
            })
            .collect()
    }
}

fn hud_point(point: Vec2) -> [f32; 2] {
    [point.x, point.y]
}
