use macroquad::prelude::{next_frame, Conf};

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
    Conf {
        window_title: content::ui_copy("window_title").to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
