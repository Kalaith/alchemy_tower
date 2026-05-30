use super::hud_gem_icons::draw_small_diamond;
use macroquad::prelude::*;

pub(super) fn draw_side_status_hardware(rect: Rect) {
    let rail = Color::from_rgba(238, 196, 119, 116);
    let dark = Color::from_rgba(0, 0, 0, 82);
    for x in [rect.x - 5.0, rect.x + rect.w + 5.0] {
        draw_line(
            x + 1.0,
            rect.y + 25.0,
            x + 1.0,
            rect.y + rect.h - 24.0,
            2.0,
            dark,
        );
        draw_line(x, rect.y + 25.0, x, rect.y + rect.h - 24.0, 1.1, rail);
        draw_circle_lines(x, rect.y + 22.0, 5.0, 1.0, rail);
        draw_circle_lines(x, rect.y + rect.h - 21.0, 5.0, 1.0, rail);
    }

    for point in [
        vec2(rect.x + rect.w * 0.5, rect.y + 4.0),
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h - 4.0),
    ] {
        draw_poly(
            point.x + 1.0,
            point.y + 2.0,
            4,
            7.0,
            45.0,
            Color::from_rgba(0, 0, 0, 72),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            7.0,
            45.0,
            Color::from_rgba(242, 205, 126, 174),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            4.0,
            45.0,
            Color::from_rgba(52, 130, 124, 148),
        );
    }

    for point in [
        vec2(rect.x + 13.0, rect.y + 13.0),
        vec2(rect.x + rect.w - 13.0, rect.y + 13.0),
        vec2(rect.x + rect.w - 13.0, rect.y + rect.h - 13.0),
        vec2(rect.x + 13.0, rect.y + rect.h - 13.0),
    ] {
        draw_circle(point.x, point.y, 2.2, Color::from_rgba(242, 205, 126, 146));
        draw_circle(
            point.x - 0.7,
            point.y - 0.7,
            0.8,
            Color::from_rgba(255, 238, 182, 160),
        );
    }
}

pub(super) fn draw_status_icon_medallion(center: Vec2, tint: Color) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        16.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_circle(center.x, center.y, 15.0, Color::from_rgba(91, 62, 36, 166));
    draw_circle(center.x, center.y, 11.0, tint);
    draw_circle_lines(
        center.x,
        center.y,
        15.0,
        1.1,
        Color::from_rgba(242, 205, 126, 146),
    );
    draw_circle_lines(
        center.x,
        center.y,
        9.0,
        0.8,
        Color::from_rgba(255, 238, 181, 72),
    );
}

pub(super) fn draw_side_status_divider(rect: Rect, y: f32) {
    let color = Color::from_rgba(221, 174, 91, 92);
    let center = rect.x + rect.w * 0.5;
    draw_line(rect.x + 13.0, y, center - 7.0, y, 1.0, color);
    draw_line(center + 7.0, y, rect.x + rect.w - 13.0, y, 1.0, color);
    draw_small_diamond(vec2(center, y), Color::from_rgba(242, 205, 126, 128));
    draw_circle_lines(center - 18.0, y, 3.0, 0.8, color);
    draw_circle_lines(center + 18.0, y, 3.0, 0.8, color);
}
