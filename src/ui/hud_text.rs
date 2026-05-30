use crate::content::ui_copy;
use crate::ui::truncate_text_to_width;
use macroquad::prelude::{draw_text, measure_text, Color};

pub(super) fn draw_centered_text(
    text: &str,
    x: f32,
    baseline_y: f32,
    width: f32,
    font_size: f32,
    color: Color,
) {
    let measured = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
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
    let measured = measure_text(text, None, font_size as u16, 1.0);
    draw_text_shadowed(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        color,
    );
}

pub(super) fn draw_text_shadowed(text: &str, x: f32, baseline_y: f32, font_size: f32, color: Color) {
    draw_text(
        text,
        x + 1.5,
        baseline_y + 2.0,
        font_size,
        Color::from_rgba(0, 0, 0, 130),
    );
    draw_text(text, x, baseline_y, font_size, color);
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
) {
    let mut lines = macroquad_toolkit::ui::wrap_text(text, max_width, font_size);
    if lines.len() > max_lines {
        lines.truncate(max_lines);
        if let Some(last) = lines.last_mut() {
            let trimmed = last.trim_end_matches('.').to_owned();
            *last = truncate_text_to_width(&format!("{trimmed}..."), max_width, font_size);
        }
    }
    for (index, line) in lines.iter().enumerate() {
        draw_text(line, x, y + index as f32 * line_height, font_size, color);
    }
}

pub(super) fn clock_text_12h(day_clock_seconds: f32, full_day_seconds: f32) -> String {
    let total_minutes = ((day_clock_seconds / full_day_seconds) * 24.0 * 60.0) as i32;
    let hour_24 = (total_minutes / 60).rem_euclid(24);
    let minute = total_minutes.rem_euclid(60);
    let period = if hour_24 < 12 {
        ui_copy("hud_time_period_am")
    } else {
        ui_copy("hud_time_period_pm")
    };
    let hour_12 = match hour_24 % 12 {
        0 => 12,
        hour => hour,
    };
    format!("{hour_12:02}:{minute:02} {period}")
}

pub(super) fn title_case_label(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}
