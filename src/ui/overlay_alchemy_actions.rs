use crate::content::ui_copy;
use super::draw_action_button;
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, draw_text, Color, Rect};
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_action_buttons(x: f32, y: f32) {
    draw_action_button(
        Rect::new(x + 20.0, y + 368.0, 82.0, 28.0),
        ui_copy("overlay_alchemy_sort_button"),
        0.0,
    );
    draw_action_button(
        Rect::new(x + 114.0, y + 368.0, 82.0, 28.0),
        ui_copy("overlay_alchemy_clear_button"),
        0.0,
    );
    draw_action_button(
        Rect::new(x + 208.0, y + 368.0, 90.0, 28.0),
        ui_copy("overlay_alchemy_repeat_button"),
        0.0,
    );
    draw_rectangle(
        x + 310.0,
        y + 368.0,
        90.0,
        28.0,
        Color::from_rgba(38, 58, 46, 210),
    );
    draw_rectangle_lines(
        x + 310.0,
        y + 368.0,
        90.0,
        28.0,
        1.5,
        Color::from_rgba(188, 255, 220, 96),
    );
    draw_text(
        ui_copy("overlay_alchemy_brew_button"),
        x + 338.0,
        y + 388.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}
