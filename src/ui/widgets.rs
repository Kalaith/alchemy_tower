use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use super::truncate_text_to_width;
use crate::input::mouse_position_vec;

#[path = "widgets_selection_card.rs"]
mod widgets_selection_card;

pub(crate) use self::widgets_selection_card::draw_selection_card;

pub(crate) fn draw_action_button(rect: Rect, label: &str, label_offset_x: f32) {
    let hovered = rect.contains(mouse_position_vec());
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

pub(crate) fn draw_state_banner(x: f32, y: f32, w: f32, text: &str, is_locked: bool) {
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
