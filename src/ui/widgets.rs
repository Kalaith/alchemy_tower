use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use crate::ui::truncate_text_to_width;

pub fn draw_action_button(rect: Rect, label: &str, label_offset_x: f32) {
    let hovered = rect.contains(mouse_position().into());
    let fill = if hovered {
        Color::from_rgba(34, 40, 52, 220)
    } else {
        Color::from_rgba(18, 22, 30, 180)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        if hovered {
            Color::from_rgba(255, 238, 196, 112)
        } else {
            Color::from_rgba(160, 170, 190, 72)
        },
    );
    let safe = truncate_text_to_width(label, rect.w - 20.0, 24.0);
    let measured = measure_text(&safe, None, 24, 1.0);
    let x = rect.x + ((rect.w - measured.width) * 0.5).max(label_offset_x.min(rect.w - 20.0));
    draw_text(
        &safe,
        x,
        rect.y + 29.0,
        24.0,
        Color::from_rgba(240, 236, 228, 255),
    );
}

pub fn draw_state_banner(x: f32, y: f32, w: f32, text: &str, is_locked: bool) {
    let color = if is_locked {
        Color::from_rgba(54, 36, 40, 194)
    } else {
        Color::from_rgba(20, 24, 32, 184)
    };
    draw_rectangle(x, y, w, 36.0, color);
    draw_rectangle(
        x,
        y,
        6.0,
        36.0,
        if is_locked {
            Color::from_rgba(255, 214, 132, 164)
        } else {
            Color::from_rgba(176, 226, 255, 136)
        },
    );
    draw_rectangle_lines(x, y, w, 36.0, 1.5, Color::from_rgba(160, 170, 190, 62));
    draw_text(
        &truncate_text_to_width(text, w - 20.0, 18.0),
        x + 16.0,
        y + 23.0,
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
        Color::from_rgba(28, 34, 44, 214)
    } else {
        Color::from_rgba(16, 18, 26, 154)
    };
    let text_color = if enabled {
        dark::TEXT_BRIGHT
    } else {
        dark::TEXT_DIM
    };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle(
        x,
        y,
        4.0,
        h,
        if selected {
            Color::from_rgba(176, 226, 255, 144)
        } else if enabled {
            Color::from_rgba(255, 214, 132, 92)
        } else {
            Color::from_rgba(102, 108, 120, 82)
        },
    );
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        1.5,
        if selected {
            Color::from_rgba(176, 226, 255, 92)
        } else {
            Color::from_rgba(160, 170, 190, 54)
        },
    );
    let meta_width = if meta.is_empty() {
        0.0
    } else {
        measure_text(meta, None, 16, 1.0)
            .width
            .min((w * 0.34).max(56.0))
    };
    let title_width = (w - 30.0 - meta_width).max(80.0);
    draw_text(
        &truncate_text_to_width(title, title_width, 20.0),
        x + 18.0,
        y + 22.0,
        20.0,
        text_color,
    );
    if !subtitle.is_empty() {
        draw_text(
            &truncate_text_to_width(subtitle, w - 24.0, 16.0),
            x + 18.0,
            y + 42.0,
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
