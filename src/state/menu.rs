//! Main menu screen.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::art::{draw_character_frame, draw_texture_centered, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;
use crate::state::StateTransition;
use crate::ui::{centered_panel_rect, draw_action_button, draw_panel_frame, draw_wrapped_text, inset_rect};

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

    pub fn draw(&self, data: &GameData, art: &ArtAssets) {
        if let Some(texture) = art.background(&data.config.starting_area) {
            draw_texture_ex(
                texture,
                0.0,
                0.0,
                Color::from_rgba(255, 255, 255, 215),
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(12, 14, 20, 120),
        );

        if let Some(texture) = art.station("entry_cauldron") {
            draw_texture_centered(
                texture,
                vec2(screen_width() - 180.0, screen_height() - 170.0),
                vec2(196.0, 196.0),
                Color::new(1.0, 1.0, 1.0, 0.92),
            );
        }
        if let Some(texture) = art.player() {
            draw_character_frame(
                texture,
                vec2(170.0, screen_height() - 170.0),
                vec2(0.0, 1.0),
                true,
                1.0,
            );
        }

        let panel = centered_panel_rect(620.0, 360.0);

        draw_panel_frame(panel);

        draw_text(ui_copy("menu_title"), panel.x + 28.0, panel.y + 60.0, 42.0, dark::TEXT_BRIGHT);
        draw_wrapped_text(
            ui_copy("menu_subtitle"),
            panel.x + 28.0,
            panel.y + 105.0,
            panel.w - 56.0,
            24.0,
            24.0,
            dark::TEXT,
        );
        draw_text(
            &ui_format("menu_starting_area", &[("area", &data.config.starting_area)]),
            panel.x + 28.0,
            panel.y + 165.0,
            22.0,
            dark::TEXT_DIM,
        );

        draw_action_button(start_button_rect(), ui_copy("menu_start_game"), 24.0);

        draw_wrapped_text(
            &ui_format(
                "menu_controls_primary",
                &[
                    ("interact", &input_bindings().global.interact),
                    ("alchemy", &input_bindings().alchemy.open),
                    ("journal", &input_bindings().global.journal),
                ],
            ),
            panel.x + 28.0,
            panel.y + 272.0,
            panel.w - 56.0,
            20.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &ui_format(
                "menu_controls_secondary",
                &[
                    ("confirm", &input_bindings().global.confirm),
                    ("cancel", &input_bindings().global.cancel),
                ],
            ),
            panel.x + 28.0,
            panel.y + 300.0,
            panel.w - 56.0,
            20.0,
            20.0,
            dark::TEXT_DIM,
        );
    }
}

fn start_button_rect() -> Rect {
    inset_rect(centered_panel_rect(620.0, 360.0), 28.0, 205.0, 220.0, 46.0)
}
