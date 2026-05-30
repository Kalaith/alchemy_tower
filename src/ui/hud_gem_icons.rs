use super::hud_palette::brass_light;
use macroquad::prelude::*;

pub(super) fn draw_gem(center: Vec2, radius: f32) {
    draw_poly(
        center.x,
        center.y,
        4,
        radius,
        45.0,
        Color::from_rgba(73, 213, 220, 240),
    );
    draw_poly(
        center.x,
        center.y,
        4,
        radius - 4.0,
        45.0,
        Color::from_rgba(185, 255, 244, 230),
    );
    draw_circle_lines(center.x, center.y, radius + 3.0, 1.5, brass_light());
}

pub(super) fn draw_small_diamond(center: Vec2, color: Color) {
    draw_poly(center.x, center.y, 4, 6.0, 45.0, color);
}
