use super::truncate_text_to_width;
use macroquad::prelude::Color;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(super) fn draw_centered_text(
    text: &str,
    x: f32,
    baseline_y: f32,
    width: f32,
    font_size: f32,
    color: Color,
) {
    let measured = measure_ui_text(text, None, font_size as u16, 1.0);
    draw_ui_text(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        color,
    );
}

pub(super) fn draw_centered_text_shadowed(
    text: &str,
    x: f32,
    baseline_y: f32,
    width: f32,
    font_size: f32,
    color: Color,
) {
    let measured = measure_ui_text(text, None, font_size as u16, 1.0);
    draw_text_shadowed(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        color,
    );
}

pub(super) fn draw_text_shadowed(
    text: &str,
    x: f32,
    baseline_y: f32,
    font_size: f32,
    color: Color,
) {
    draw_ui_text(
        text,
        x + 1.5,
        baseline_y + 2.0,
        font_size,
        Color::from_rgba(0, 0, 0, 130),
    );
    draw_ui_text(text, x, baseline_y, font_size, color);
}

pub(super) fn draw_wrapped_text_limited(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: f32,
    line_height: f32,
    color: Color,
    max_lines: usize,
    truncation_suffix: &str,
) {
    let mut lines = macroquad_toolkit::ui::wrap_text(text, max_width, font_size);
    if lines.len() > max_lines {
        lines.truncate(max_lines);
        if let Some(last) = lines.last_mut() {
            let trimmed = last.trim_end_matches('.').to_owned();
            let truncated = format!("{trimmed}{truncation_suffix}");
            *last = truncate_text_to_width(&truncated, max_width, font_size);
        }
    }
    for (index, line) in lines.iter().enumerate() {
        draw_ui_text(line, x, y + index as f32 * line_height, font_size, color);
    }
}
