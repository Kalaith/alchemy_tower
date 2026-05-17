use macroquad::prelude::*;

pub fn wrapped_lines(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    macroquad_toolkit::ui::wrap_text(text, max_width, font_size)
}

pub fn truncate_text_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    macroquad_toolkit::ui::truncate_text_to_width(text, max_width, font_size)
}

pub fn draw_wrapped_text(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: f32,
    line_height: f32,
    color: Color,
) {
    let mut line_y = y;
    for line in wrapped_lines(text, max_width, font_size) {
        draw_text(&line, x, line_y, font_size, color);
        line_y += line_height;
    }
}
