use super::GameplayState;
use crate::content::{narrative_text, ui_copy};
use crate::view_models::ending::EndingOverlayView;

impl GameplayState {
    pub(super) fn ending_overlay_view(&self) -> EndingOverlayView {
        EndingOverlayView {
            title: ui_copy("overlay_ending_title").to_owned(),
            body: narrative_text().overlays.observatory_epilogue.clone(),
            footer: narrative_text().overlays.observatory_footer.clone(),
        }
    }
}
