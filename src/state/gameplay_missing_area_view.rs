use super::GameplayState;
use crate::content::ui_copy;

impl GameplayState {
    pub(super) fn missing_area_message(&self) -> &str {
        ui_copy("gameplay_missing_area")
    }
}
