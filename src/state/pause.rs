//! Pause screen that preserves the current gameplay snapshot.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::art::ArtAssets;
use crate::content::ui_copy;
use crate::data::GameData;
use crate::state::{GameplayState, StateTransition};
use crate::ui::{centered_panel_rect, draw_action_button, draw_panel_frame, draw_wrapped_text, inset_rect};

pub struct PauseState {
    gameplay: GameplayState,
}

impl PauseState {
    pub fn new(gameplay: GameplayState) -> Self {
        Self { gameplay }
    }

    pub fn update(&mut self, data: &GameData) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ResumeGame);
        }

        let resume = resume_button_rect();
        if was_clicked(resume.x, resume.y, resume.w, resume.h) {
            return Some(StateTransition::ResumeGame);
        }

        let save = save_button_rect();
        if is_key_pressed(KeyCode::F5) || was_clicked(save.x, save.y, save.w, save.h) {
            self.gameplay.save_progress(data);
        }

        let load = load_button_rect();
        if is_key_pressed(KeyCode::F9) || was_clicked(load.x, load.y, load.w, load.h) {
            self.gameplay.load_progress(data);
        }

        let menu = menu_button_rect();
        if was_clicked(menu.x, menu.y, menu.w, menu.h) {
            return Some(StateTransition::ReturnToMenu);
        }

        None
    }

    pub fn draw(&self, data: &GameData, art: &ArtAssets) {
        self.gameplay.draw(data, art);

        let panel = centered_panel_rect(380.0, 300.0);

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
        draw_action_button(save_button_rect(), ui_copy("pause_save"), 20.0);
        draw_action_button(load_button_rect(), ui_copy("pause_load"), 20.0);

        draw_action_button(menu_button_rect(), ui_copy("pause_menu"), 28.0);

        draw_wrapped_text(
            ui_copy("pause_resume_hint"),
            panel.x + 24.0,
            panel.y + 202.0,
            panel.w - 48.0,
            20.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            self.gameplay.pause_status_text(),
            panel.x + 24.0,
            panel.y + 238.0,
            panel.w - 48.0,
            18.0,
            18.0,
            dark::TEXT_BRIGHT,
        );
    }

    pub fn into_gameplay(self) -> GameplayState {
        self.gameplay
    }
}

fn resume_button_rect() -> Rect {
    inset_rect(centered_panel_rect(380.0, 300.0), 24.0, 88.0, 150.0, 42.0)
}

fn save_button_rect() -> Rect {
    inset_rect(centered_panel_rect(380.0, 300.0), 206.0, 88.0, 150.0, 42.0)
}

fn load_button_rect() -> Rect {
    inset_rect(centered_panel_rect(380.0, 300.0), 24.0, 144.0, 150.0, 42.0)
}

fn menu_button_rect() -> Rect {
    inset_rect(centered_panel_rect(380.0, 300.0), 206.0, 144.0, 150.0, 42.0)
}
