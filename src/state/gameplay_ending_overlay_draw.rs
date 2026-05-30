use super::GameplayState;

impl GameplayState {
    pub(super) fn draw_ending_overlay(&self) {
        crate::ui::draw_ending_overlay_view();
    }
}
