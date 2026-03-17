//! Pause screen that preserves the current gameplay snapshot.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};

pub struct PauseState {
    gameplay: GameplayState,
}

impl PauseState {
    pub fn new(gameplay: GameplayState) -> Self {
        Self { gameplay }
    }

    pub fn update(&mut self) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ResumeGame);
        }

        let (resume_x, resume_y, resume_w, resume_h) = resume_button_rect();
        if was_clicked(resume_x, resume_y, resume_w, resume_h) {
            return Some(StateTransition::ResumeGame);
        }

        let (menu_x, menu_y, menu_w, menu_h) = menu_button_rect();
        if was_clicked(menu_x, menu_y, menu_w, menu_h) {
            return Some(StateTransition::ReturnToMenu);
        }

        None
    }

    pub fn draw(&self, data: &GameData) {
        self.gameplay.draw(data);

        let width = 320.0;
        let height = 180.0;
        let x = screen_width() * 0.5 - width * 0.5;
        let y = screen_height() * 0.5 - height * 0.5;

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 110),
        );
        draw_rectangle(x, y, width, height, dark::PANEL);
        draw_rectangle_lines(x, y, width, height, 2.0, dark::ACCENT);
        draw_text("Paused", x + 24.0, y + 48.0, 38.0, dark::TEXT_BRIGHT);

        let (resume_x, resume_y, resume_w, resume_h) = resume_button_rect();
        let resume_hovered =
            Rect::new(resume_x, resume_y, resume_w, resume_h).contains(mouse_position().into());
        draw_rectangle(
            resume_x,
            resume_y,
            resume_w,
            resume_h,
            if resume_hovered {
                dark::HOVERED
            } else {
                dark::ACCENT
            },
        );
        draw_text(
            "Resume",
            resume_x + 20.0,
            resume_y + 28.0,
            28.0,
            dark::TEXT_BRIGHT,
        );

        let (menu_x, menu_y, menu_w, menu_h) = menu_button_rect();
        let menu_hovered =
            Rect::new(menu_x, menu_y, menu_w, menu_h).contains(mouse_position().into());
        draw_rectangle(
            menu_x,
            menu_y,
            menu_w,
            menu_h,
            if menu_hovered {
                dark::HOVERED
            } else {
                dark::ACCENT
            },
        );
        draw_text(
            "Menu",
            menu_x + 28.0,
            menu_y + 28.0,
            28.0,
            dark::TEXT_BRIGHT,
        );

        draw_text(
            "Esc also resumes.",
            x + 24.0,
            y + 148.0,
            22.0,
            dark::TEXT_DIM,
        );
    }

    pub fn into_gameplay(self) -> GameplayState {
        self.gameplay
    }
}

fn resume_button_rect() -> (f32, f32, f32, f32) {
    let width = 320.0;
    let height = 180.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() * 0.5 - height * 0.5;
    (x + 24.0, y + 78.0, 130.0, 42.0)
}

fn menu_button_rect() -> (f32, f32, f32, f32) {
    let width = 320.0;
    let height = 180.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() * 0.5 - height * 0.5;
    (x + 166.0, y + 78.0, 130.0, 42.0)
}
