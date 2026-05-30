use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    pub(super) fn draw_alchemy_materials_panel(&self, data: &GameData, x: f32, y: f32) {
        let view = self.alchemy_materials_panel_view(data);
        crate::ui::draw_alchemy_materials_panel_view(&view, x, y);
    }
}
