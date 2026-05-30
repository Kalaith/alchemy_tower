use super::GameplayState;

impl GameplayState {
    pub(super) fn draw_sleep_flash_overlay(&self) {
        crate::ui::draw_sleep_flash_overlay_view(self.runtime.sleep_flash_seconds);
    }
}
