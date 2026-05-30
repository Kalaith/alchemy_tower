use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::state::{GameplayState, MenuState, PauseState, StateTransition};

pub(in crate::game) enum GameState {
    Menu(MenuState),
    Gameplay(GameplayState),
    Paused(PauseState),
}

impl GameState {
    pub(in crate::game) fn update(
        &mut self,
        data: &GameData,
        audio: &AudioAssets,
    ) -> Option<StateTransition> {
        match self {
            GameState::Menu(state) => state.update(data),
            GameState::Gameplay(state) => state.update(data, audio),
            GameState::Paused(state) => state.update(data),
        }
    }

    pub(in crate::game) fn draw(&self, data: &GameData, art: &ArtAssets) {
        match self {
            GameState::Menu(state) => state.draw(data, art),
            GameState::Gameplay(state) => state.draw(data, art),
            GameState::Paused(state) => state.draw(data, art),
        }
    }
}
