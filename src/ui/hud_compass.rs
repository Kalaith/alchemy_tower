use super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_compass_backplate(center: Vec2, radius: f32) {
    let brass_shadow = Color::from_rgba(0, 0, 0, 84);
    let warm = Color::from_rgba(112, 76, 41, 138);
    draw_circle(center.x + 5.0, center.y + 8.0, radius + 12.0, brass_shadow);
    draw_circle(center.x, center.y, radius + 9.0, warm);
    draw_circle_lines(
        center.x,
        center.y,
        radius + 9.0,
        1.5,
        Color::from_rgba(235, 196, 118, 138),
    );
    for angle in [
        0.0_f32,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    ] {
        let point = center + vec2(angle.cos(), angle.sin()) * (radius + 5.0);
        draw_small_diamond(point, Color::from_rgba(242, 205, 126, 190));
    }
}

pub(super) fn draw_compass_ticks(center: Vec2, radius: f32) {
    for index in 0..32 {
        let angle = index as f32 * std::f32::consts::TAU / 32.0;
        let is_cardinal = index % 8 == 0;
        let inner_radius = if is_cardinal {
            radius - 18.0
        } else {
            radius - 12.0
        };
        let inner = center + vec2(angle.cos(), angle.sin()) * inner_radius;
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius - 6.0);
        let alpha = if is_cardinal { 142 } else { 78 };
        draw_line(
            inner.x,
            inner.y,
            outer.x,
            outer.y,
            if is_cardinal { 1.4 } else { 0.8 },
            Color::from_rgba(232, 198, 129, alpha),
        );
    }
}

pub(super) fn draw_compass_map_texture(center: Vec2, radius: f32) {
    for index in 0..9 {
        let y = center.y - 36.0 + index as f32 * 9.0;
        let width = (radius - 22.0) * (1.0 - ((y - center.y).abs() / radius).min(0.84));
        draw_line(
            center.x - width,
            y,
            center.x + width,
            y + if index % 2 == 0 { 1.5 } else { -1.0 },
            0.7,
            Color::from_rgba(236, 211, 162, 26),
        );
    }
    for index in 0..5 {
        let x = center.x - 28.0 + index as f32 * 14.0;
        draw_line(
            x,
            center.y - 35.0,
            x + 4.0,
            center.y + 36.0,
            0.6,
            Color::from_rgba(52, 34, 23, 34),
        );
    }
}

pub(super) fn draw_compass_rosette(center: Vec2) {
    let teal = Color::from_rgba(91, 223, 205, 116);
    let brass = Color::from_rgba(242, 205, 126, 166);
    draw_circle(center.x, center.y, 12.0, Color::from_rgba(28, 24, 20, 110));
    for angle in [
        0.0_f32,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    ] {
        let tip = center + vec2(angle.cos(), angle.sin()) * 14.0;
        let left = center + vec2((angle + 2.45).cos(), (angle + 2.45).sin()) * 5.0;
        let right = center + vec2((angle - 2.45).cos(), (angle - 2.45).sin()) * 5.0;
        draw_triangle(tip, left, right, brass);
    }
    draw_circle(center.x, center.y, 4.0, teal);
    draw_circle_lines(
        center.x,
        center.y,
        12.0,
        1.0,
        Color::from_rgba(242, 205, 126, 148),
    );
}
