//! Main menu screen.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::content::input_bindings;
use crate::data::GameData;
use crate::state::StateTransition;

pub struct MenuState;

impl MenuState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Enter) {
            return Some(StateTransition::StartGame);
        }

        let (button_x, button_y, button_w, button_h) = start_button_rect();
        if was_clicked(button_x, button_y, button_w, button_h) {
            return Some(StateTransition::StartGame);
        }

        None
    }

    pub fn draw(&self, data: &GameData) {
        let panel_width = 480.0;
        let panel_height = 280.0;
        let panel_x = screen_width() * 0.5 - panel_width * 0.5;
        let panel_y = screen_height() * 0.5 - panel_height * 0.5;

        draw_rectangle(panel_x, panel_y, panel_width, panel_height, dark::PANEL);
        draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            2.0,
            dark::ACCENT,
        );

        draw_text(
            "Alchemy Tower",
            panel_x + 28.0,
            panel_y + 60.0,
            42.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            "Restore the tower, explore the valley, and brew unstable magic.",
            panel_x + 28.0,
            panel_y + 105.0,
            24.0,
            dark::TEXT,
        );
        draw_text(
            &format!("Starting area: {}", data.config.starting_area),
            panel_x + 28.0,
            panel_y + 145.0,
            22.0,
            dark::TEXT_DIM,
        );

        let (button_x, button_y, button_w, button_h) = start_button_rect();
        let hovered =
            Rect::new(button_x, button_y, button_w, button_h).contains(mouse_position().into());
        let button_color = if hovered { dark::HOVERED } else { dark::ACCENT };
        draw_rectangle(button_x, button_y, button_w, button_h, button_color);
        draw_text(
            "Start Game",
            button_x + 24.0,
            button_y + 30.0,
            28.0,
            dark::TEXT_BRIGHT,
        );

        draw_text(
            &format!(
                "WASD move  {} interact  {} alchemy  F5 save  F9 load  Esc pause",
                input_bindings().global.interact,
                input_bindings().alchemy.open
            ),
            panel_x + 28.0,
            panel_y + 250.0,
            20.0,
            dark::TEXT_DIM,
        );
    }
}

fn start_button_rect() -> (f32, f32, f32, f32) {
    let panel_width = 480.0;
    let panel_height = 280.0;
    let panel_x = screen_width() * 0.5 - panel_width * 0.5;
    let panel_y = screen_height() * 0.5 - panel_height * 0.5;
    (panel_x + 28.0, panel_y + 185.0, 180.0, 46.0)
}
