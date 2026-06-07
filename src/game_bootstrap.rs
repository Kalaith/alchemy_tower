use super::game_state::GameState;
use super::Game;
use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::load_embedded_or_fallback;

pub(super) async fn load_game() -> Game {
    let data = load_embedded_or_fallback();
    let art = ArtAssets::load(&data)
        .await
        .unwrap_or_else(|error| panic!("Failed to load art assets: {error}"));
    let audio = AudioAssets::load()
        .await
        .unwrap_or_else(|error| panic!("Failed to load audio assets: {error}"));

    Game {
        data,
        art,
        audio,
        state: Some(GameState::new_menu()),
    }
}
