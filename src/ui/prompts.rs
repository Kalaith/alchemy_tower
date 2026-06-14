use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

#[path = "prompt_chrome.rs"]
mod prompt_chrome;
#[path = "prompt_shapes.rs"]
mod prompt_shapes;

use self::prompt_chrome::*;
use self::prompt_shapes::*;

pub(crate) fn draw_interaction_prompt(text: &str) {
    let (key, label) = split_prompt(text);
    let label_width = measure_ui_text(label, None, 22, 1.0).width;
    let width = (label_width + 178.0).clamp(320.0, 470.0);
    let x = screen_width() - width - 24.0;
    let y = screen_height() - 78.0;
    let center_y = y + 28.0;
    let label_rect = Rect::new(x + 18.0, y + 9.0, width - 100.0, 40.0);
    let key_center = vec2(x + width - 42.0, center_y);

    draw_prompt_backplate(label_rect, key_center);
    draw_prompt_flourish(x, center_y, width);
    draw_beveled_rect(
        Rect::new(
            label_rect.x + 5.0,
            label_rect.y + 6.0,
            label_rect.w,
            label_rect.h,
        ),
        9.0,
        Color::from_rgba(0, 0, 0, 86),
    );
    draw_beveled_rect(label_rect, 9.0, Color::from_rgba(25, 24, 23, 184));
    draw_beveled_rect_lines(label_rect, 9.0, 1.6, Color::from_rgba(221, 177, 96, 178));
    draw_beveled_rect_lines(
        Rect::new(
            label_rect.x + 4.0,
            label_rect.y + 4.0,
            label_rect.w - 8.0,
            label_rect.h - 8.0,
        ),
        6.0,
        0.8,
        Color::from_rgba(249, 224, 158, 86),
    );
    draw_ui_text(
        &truncate_to_width(label, width - 118.0, 22.0),
        label_rect.x + 19.0,
        y + 35.0,
        22.0,
        Color::from_rgba(246, 238, 213, 255),
    );

    if !key.is_empty() {
        let key_rect = Rect::new(key_center.x - 26.0, key_center.y - 26.0, 52.0, 52.0);
        draw_key_medallion(key_center);
        draw_centered_text(key, key_rect.x, key_rect.y + 32.0, key_rect.w, 18.0);
    }
}

fn split_prompt(text: &str) -> (&str, &str) {
    if let Some((raw_key, label)) = text.split_once(": ") {
        let key = raw_key.split('/').next().unwrap_or(raw_key).trim();
        (key, label.trim())
    } else {
        ("", text)
    }
}

fn draw_centered_text(text: &str, x: f32, baseline_y: f32, width: f32, font_size: f32) {
    let measured = measure_ui_text(text, None, font_size as u16, 1.0);
    draw_ui_text(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        Color::from_rgba(246, 238, 213, 255),
    );
}

fn truncate_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    macroquad_toolkit::ui::truncate_text_to_width(text, max_width, font_size)
}
