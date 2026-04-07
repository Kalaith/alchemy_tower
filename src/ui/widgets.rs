use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use crate::ui::truncate_text_to_width;

pub fn draw_action_button(rect: Rect, label: &str, label_offset_x: f32) {
    let hovered = rect.contains(mouse_position().into());
    let fill = if hovered { dark::HOVERED } else { dark::ACCENT };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    let safe = truncate_text_to_width(label, rect.w - 20.0, 24.0);
    let measured = measure_text(&safe, None, 24, 1.0);
    let x = rect.x + ((rect.w - measured.width) * 0.5).max(label_offset_x.min(rect.w - 20.0));
    draw_text(&safe, x, rect.y + 29.0, 24.0, dark::TEXT_BRIGHT);
}

pub fn draw_state_banner(x: f32, y: f32, w: f32, text: &str, is_locked: bool) {
    let color = if is_locked {
        Color::from_rgba(120, 72, 72, 255)
    } else {
        Color::from_rgba(68, 74, 88, 255)
    };
    draw_rectangle(x, y, w, 32.0, color);
    draw_rectangle_lines(x, y, w, 32.0, 2.0, dark::ACCENT);
    draw_text(
        &truncate_text_to_width(text, w - 20.0, 18.0),
        x + 10.0,
        y + 21.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}

pub fn draw_selection_card(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    selected: bool,
    enabled: bool,
    title: &str,
    subtitle: &str,
    meta: &str,
) {
    let bg = if selected {
        dark::ACCENT
    } else {
        Color::from_rgba(38, 40, 50, 255)
    };
    let text_color = if enabled {
        dark::TEXT_BRIGHT
    } else {
        dark::TEXT_DIM
    };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 2.0, if selected { WHITE } else { dark::ACCENT });
    let meta_width = if meta.is_empty() {
        0.0
    } else {
        measure_text(meta, None, 16, 1.0).width.min((w * 0.34).max(56.0))
    };
    let title_width = (w - 30.0 - meta_width).max(80.0);
    draw_text(
        &truncate_text_to_width(title, title_width, 20.0),
        x + 12.0,
        y + 21.0,
        20.0,
        text_color,
    );
    if !subtitle.is_empty() {
        draw_text(
            &truncate_text_to_width(subtitle, w - 24.0, 16.0),
            x + 12.0,
            y + 40.0,
            16.0,
            dark::TEXT_DIM,
        );
    }
    if !meta.is_empty() {
        let safe_meta = truncate_text_to_width(meta, (w * 0.34).max(56.0), 16.0);
        let measured = measure_text(&safe_meta, None, 16, 1.0);
        draw_text(
            &safe_meta,
            x + w - measured.width - 12.0,
            y + 21.0,
            16.0,
            dark::TEXT_DIM,
        );
    }
}
