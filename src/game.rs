//! High-level game loop and state transitions.

use macroquad::prelude::*;

use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::{GameData, GameDataLoader};
use crate::state::{GameplayState, MenuState, PauseState, StateTransition};

pub enum GameState {
    Menu(MenuState),
    Gameplay(GameplayState),
    Paused(PauseState),
}

pub struct Game {
    data: GameData,
    art: ArtAssets,
    audio: AudioAssets,
    state: Option<GameState>,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameDataLoader::load_embedded().unwrap_or_else(|error| {
            eprintln!("Failed to load embedded game data: {error}");
            GameData::runtime_fallback()
        });
        let art = ArtAssets::load(&data).await;
        let audio = AudioAssets::load().await;

        Self {
            data,
            art,
            audio,
            state: Some(GameState::Menu(MenuState::new())),
        }
    }

    pub fn update(&mut self) {
        let mut current_state = self
            .state
            .take()
            .expect("game state should always be present during update");
        let transition = match &mut current_state {
            GameState::Menu(state) => state.update(),
            GameState::Gameplay(state) => state.update(&self.data, &self.audio),
            GameState::Paused(state) => state.update(&self.data),
        };

        self.state = Some(match transition {
            Some(next) => self.apply_transition(current_state, next),
            None => current_state,
        });
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(22, 24, 30, 255));

        match self
            .state
            .as_ref()
            .expect("game state should always be present during draw")
        {
            GameState::Menu(state) => state.draw(&self.data, &self.art),
            GameState::Gameplay(state) => state.draw(&self.data, &self.art),
            GameState::Paused(state) => state.draw(&self.data, &self.art),
        }
    }

    fn apply_transition(&self, current_state: GameState, transition: StateTransition) -> GameState {
        match transition {
            StateTransition::StartGame => GameState::Gameplay(GameplayState::new(&self.data)),
            StateTransition::ReturnToMenu => GameState::Menu(MenuState::new()),
            StateTransition::Pause => match current_state {
                GameState::Gameplay(gameplay) => GameState::Paused(PauseState::new(gameplay)),
                other => other,
            },
            StateTransition::ResumeGame => match current_state {
                GameState::Paused(paused) => GameState::Gameplay(paused.into_gameplay()),
                other => other,
            },
        }
    }
}
