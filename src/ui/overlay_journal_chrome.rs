use crate::art::{draw_texture_centered, ArtAssets};
use crate::input::mouse_position_vec;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_journal_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 150),
    );
}

pub(crate) fn draw_journal_close_button(close_rect: Rect, close_label: &str) {
    let close_hovered = close_rect.contains(mouse_position_vec());
    draw_rectangle(
        close_rect.x,
        close_rect.y,
        close_rect.w,
        close_rect.h,
        if close_hovered {
            dark::ACCENT
        } else {
            Color::from_rgba(38, 40, 50, 255)
        },
    );
    draw_rectangle_lines(
        close_rect.x,
        close_rect.y,
        close_rect.w,
        close_rect.h,
        2.0,
        if close_hovered { WHITE } else { dark::ACCENT },
    );
    draw_text(
        close_label,
        close_rect.x + 18.0,
        close_rect.y + 19.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}

pub(crate) fn draw_journal_current_conditions(current_conditions_text: &str, x: f32, y: f32) {
    draw_text(
        current_conditions_text,
        x + 20.0,
        y + 50.0,
        24.0,
        dark::TEXT_BRIGHT,
    );
}

pub(crate) fn draw_journal_tabs(
    tabs: &[&str],
    selected_index: usize,
    tab_rects: &[Rect],
    art: &ArtAssets,
) {
    for (index, tab) in tabs.iter().enumerate() {
        let Some(rect) = tab_rects.get(index) else {
            break;
        };
        let selected = selected_index == index;
        let hovered = rect.contains(mouse_position_vec());
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if selected || hovered {
                dark::ACCENT
            } else {
                Color::from_rgba(38, 40, 50, 255)
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.0,
            if selected || hovered {
                WHITE
            } else {
                dark::ACCENT
            },
        );
        draw_text(
            tab,
            rect.x + 34.0,
            rect.y + 20.0,
            18.0,
            if selected {
                dark::TEXT_BRIGHT
            } else {
                dark::TEXT_DIM
            },
        );
        if let Some(texture) = art.journal_tab_by_label(tab) {
            draw_texture_centered(
                texture,
                vec2(rect.x + 18.0, rect.y + 14.0),
                vec2(18.0, 18.0),
                WHITE,
            );
        }
    }
}

pub(crate) fn draw_journal_footer(footer_text: &str, x: f32, y: f32, h: f32) {
    draw_text(footer_text, x + 20.0, y + h - 20.0, 18.0, dark::TEXT_DIM);
}
