//! Pause screen that preserves the current gameplay snapshot.

use crate::art::ArtAssets;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};
use crate::view_models::pause::PauseOverlayView;

#[path = "pause_input.rs"]
mod pause_input;

use self::pause_input::{selected_pause_action, PauseAction};

pub(crate) struct PauseState {
    gameplay: GameplayState,
}

impl PauseState {
    pub(crate) fn new(gameplay: GameplayState) -> Self {
        Self { gameplay }
    }

    pub(crate) fn update(&mut self, data: &GameData) -> Option<StateTransition> {
        match selected_pause_action() {
            Some(PauseAction::Resume) => Some(StateTransition::ResumeGame),
            Some(PauseAction::Save) => {
                self.gameplay.save_progress(data);
                None
            }
            Some(PauseAction::Load) => {
                self.gameplay.load_progress(data);
                None
            }
            Some(PauseAction::ReturnToMenu) => Some(StateTransition::ReturnToMenu),
            None => None,
        }
    }

    pub(crate) fn draw(&self, data: &GameData, art: &ArtAssets) {
        self.gameplay.draw(data, art);
        crate::ui::draw_pause_overlay(&self.pause_overlay_view());
    }

    pub(crate) fn into_gameplay(self) -> GameplayState {
        self.gameplay
    }

    fn pause_overlay_view(&self) -> PauseOverlayView {
        PauseOverlayView {
            title: ui_copy("pause_title").to_owned(),
            resume_label: ui_copy("pause_resume").to_owned(),
            save_label: ui_copy("pause_save").to_owned(),
            load_label: ui_copy("pause_load").to_owned(),
            menu_label: ui_copy("pause_menu").to_owned(),
            resume_hint: ui_format(
                "pause_resume_hint",
                &[
                    ("cancel", &input_bindings().global.cancel),
                    ("save", &input_bindings().global.save),
                    ("load", &input_bindings().global.load),
                ],
            ),
            status_text: self.gameplay.pause_status_text().to_owned(),
        }
    }
}
