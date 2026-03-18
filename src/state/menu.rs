//! Main menu screen.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;
use crate::state::StateTransition;
use crate::ui::{centered_panel_rect, draw_action_button, draw_panel_frame, inset_rect};

pub struct MenuState;

impl MenuState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Enter) {
            return Some(StateTransition::StartGame);
        }

        let button = start_button_rect();
        if was_clicked(button.x, button.y, button.w, button.h) {
            return Some(StateTransition::StartGame);
        }

        None
    }

    pub fn draw(&self, data: &GameData) {
        let panel = centered_panel_rect(480.0, 280.0);

        draw_panel_frame(panel);

        draw_text(ui_copy("menu_title"), panel.x + 28.0, panel.y + 60.0, 42.0, dark::TEXT_BRIGHT);
        draw_text(
            ui_copy("menu_subtitle"),
            panel.x + 28.0,
            panel.y + 105.0,
            24.0,
            dark::TEXT,
        );
        draw_text(
            &ui_format("menu_starting_area", &[("area", &data.config.starting_area)]),
            panel.x + 28.0,
            panel.y + 145.0,
            22.0,
            dark::TEXT_DIM,
        );

        draw_action_button(start_button_rect(), ui_copy("menu_start_game"), 24.0);

        draw_text(
            &ui_format(
                "menu_controls",
                &[
                    ("interact", &input_bindings().global.interact),
                    ("alchemy", &input_bindings().alchemy.open),
                ],
            ),
            panel.x + 28.0,
            panel.y + 250.0,
            20.0,
            dark::TEXT_DIM,
        );
    }
}

fn start_button_rect() -> Rect {
    inset_rect(centered_panel_rect(480.0, 280.0), 28.0, 185.0, 180.0, 46.0)
}
