use macroquad::prelude::*;

mod alchemy;
mod content;
mod data;
mod game;
mod save;
mod state;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Alchemy Tower".to_owned(),
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
