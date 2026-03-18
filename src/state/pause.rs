//! Pause screen that preserves the current gameplay snapshot.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::content::ui_copy;
use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};
use crate::ui::{centered_panel_rect, draw_action_button, draw_panel_frame, inset_rect};

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

        let resume = resume_button_rect();
        if was_clicked(resume.x, resume.y, resume.w, resume.h) {
            return Some(StateTransition::ResumeGame);
        }

        let menu = menu_button_rect();
        if was_clicked(menu.x, menu.y, menu.w, menu.h) {
            return Some(StateTransition::ReturnToMenu);
        }

        None
    }

    pub fn draw(&self, data: &GameData) {
        self.gameplay.draw(data);

        let panel = centered_panel_rect(320.0, 180.0);

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 110),
        );
        draw_panel_frame(panel);
        draw_text(ui_copy("pause_title"), panel.x + 24.0, panel.y + 48.0, 38.0, dark::TEXT_BRIGHT);

        draw_action_button(resume_button_rect(), ui_copy("pause_resume"), 20.0);

        draw_action_button(menu_button_rect(), ui_copy("pause_menu"), 28.0);

        draw_text(
            ui_copy("pause_resume_hint"),
            panel.x + 24.0,
            panel.y + 148.0,
            22.0,
            dark::TEXT_DIM,
        );
    }

    pub fn into_gameplay(self) -> GameplayState {
        self.gameplay
    }
}

fn resume_button_rect() -> Rect {
    inset_rect(centered_panel_rect(320.0, 180.0), 24.0, 78.0, 130.0, 42.0)
}

fn menu_button_rect() -> Rect {
    inset_rect(centered_panel_rect(320.0, 180.0), 166.0, 78.0, 130.0, 42.0)
}
