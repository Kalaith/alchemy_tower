use crate::content::{input_bindings, ui_copy, ui_format};
use crate::pause_layout::{
    load_pause_button_rect, pause_menu_button_rect, pause_panel_rect, resume_pause_button_rect,
    save_pause_button_rect,
};
use super::{draw_action_button, draw_panel_frame, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_pause_overlay(status_text: &str) {
    let panel = pause_panel_rect();

    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 110),
    );
    draw_panel_frame(panel);
    draw_text(
        ui_copy("pause_title"),
        panel.x + 24.0,
        panel.y + 48.0,
        38.0,
        dark::TEXT_BRIGHT,
    );

    draw_action_button(resume_pause_button_rect(), ui_copy("pause_resume"), 20.0);
    draw_action_button(save_pause_button_rect(), ui_copy("pause_save"), 20.0);
    draw_action_button(load_pause_button_rect(), ui_copy("pause_load"), 20.0);
    draw_action_button(pause_menu_button_rect(), ui_copy("pause_menu"), 28.0);

    draw_wrapped_text(
        &ui_format(
            "pause_resume_hint",
            &[
                ("cancel", &input_bindings().global.cancel),
                ("save", &input_bindings().global.save),
                ("load", &input_bindings().global.load),
            ],
        ),
        panel.x + 24.0,
        panel.y + 202.0,
        panel.w - 48.0,
        20.0,
        20.0,
        dark::TEXT_DIM,
    );
    draw_wrapped_text(
        status_text,
        panel.x + 24.0,
        panel.y + 238.0,
        panel.w - 48.0,
        18.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}
