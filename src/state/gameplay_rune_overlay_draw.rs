use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_rune_overlay(&self, data: &GameData) {
        if let Some(view) = self.rune_overlay_view(data) {
            crate::ui::draw_rune_overlay_view(&view);
        }
    }
}
