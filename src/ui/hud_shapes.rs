use super::hud_palette::*;
use macroquad::prelude::*;

pub(super) fn draw_ornate_panel(rect: Rect, fill: Color, opacity: f32) {
    let bevel = rect.w.min(rect.h).min(18.0) * 0.45;
    draw_beveled_rect(
        Rect::new(rect.x + 6.0, rect.y + 8.0, rect.w, rect.h),
        bevel,
        Color::new(0.0, 0.0, 0.0, 0.28 * opacity),
    );
    draw_beveled_rect(rect, bevel, fill);
    draw_panel_texture(rect, bevel, fill, opacity);
    draw_beveled_rect_lines(
        rect,
        bevel,
        2.0,
        Color::new(brass().r, brass().g, brass().b, opacity),
    );
    draw_beveled_rect_lines(
        Rect::new(rect.x + 4.0, rect.y + 4.0, rect.w - 8.0, rect.h - 8.0),
        (bevel - 3.0).max(4.0),
        1.0,
        Color::from_rgba(255, 239, 184, (58.0 * opacity) as u8),
    );
    draw_corner_marks(rect, opacity);
}

pub(super) fn draw_panel_texture(rect: Rect, bevel: f32, fill: Color, opacity: f32) {
    let warm_fill = fill.r > fill.b;
    let light = if warm_fill {
        Color::from_rgba(255, 222, 159, (24.0 * opacity) as u8)
    } else {
        Color::from_rgba(255, 232, 176, (18.0 * opacity) as u8)
    };
    let dark = Color::from_rgba(0, 0, 0, (26.0 * opacity) as u8);
    let inset = (bevel + 4.0).min(rect.w * 0.18).min(rect.h * 0.28);
    let rows = ((rect.h / 18.0).ceil() as usize).clamp(1, 8);

    for row in 0..rows {
        let y = rect.y + inset + 5.0 + row as f32 * 14.0;
        if y > rect.y + rect.h - inset {
            break;
        }
        let offset = (row % 3) as f32 * 7.0;
        let x1 = rect.x + inset + offset;
        let x2 = rect.x + rect.w - inset - 5.0 - (row % 2) as f32 * 10.0;
        draw_line(x1, y, x2, y + 0.7, 1.0, light);
        if row % 2 == 0 {
            draw_line(x1 + 8.0, y + 3.0, x2 - 16.0, y + 3.6, 1.0, dark);
        }
    }

    let scuffs = ((rect.w / 78.0).ceil() as usize).clamp(1, 6);
    for scuff in 0..scuffs {
        let x = rect.x + inset + 14.0 + scuff as f32 * 66.0;
        if x > rect.x + rect.w - inset - 16.0 {
            break;
        }
        let y = rect.y + rect.h - inset - 10.0 - (scuff % 2) as f32 * 12.0;
        draw_line(x, y, x + 18.0, y - 4.0, 1.0, dark);
    }
}

pub(super) fn draw_corner_marks(rect: Rect, opacity: f32) {
    let color = Color::from_rgba(242, 202, 126, (168.0 * opacity) as u8);
    let len = 16.0;
    for (x, y, sx, sy) in [
        (rect.x, rect.y, 1.0, 1.0),
        (rect.x + rect.w, rect.y, -1.0, 1.0),
        (rect.x, rect.y + rect.h, 1.0, -1.0),
        (rect.x + rect.w, rect.y + rect.h, -1.0, -1.0),
    ] {
        draw_line(x, y + sy * len, x + sx * len, y, 1.5, color);
        draw_circle(x + sx * 10.0, y + sy * 10.0, 2.0, color);
    }
}

pub(super) fn draw_beveled_rect(rect: Rect, bevel: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    draw_rectangle(rect.x + bevel, rect.y, rect.w - bevel * 2.0, rect.h, color);
    draw_rectangle(rect.x, rect.y + bevel, rect.w, rect.h - bevel * 2.0, color);

    let center_tl = vec2(rect.x + bevel, rect.y + bevel);
    let center_tr = vec2(rect.x + rect.w - bevel, rect.y + bevel);
    let center_br = vec2(rect.x + rect.w - bevel, rect.y + rect.h - bevel);
    let center_bl = vec2(rect.x + bevel, rect.y + rect.h - bevel);
    draw_triangle(
        vec2(rect.x + bevel, rect.y),
        vec2(rect.x, rect.y + bevel),
        center_tl,
        color,
    );
    draw_triangle(
        vec2(rect.x + rect.w - bevel, rect.y),
        vec2(rect.x + rect.w, rect.y + bevel),
        center_tr,
        color,
    );
    draw_triangle(
        vec2(rect.x + rect.w, rect.y + rect.h - bevel),
        vec2(rect.x + rect.w - bevel, rect.y + rect.h),
        center_br,
        color,
    );
    draw_triangle(
        vec2(rect.x, rect.y + rect.h - bevel),
        vec2(rect.x + bevel, rect.y + rect.h),
        center_bl,
        color,
    );
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

pub(super) fn draw_circle_arc(
    center: Vec2,
    radius: f32,
    start_degrees: f32,
    sweep_degrees: f32,
    thickness: f32,
    color: Color,
) {
    let segments = (sweep_degrees.abs() / 8.0).ceil().max(5.0) as usize;
    let start = start_degrees.to_radians();
    let sweep = sweep_degrees.to_radians();
    let mut previous = center + vec2(start.cos(), start.sin()) * radius;

    for step in 1..=segments {
        let angle = start + sweep * step as f32 / segments as f32;
        let next = center + vec2(angle.cos(), angle.sin()) * radius;
        draw_line(previous.x, previous.y, next.x, next.y, thickness, color);
        previous = next;
    }
}
