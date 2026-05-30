use super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_ornate_divider(x: f32, y: f32, width: f32, opacity: f32) {
    let color = Color::from_rgba(220, 182, 109, (132.0 * opacity) as u8);
    let center = x + width * 0.5;
    draw_line(x, y, center - 12.0, y, 1.0, color);
    draw_line(center + 12.0, y, x + width, y, 1.0, color);
    draw_small_diamond(
        vec2(center, y),
        Color::from_rgba(242, 205, 126, (176.0 * opacity) as u8),
    );
}

pub(super) fn draw_panel_filigree(rect: Rect, opacity: f32) {
    let color = Color::from_rgba(242, 205, 126, (112.0 * opacity) as u8);
    let dark = Color::from_rgba(46, 30, 18, (120.0 * opacity) as u8);
    let corner_gap = 14.0;
    for (x, y, sx, sy) in [
        (rect.x + corner_gap, rect.y + corner_gap, 1.0, 1.0),
        (rect.x + rect.w - corner_gap, rect.y + corner_gap, -1.0, 1.0),
        (rect.x + corner_gap, rect.y + rect.h - corner_gap, 1.0, -1.0),
        (
            rect.x + rect.w - corner_gap,
            rect.y + rect.h - corner_gap,
            -1.0,
            -1.0,
        ),
    ] {
        draw_circle_lines(x + sx * 8.0, y + sy * 8.0, 7.0, 1.0, color);
        draw_line(x, y + sy * 13.0, x + sx * 22.0, y + sy * 13.0, 1.0, color);
        draw_line(x + sx * 13.0, y, x + sx * 13.0, y + sy * 22.0, 1.0, color);
        draw_circle(x + sx * 8.0, y + sy * 8.0, 1.3, dark);
    }

    if rect.h > 80.0 {
        draw_panel_side_knot(vec2(rect.x + 2.0, rect.y + rect.h * 0.5), 1.0, opacity);
        draw_panel_side_knot(
            vec2(rect.x + rect.w - 2.0, rect.y + rect.h * 0.5),
            -1.0,
            opacity,
        );
    }

    if rect.w > 150.0 {
        let top = vec2(rect.x + rect.w * 0.5, rect.y + 2.0);
        let bottom = vec2(rect.x + rect.w * 0.5, rect.y + rect.h - 2.0);
        draw_small_diamond(
            top,
            Color::from_rgba(242, 205, 126, (154.0 * opacity) as u8),
        );
        draw_small_diamond(
            bottom,
            Color::from_rgba(242, 205, 126, (122.0 * opacity) as u8),
        );
    }
}

pub(super) fn draw_panel_side_knot(center: Vec2, direction: f32, opacity: f32) {
    let color = Color::from_rgba(242, 205, 126, (130.0 * opacity) as u8);
    let glow = Color::from_rgba(87, 214, 199, (72.0 * opacity) as u8);
    draw_poly(center.x, center.y, 4, 5.5, 45.0, color);
    draw_circle_lines(center.x + direction * 10.0, center.y, 7.0, 1.0, color);
    draw_line(
        center.x + direction * 3.0,
        center.y,
        center.x + direction * 22.0,
        center.y,
        1.0,
        color,
    );
    draw_circle(center.x + direction * 10.0, center.y, 2.0, glow);
}
