use super::gameplay_feedback_types::{GatherFeedback, GatherToast};
use super::GameplayState;
use macroquad::prelude::{Color, Vec2};

impl GameplayState {
    pub(super) fn update_active_effects(&mut self, frame_time: f32) {
        for effect in &mut self.runtime.active_effects {
            effect.remaining_seconds -= frame_time;
        }
        self.runtime
            .active_effects
            .retain(|effect| effect.remaining_seconds > 0.0);
    }

    pub(super) fn update_gather_feedback(&mut self, frame_time: f32) {
        self.runtime.gather_pause_seconds =
            (self.runtime.gather_pause_seconds - frame_time).max(0.0);
        self.runtime.camera_shake_seconds =
            (self.runtime.camera_shake_seconds - frame_time).max(0.0);
        self.runtime.sleep_flash_seconds = (self.runtime.sleep_flash_seconds - frame_time).max(0.0);
        self.runtime.footstep_cooldown_seconds =
            (self.runtime.footstep_cooldown_seconds - frame_time).max(0.0);
        if self.runtime.camera_shake_seconds <= 0.0 {
            self.runtime.camera_shake_intensity = 0.0;
        }
        for toast in &mut self.runtime.gather_toasts {
            toast.remaining_seconds -= frame_time;
        }
        self.runtime
            .gather_toasts
            .retain(|toast| toast.remaining_seconds > 0.0);
        for feedback in &mut self.runtime.gather_feedbacks {
            feedback.remaining_seconds -= frame_time;
        }
        self.runtime
            .gather_feedbacks
            .retain(|feedback| feedback.remaining_seconds > 0.0);
    }

    pub(super) fn push_event_toast(&mut self, text: impl Into<String>, color: Color) {
        self.push_event_toast_with_icon(text, color, "");
    }

    pub(super) fn push_event_toast_with_icon(
        &mut self,
        _text: impl Into<String>,
        _color: Color,
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
        position: Vec2,
        color: Color,
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
}
