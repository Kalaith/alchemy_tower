use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_alchemy_slots_panel(&self, data: &GameData, x: f32, y: f32, w: f32) {
        let view = self.alchemy_slots_panel_view(data);
        crate::ui::draw_alchemy_slots_panel_view(&view, x, y, w);
    }
}
