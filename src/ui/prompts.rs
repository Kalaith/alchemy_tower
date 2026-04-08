use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub fn draw_interaction_prompt(position: Vec2, text: &str) {
    let measured = measure_text(text, None, 20, 1.0);
    let width = measured.width + 28.0;
    let x = position.x - width * 0.5;
    let y = position.y - 28.0;

    draw_rectangle(x, y, width, 32.0, Color::from_rgba(16, 18, 26, 204));
    draw_rectangle_lines(x, y, width, 32.0, 1.5, Color::from_rgba(176, 226, 255, 120));
    draw_text(text, x + 14.0, y + 22.0, 20.0, dark::TEXT_BRIGHT);
}
