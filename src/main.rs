#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]

use macroquad::prelude::{next_frame, Conf};
use macroquad_toolkit::capture;

mod alchemy;
mod alchemy_layout;
mod archive_layout;
mod art;
mod audio;
mod content;
mod data;
mod game;
mod input;
mod journal_layout;
mod menu_layout;
mod pause_layout;
mod save;
mod state;
mod ui;
mod view_models;

use game::Game;

fn window_conf() -> Conf {
    capture::capture_window_conf(
        "ALCHEMY_TOWER",
        content::ui_copy("window_title"),
        1280,
        720,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Screenshot harness: when ALCHEMY_TOWER_CAPTURE_PATH is set, seed a
    // scene, simulate deterministic frames, write a PNG, and exit.
    if let Some(config) = capture::CaptureConfig::from_env("ALCHEMY_TOWER") {
        game.begin_capture_scene(&config.scene);
        capture::run_capture(&config, |_dt| {
            game.update();
            game.draw();
        })
        .await;
        return;
    }

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
