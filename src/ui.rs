//! Small UI helpers for overlays and prompts.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub fn draw_panel(x: f32, y: f32, width: f32, height: f32, title: &str) {
    draw_rectangle(x, y, width, height, dark::PANEL);
    draw_rectangle(x, y, width, 28.0, dark::PANEL_HEADER);
    draw_rectangle_lines(x, y, width, height, 2.0, dark::ACCENT);
    draw_text(title, x + 12.0, y + 20.0, 22.0, dark::TEXT_BRIGHT);
}

pub fn draw_interaction_prompt(position: Vec2, text: &str) {
    let measured = measure_text(text, None, 22, 1.0);
    let width = measured.width + 18.0;
    let x = position.x - width * 0.5;
    let y = position.y - 24.0;

    draw_rectangle(x, y, width, 28.0, Color::from_rgba(18, 18, 24, 220));
    draw_rectangle_lines(x, y, width, 28.0, 2.0, dark::ACCENT);
    draw_text(text, x + 9.0, y + 20.0, 22.0, dark::TEXT_BRIGHT);
}
