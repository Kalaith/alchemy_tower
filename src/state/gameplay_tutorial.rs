use super::gameplay_tutorial_hint_selection::TutorialHintTone;
use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn update_tutorial_hints(&mut self, data: &GameData, frame_time: f32) {
        if self.runtime.tutorial.next_hint_delay_seconds > 0.0 {
            self.runtime.tutorial.next_hint_delay_seconds =
                (self.runtime.tutorial.next_hint_delay_seconds - frame_time).max(0.0);
        }
        if self.runtime.tutorial.next_hint_delay_seconds > 0.0
            || !self.runtime.gather_toasts.is_empty()
            || self.overlay().is_some()
        {
            return;
        }

        if let Some((text, tone)) = self.take_next_tutorial_hint(data) {
            self.push_tutorial_hint(text, tone);
            self.runtime.tutorial.next_hint_delay_seconds = 6.0;
        }
    }

    fn push_tutorial_hint(&mut self, text: String, tone: TutorialHintTone) {
        match tone {
            TutorialHintTone::Note => self.trigger_tutorial_note_hint(text),
            TutorialHintTone::Goal => self.trigger_tutorial_goal_hint(text),
            TutorialHintTone::Success => self.trigger_tutorial_success_hint(text),
            TutorialHintTone::Item => self.trigger_tutorial_item_hint(text),
        }
    }
}
