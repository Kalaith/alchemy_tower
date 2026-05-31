use super::super::hud_palette::*;
use macroquad::prelude::*;

pub(super) fn draw_slot_corner_dots(rect: Rect, glow: Color) {
    let color = Color::new(glow.r, glow.g, glow.b, 0.34);
    for point in [
        vec2(rect.x + 7.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 7.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 7.0, rect.y + rect.h - 7.0),
        vec2(rect.x + 7.0, rect.y + rect.h - 7.0),
    ] {
        draw_circle(point.x, point.y, 1.5, color);
    }
}

pub(super) fn draw_glass_potion_bottle(rect: Rect, liquid: Color, alpha: f32) {
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 16.0;
    let glass = Color::new(parchment().r, parchment().g, parchment().b, alpha * 0.9);
    let outline = Color::new(
        brass_light().r,
        brass_light().g,
        brass_light().b,
        alpha * 0.8,
    );
    let liquid_color = Color::new(liquid.r, liquid.g, liquid.b, alpha);

    draw_rectangle(cx - 5.0, top, 10.0, 13.0, glass);
    draw_rectangle(
        cx - 8.0,
        top - 3.0,
        16.0,
        5.0,
        Color::new(0.44, 0.33, 0.24, alpha),
    );
    draw_rectangle(cx - 13.0, top + 15.0, 26.0, 21.0, glass);
    draw_circle(cx, top + 38.0, 14.0, glass);
    draw_rectangle(cx - 10.0, top + 29.0, 20.0, 11.0, liquid_color);
    draw_circle(cx, top + 38.0, 10.5, liquid_color);
    draw_circle(
        cx - 5.0,
        top + 26.0,
        3.0,
        Color::new(1.0, 1.0, 1.0, alpha * 0.72),
    );
    draw_line(cx - 9.0, top + 17.0, cx - 12.0, top + 35.0, 1.2, outline);
    draw_line(cx + 9.0, top + 17.0, cx + 12.0, top + 35.0, 1.2, outline);
    draw_circle_lines(cx, top + 38.0, 14.0, 1.2, outline);
}

pub(super) fn draw_slot_sparkle(rect: Rect, color: Color, alpha: f32) {
    let center = vec2(rect.x + rect.w - 12.0, rect.y + 15.0);
    let tint = Color::new(color.r, color.g, color.b, alpha);
    draw_line(
        center.x - 5.0,
        center.y,
        center.x + 5.0,
        center.y,
        1.0,
        tint,
    );
    draw_line(
        center.x,
        center.y - 5.0,
        center.x,
        center.y + 5.0,
        1.0,
        tint,
    );
    draw_circle(center.x, center.y, 1.5, bright_ink());
}
