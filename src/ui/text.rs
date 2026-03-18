use macroquad::prelude::*;

pub fn draw_wrapped_text(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: f32,
    line_height: f32,
    color: Color,
) {
    let mut line = String::new();
    let mut line_y = y;

    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if measure_text(&candidate, None, font_size as u16, 1.0).width <= max_width {
            line = candidate;
        } else {
            if !line.is_empty() {
                draw_text(&line, x, line_y, font_size, color);
                line_y += line_height;
            }
            line = word.to_owned();
        }
    }

    if !line.is_empty() {
        draw_text(&line, x, line_y, font_size, color);
    }
}
