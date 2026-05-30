use macroquad::prelude::*;

pub(super) fn draw_world_marker_plate(text: &str, center: Vec2, color: Color, priority: bool) {
    let font_size = if priority { 16.0 } else { 15.0 };
    let measured = measure_text(text, None, font_size as u16, 1.0);
    let width = (measured.width + 26.0).clamp(92.0, 190.0);
    let height = if priority { 24.0 } else { 22.0 };
    let rect = Rect::new(
        center.x - width * 0.5,
        center.y - height + 7.0,
        width,
        height,
    );
    let fill = if priority {
        Color::from_rgba(25, 35, 31, 168)
    } else {
        Color::from_rgba(23, 21, 18, 142)
    };
    let border = if priority {
        Color::from_rgba(99, 230, 205, 172)
    } else {
        Color::from_rgba(226, 184, 105, 132)
    };

    draw_marker_beveled_rect(
        Rect::new(rect.x + 2.0, rect.y + 3.0, rect.w, rect.h),
        6.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_marker_beveled_rect(rect, 6.0, fill);
    draw_marker_beveled_lines(rect, 6.0, 1.0, border);
    draw_marker_diamond(vec2(rect.x + 8.0, rect.y + rect.h * 0.5), border);
    draw_marker_diamond(vec2(rect.x + rect.w - 8.0, rect.y + rect.h * 0.5), border);
    draw_text(
        text,
        center.x - measured.width * 0.5,
        rect.y + rect.h - 6.0,
        font_size,
        color,
    );
}

fn draw_marker_beveled_rect(rect: Rect, bevel: f32, color: Color) {
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

fn draw_marker_beveled_lines(rect: Rect, bevel: f32, thickness: f32, color: Color) {
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

fn draw_marker_diamond(center: Vec2, color: Color) {
    draw_poly(center.x, center.y, 4, 3.3, 45.0, color);
}
