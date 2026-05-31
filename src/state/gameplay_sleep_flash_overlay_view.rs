use super::GameplayState;
use crate::content::ui_copy;
use crate::view_models::sleep::SleepFlashOverlayView;

impl GameplayState {
    pub(super) fn sleep_flash_overlay_view(&self) -> SleepFlashOverlayView {
        SleepFlashOverlayView {
            remaining_seconds: self.runtime.sleep_flash_seconds,
            title: ui_copy("gameplay_sleep_flash_title").to_owned(),
            body: ui_copy("gameplay_fainted_home").to_owned(),
        }
    }
}
