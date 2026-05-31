use super::hud_primitives::*;
use macroquad::prelude::*;

#[path = "hud_chrome_time_panel.rs"]
mod hud_chrome_time_panel;

pub(super) fn draw_goal_note_hardware(rect: Rect, has_icon: bool) {
    let sheet_width = if has_icon {
        rect.w - 112.0
    } else {
        rect.w - 32.0
    };
    let sheet = Rect::new(rect.x + 15.0, rect.y + 89.0, sheet_width, 75.0);
    draw_beveled_rect(sheet, 7.0, Color::from_rgba(76, 60, 39, 74));
    draw_beveled_rect_lines(sheet, 7.0, 0.9, Color::from_rgba(242, 205, 126, 58));
    draw_panel_texture(sheet, 7.0, Color::from_rgba(76, 60, 39, 74), 0.54);

    let margin_x = rect.x + 18.0;
    draw_line(
        margin_x,
        rect.y + 52.0,
        margin_x,
        rect.y + rect.h - 18.0,
        1.0,
        Color::from_rgba(242, 205, 126, 80),
    );
    for index in 0..6 {
        let y = rect.y + 61.0 + index as f32 * 22.0;
        draw_circle(margin_x, y, 1.8, Color::from_rgba(242, 205, 126, 128));
        draw_circle(margin_x, y, 0.8, Color::from_rgba(48, 33, 21, 154));
    }

    for point in [
        vec2(rect.x + 31.0, rect.y + 52.0),
        vec2(rect.x + rect.w - 31.0, rect.y + 52.0),
        vec2(rect.x + 31.0, rect.y + rect.h - 18.0),
        vec2(rect.x + rect.w - 31.0, rect.y + rect.h - 18.0),
    ] {
        draw_poly(
            point.x + 1.0,
            point.y + 2.0,
            4,
            4.5,
            45.0,
            Color::from_rgba(0, 0, 0, 74),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            4.5,
            45.0,
            Color::from_rgba(242, 205, 126, 142),
        );
    }

    draw_line(
        rect.x + rect.w - 18.0,
        rect.y + 55.0,
        rect.x + rect.w - 18.0,
        rect.y + rect.h - 22.0,
        0.8,
        Color::from_rgba(84, 218, 198, 58),
    );
    draw_leaf_cluster_scaled(
        vec2(rect.x + rect.w - 31.0, rect.y + rect.h - 30.0),
        true,
        0.34,
    );
}

pub(super) fn draw_goal_action_strip(rect: Rect) {
    let strip = Rect::new(rect.x + 16.0, rect.y + rect.h - 29.0, rect.w - 32.0, 22.0);
    draw_beveled_rect(
        Rect::new(strip.x + 2.0, strip.y + 3.0, strip.w, strip.h),
        6.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_beveled_rect(strip, 6.0, Color::from_rgba(44, 38, 30, 142));
    draw_beveled_rect_lines(strip, 6.0, 0.9, Color::from_rgba(242, 205, 126, 82));
    draw_small_diamond(
        vec2(strip.x + strip.w - 11.0, strip.y + strip.h * 0.5),
        Color::from_rgba(91, 223, 205, 122),
    );
}

pub(super) fn draw_time_panel_hardware(rect: Rect) {
    hud_chrome_time_panel::draw_time_panel_hardware(rect);
}
