use super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_banner_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 38.0, rect.y - 7.0, rect.w + 76.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        15.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_beveled_rect(back, 15.0, Color::from_rgba(139, 101, 58, 172));
    draw_panel_texture(back, 15.0, Color::from_rgba(139, 101, 58, 172), 0.86);
    draw_beveled_rect_lines(back, 15.0, 1.6, Color::from_rgba(244, 204, 128, 196));
    draw_beveled_rect_lines(
        Rect::new(back.x + 8.0, back.y + 8.0, back.w - 16.0, back.h - 16.0),
        10.0,
        1.0,
        Color::from_rgba(61, 40, 24, 114),
    );
    draw_banner_wing(vec2(back.x + 9.0, back.y + back.h * 0.5), -1.0);
    draw_banner_wing(vec2(back.x + back.w - 9.0, back.y + back.h * 0.5), 1.0);
    draw_leaf_cluster_scaled(vec2(back.x + 66.0, back.y + 5.0), false, 0.36);
    draw_leaf_cluster_scaled(vec2(back.x + back.w - 66.0, back.y + 5.0), true, 0.36);
}

pub(super) fn draw_banner_wing(center: Vec2, direction: f32) {
    let outer = Color::from_rgba(167, 119, 61, 168);
    let trim = Color::from_rgba(242, 205, 126, 190);
    draw_triangle(
        center + vec2(direction * 2.0, -27.0),
        center + vec2(direction * 52.0, -11.0),
        center + vec2(direction * 2.0, 0.0),
        outer,
    );
    draw_triangle(
        center + vec2(direction * 2.0, 27.0),
        center + vec2(direction * 52.0, 11.0),
        center + vec2(direction * 2.0, 0.0),
        outer,
    );
    draw_line(
        center.x + direction * 7.0,
        center.y - 21.0,
        center.x + direction * 43.0,
        center.y - 8.0,
        1.3,
        trim,
    );
    draw_line(
        center.x + direction * 7.0,
        center.y + 21.0,
        center.x + direction * 43.0,
        center.y + 8.0,
        1.3,
        trim,
    );
    draw_small_diamond(center + vec2(direction * 18.0, 0.0), trim);
}
