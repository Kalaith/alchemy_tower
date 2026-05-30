use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_quest_board_overlay(&self, data: &GameData) {
        let view = self.quest_board_overlay_view(data);
        crate::ui::draw_quest_board_overlay_view(&view);
    }
}
