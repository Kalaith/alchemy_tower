use super::GameplayState;

impl GameplayState {
    pub(super) fn draw_alchemy_controls_panel(&self, x: f32, y: f32) {
        crate::ui::draw_alchemy_controls_panel_view(x, y);
    }
}
