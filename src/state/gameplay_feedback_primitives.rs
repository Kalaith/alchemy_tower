use super::gameplay_feedback_types::{GatherFeedback, GatherToast};
use super::GameplayState;

impl GameplayState {
    pub(super) fn push_event_toast(&mut self, text: impl Into<String>, color: [f32; 4]) {
        self.push_event_toast_with_icon(text, color, "");
    }

    pub(super) fn push_event_toast_with_icon(
        &mut self,
        _text: impl Into<String>,
        _color: [f32; 4],
        _icon_key: &str,
    ) {
        self.runtime.gather_toasts.insert(
            0,
            GatherToast {
                remaining_seconds: 2.2,
            },
        );
        self.runtime.gather_toasts.truncate(3);
    }

    pub(super) fn trigger_camera_shake(&mut self, seconds: f32, intensity: f32) {
        self.runtime.camera_shake_seconds = self.runtime.camera_shake_seconds.max(seconds);
        self.runtime.camera_shake_intensity = self.runtime.camera_shake_intensity.max(intensity);
    }

    pub(super) fn trigger_world_feedback(
        &mut self,
        position: [f32; 2],
        color: [f32; 4],
        emphasis: bool,
        burst_scale: f32,
    ) {
        self.runtime.gather_feedbacks.push(GatherFeedback {
            position,
            remaining_seconds: if emphasis { 0.9 } else { 0.55 },
            color,
            emphasis,
            burst_scale,
        });
    }

    pub(super) fn player_feedback_position(&self) -> [f32; 2] {
        [self.world.player.position.x, self.world.player.position.y]
    }
}

pub(super) fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> [f32; 4] {
    [
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        alpha as f32 / 255.0,
    ]
}
