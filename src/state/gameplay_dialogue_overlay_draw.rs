use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_dialogue_overlay(&self, data: &GameData) {
        if let Some(view) = self.dialogue_overlay_view(data) {
            crate::ui::draw_dialogue_overlay_view(&view);
        }
    }
}
