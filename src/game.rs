//! High-level game loop orchestration.

use macroquad::prelude::*;

#[path = "game_bootstrap.rs"]
mod game_bootstrap;
#[path = "game_state.rs"]
mod game_state;
#[path = "game_transition.rs"]
mod game_transition;

use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::GameData;

use self::game_state::GameState;
use self::game_transition::apply_transition;

pub(crate) struct Game {
    data: GameData,
    art: ArtAssets,
    audio: AudioAssets,
    state: Option<GameState>,
}

impl Game {
    pub(crate) async fn new() -> Self {
        game_bootstrap::load_game().await
    }

    pub(crate) fn update(&mut self) {
        let Some(mut current_state) = self.state.take() else {
            return;
        };
        let transition = current_state.update(&self.data, &self.audio);

        self.state = Some(match transition {
            Some(next) => apply_transition(current_state, next),
            None => current_state,
        });
    }

    pub(crate) fn draw(&self) {
        clear_background(Color::from_rgba(22, 24, 30, 255));

        let Some(state) = self.state.as_ref() else {
            return;
        };
        state.draw(&self.data, &self.art);
    }
}
