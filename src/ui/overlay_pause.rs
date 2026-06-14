use super::{draw_action_button, draw_panel_frame, draw_wrapped_text};
use crate::pause_layout::{
    load_pause_button_rect, pause_menu_button_rect, pause_panel_rect, resume_pause_button_rect,
    save_pause_button_rect,
};
use crate::view_models::pause::PauseOverlayView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_pause_overlay(view: &PauseOverlayView) {
    let panel = pause_panel_rect();

    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 110),
    );
    draw_panel_frame(panel);
    draw_ui_text(
        &view.title,
        panel.x + 24.0,
        panel.y + 48.0,
        38.0,
        dark::TEXT_BRIGHT,
    );

    draw_action_button(resume_pause_button_rect(), &view.resume_label, 20.0);
    draw_action_button(save_pause_button_rect(), &view.save_label, 20.0);
    draw_action_button(load_pause_button_rect(), &view.load_label, 20.0);
    draw_action_button(pause_menu_button_rect(), &view.menu_label, 28.0);

    draw_wrapped_text(
        &view.resume_hint,
        panel.x + 24.0,
        panel.y + 202.0,
        panel.w - 48.0,
        20.0,
        20.0,
        dark::TEXT_DIM,
    );
    draw_wrapped_text(
        &view.status_text,
        panel.x + 24.0,
        panel.y + 238.0,
        panel.w - 48.0,
        18.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}
