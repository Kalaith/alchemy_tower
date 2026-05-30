use macroquad::prelude::*;

pub(super) fn draw_beveled_rect(rect: Rect, bevel: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    draw_rectangle(rect.x + bevel, rect.y, rect.w - bevel * 2.0, rect.h, color);
    draw_rectangle(rect.x, rect.y + bevel, rect.w, rect.h - bevel * 2.0, color);
    for (a, b, c) in [
        (
            vec2(rect.x + bevel, rect.y),
            vec2(rect.x, rect.y + bevel),
            vec2(rect.x + bevel, rect.y + bevel),
        ),
        (
            vec2(rect.x + rect.w - bevel, rect.y),
            vec2(rect.x + rect.w, rect.y + bevel),
            vec2(rect.x + rect.w - bevel, rect.y + bevel),
        ),
        (
            vec2(rect.x + rect.w, rect.y + rect.h - bevel),
            vec2(rect.x + rect.w - bevel, rect.y + rect.h),
            vec2(rect.x + rect.w - bevel, rect.y + rect.h - bevel),
        ),
        (
            vec2(rect.x, rect.y + rect.h - bevel),
            vec2(rect.x + bevel, rect.y + rect.h),
            vec2(rect.x + bevel, rect.y + rect.h - bevel),
        ),
    ] {
        draw_triangle(a, b, c, color);
    }
}

pub(super) fn draw_beveled_rect_lines(rect: Rect, bevel: f32, thickness: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    let points = [
        vec2(rect.x + bevel, rect.y),
        vec2(rect.x + rect.w - bevel, rect.y),
        vec2(rect.x + rect.w, rect.y + bevel),
        vec2(rect.x + rect.w, rect.y + rect.h - bevel),
        vec2(rect.x + rect.w - bevel, rect.y + rect.h),
        vec2(rect.x + bevel, rect.y + rect.h),
        vec2(rect.x, rect.y + rect.h - bevel),
        vec2(rect.x, rect.y + bevel),
    ];
    for index in 0..points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        draw_line(start.x, start.y, end.x, end.y, thickness, color);
    }
}
