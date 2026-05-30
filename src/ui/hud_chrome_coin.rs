use super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_coin_chip_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 8.0, rect.y - 7.0, rect.w + 16.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 4.0, back.y + 6.0, back.w, back.h),
        12.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 12.0, Color::from_rgba(119, 80, 39, 136));
    draw_panel_texture(back, 12.0, Color::from_rgba(119, 80, 39, 136), 0.66);
    draw_beveled_rect_lines(back, 12.0, 1.1, Color::from_rgba(235, 196, 118, 130));
}

pub(super) fn draw_coin_chip_connector(rect: Rect) {
    let y = rect.y + rect.h * 0.5;
    let color = Color::from_rgba(242, 205, 126, 118);
    draw_line(
        rect.x - 34.0,
        y + 2.0,
        rect.x + 9.0,
        y + 2.0,
        3.0,
        Color::from_rgba(0, 0, 0, 62),
    );
    draw_line(rect.x - 34.0, y, rect.x + 9.0, y, 1.5, color);
    draw_circle_lines(rect.x - 18.0, y, 9.0, 1.2, color);
    draw_small_diamond(vec2(rect.x - 2.0, y), Color::from_rgba(91, 223, 205, 130));
    draw_leaf_cluster_scaled(vec2(rect.x - 31.0, y + 14.0), false, 0.34);
}

pub(super) fn draw_coin_face(center: Vec2) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        15.0,
        Color::from_rgba(0, 0, 0, 70),
    );
    draw_circle(
        center.x,
        center.y,
        14.0,
        Color::from_rgba(208, 151, 50, 255),
    );
    draw_circle(
        center.x - 2.0,
        center.y - 2.0,
        10.0,
        Color::from_rgba(235, 181, 72, 230),
    );
    draw_circle_lines(
        center.x,
        center.y,
        14.0,
        2.0,
        Color::from_rgba(255, 229, 148, 230),
    );
    draw_circle_lines(
        center.x,
        center.y,
        8.5,
        0.9,
        Color::from_rgba(118, 75, 30, 128),
    );
    draw_line(
        center.x - 4.5,
        center.y - 6.0,
        center.x + 4.5,
        center.y - 6.0,
        1.0,
        Color::from_rgba(255, 236, 166, 210),
    );
    draw_line(
        center.x,
        center.y - 5.0,
        center.x,
        center.y + 6.0,
        1.0,
        Color::from_rgba(118, 75, 30, 128),
    );
    draw_circle(
        center.x - 4.0,
        center.y - 5.0,
        3.0,
        Color::from_rgba(255, 236, 166, 210),
    );
}
