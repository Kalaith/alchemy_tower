//! Pause screen that preserves the current gameplay snapshot.

use crate::art::ArtAssets;
use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};

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
        crate::ui::draw_pause_overlay(self.gameplay.pause_status_text());
    }

    pub(crate) fn into_gameplay(self) -> GameplayState {
        self.gameplay
    }
}
