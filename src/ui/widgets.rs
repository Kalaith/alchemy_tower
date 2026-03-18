use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub fn draw_action_button(rect: Rect, label: &str, label_offset_x: f32) {
    let hovered = rect.contains(mouse_position().into());
    let fill = if hovered { dark::HOVERED } else { dark::ACCENT };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_text(
        label,
        rect.x + label_offset_x,
        rect.y + 30.0,
        28.0,
        dark::TEXT_BRIGHT,
    );
}

pub fn draw_state_banner(x: f32, y: f32, w: f32, text: &str, is_locked: bool) {
    let color = if is_locked {
        Color::from_rgba(120, 72, 72, 255)
    } else {
        Color::from_rgba(68, 74, 88, 255)
    };
    draw_rectangle(x, y, w, 32.0, color);
    draw_rectangle_lines(x, y, w, 32.0, 2.0, dark::ACCENT);
    draw_text(text, x + 10.0, y + 21.0, 18.0, dark::TEXT_BRIGHT);
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
    draw_text(title, x + 12.0, y + 22.0, 22.0, text_color);
    if !subtitle.is_empty() {
        draw_text(subtitle, x + 12.0, y + 42.0, 18.0, dark::TEXT_DIM);
    }
    if !meta.is_empty() {
        let measured = measure_text(meta, None, 18, 1.0);
        draw_text(
            meta,
            x + w - measured.width - 12.0,
            y + 22.0,
            18.0,
            dark::TEXT_DIM,
        );
    }
}
