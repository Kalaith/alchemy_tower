use super::GameplayState;

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

}
