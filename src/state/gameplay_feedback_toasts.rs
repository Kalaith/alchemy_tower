use super::gameplay_feedback_primitives::rgba;
use super::GameplayState;

impl GameplayState {
    pub(super) fn trigger_gather_journal_toast(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(176, 226, 255, 255));
    }

    pub(super) fn trigger_gather_quality_toast(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(255, 228, 150, 255));
    }

    pub(super) fn trigger_gather_variant_toast(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(188, 255, 220, 255));
    }

    pub(super) fn trigger_tutorial_note_hint(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(176, 226, 255, 255));
    }

    pub(super) fn trigger_tutorial_goal_hint(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(255, 230, 170, 255));
    }

    pub(super) fn trigger_tutorial_success_hint(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(188, 255, 220, 255));
    }

    pub(super) fn trigger_tutorial_item_hint(&mut self, toast_text: impl Into<String>) {
        self.push_event_toast(toast_text, rgba(255, 214, 132, 255));
    }
}
