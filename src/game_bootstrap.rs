use super::game_state::GameState;
use super::Game;
use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::data::load_embedded_or_fallback;
use crate::state::MenuState;

pub(super) async fn load_game() -> Game {
    let data = load_embedded_or_fallback();
    let art = ArtAssets::load(&data).await;
    let audio = AudioAssets::load().await;

    Game {
        data,
        art,
        audio,
        state: Some(GameState::Menu(MenuState::new())),
    }
}
