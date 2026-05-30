pub(super) use super::hud_text::*;
use crate::art::{draw_texture_centered, ArtAssets};
use macroquad::prelude::*;

pub(super) fn draw_goal_item_badge(rect: Rect, icon_id: &str, amount_text: &str, art: &ArtAssets) {
    let badge = Rect::new(rect.x + rect.w - 76.0, rect.y + 100.0, 48.0, 58.0);
    draw_beveled_rect(
        Rect::new(badge.x + 3.0, badge.y + 4.0, badge.w, badge.h),
        7.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_beveled_rect(badge, 7.0, Color::from_rgba(42, 35, 29, 204));
    draw_beveled_rect_lines(badge, 7.0, 1.2, Color::from_rgba(223, 184, 111, 150));

    if let Some(texture) = art.item_icon(icon_id) {
        draw_texture_centered(
            texture,
            vec2(badge.x + badge.w * 0.5, badge.y + 25.0),
            vec2(34.0, 34.0),
            WHITE,
        );
    } else {
        draw_bottle_silhouette(Rect::new(badge.x + 7.0, badge.y + 6.0, 34.0, 34.0), 0.44);
    }

    draw_centered_text(
        amount_text,
        badge.x,
        badge.y + badge.h - 8.0,
        badge.w,
        13.0,
        parchment(),
    );
}

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

pub(super) fn draw_flourish_line(x1: f32, y1: f32, x2: f32, y2: f32, left: bool) {
    let color = Color::from_rgba(221, 177, 96, 185);
    draw_line(x1, y1, x2, y2, 2.0, color);
    let sign = if left { 1.0 } else { -1.0 };
    let curl_x = if left { x1 + 22.0 } else { x1 - 22.0 };
    draw_circle_lines(curl_x, y1, 10.0, 1.5, color);
    draw_line(x2, y2, x2 + sign * 18.0, y2 - 12.0, 1.5, color);
    draw_line(x2, y2, x2 + sign * 18.0, y2 + 12.0, 1.5, color);
}

pub(super) fn draw_title_vines(x: f32, y: f32, width: f32) {
    let color = Color::from_rgba(221, 177, 96, 150);
    draw_line(x + 42.0, y + 8.0, x + 118.0, y + 8.0, 1.5, color);
    draw_line(
        x + width - 118.0,
        y + 8.0,
        x + width - 42.0,
        y + 8.0,
        1.5,
        color,
    );
    draw_circle_lines(x + 126.0, y + 8.0, 8.0, 1.2, color);
    draw_circle_lines(x + width - 126.0, y + 8.0, 8.0, 1.2, color);
    draw_leaf_cluster_scaled(vec2(x + 58.0, y + 9.0), false, 0.42);
    draw_leaf_cluster_scaled(vec2(x + width - 58.0, y + 9.0), true, 0.42);
}

pub(super) fn draw_keycap(rect: Rect, key: &str, blue: bool) {
    let fill = if blue {
        Color::from_rgba(39, 75, 110, 235)
    } else {
        Color::from_rgba(29, 31, 36, 235)
    };
    draw_rectangle(rect.x + 2.0, rect.y + 3.0, rect.w, rect.h, shadow());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        Color::from_rgba(239, 217, 174, 190),
    );
    draw_centered_text(
        key,
        rect.x,
        rect.y + rect.h - 5.0,
        rect.w,
        15.0,
        bright_ink(),
    );
}

pub(super) fn draw_bag_icon(center: Vec2, scale: f32) {
    let fill = Color::from_rgba(205, 160, 112, 230);
    let dark = Color::from_rgba(80, 54, 34, 190);
    let w = 20.0 * scale;
    let h = 17.0 * scale;
    draw_rectangle(center.x - w * 0.5, center.y - h * 0.15, w, h, fill);
    draw_circle(center.x - w * 0.25, center.y - h * 0.15, w * 0.24, fill);
    draw_circle(center.x + w * 0.25, center.y - h * 0.15, w * 0.24, fill);
    draw_circle_lines(center.x, center.y - h * 0.25, w * 0.28, 1.2, dark);
    draw_line(
        center.x - w * 0.35,
        center.y + h * 0.25,
        center.x + w * 0.35,
        center.y + h * 0.25,
        1.0,
        dark,
    );
}

pub(super) fn draw_book_icon(center: Vec2, scale: f32) {
    let cover = Color::from_rgba(63, 95, 88, 210);
    let pages = Color::from_rgba(226, 204, 162, 190);
    let w = 20.0 * scale;
    let h = 17.0 * scale;
    draw_rectangle(center.x - w * 0.5, center.y - h * 0.5, w, h, pages);
    draw_rectangle(
        center.x - w * 0.45,
        center.y - h * 0.45,
        w * 0.42,
        h * 0.9,
        cover,
    );
    draw_rectangle(
        center.x + w * 0.03,
        center.y - h * 0.45,
        w * 0.42,
        h * 0.9,
        cover,
    );
    draw_line(
        center.x,
        center.y - h * 0.45,
        center.x,
        center.y + h * 0.45,
        1.0,
        brass_light(),
    );
}

pub(super) fn draw_spark_icon(center: Vec2, scale: f32) {
    let color = Color::from_rgba(112, 222, 199, 230);
    let radius = 12.0 * scale;
    draw_line(
        center.x - radius,
        center.y,
        center.x + radius,
        center.y,
        1.4,
        color,
    );
    draw_line(
        center.x,
        center.y - radius,
        center.x,
        center.y + radius,
        1.4,
        color,
    );
    draw_line(
        center.x - radius * 0.6,
        center.y - radius * 0.6,
        center.x + radius * 0.6,
        center.y + radius * 0.6,
        1.0,
        color,
    );
    draw_line(
        center.x - radius * 0.6,
        center.y + radius * 0.6,
        center.x + radius * 0.6,
        center.y - radius * 0.6,
        1.0,
        color,
    );
    draw_circle(center.x, center.y, 2.4 * scale, bright_ink());
}

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

pub(super) fn draw_flower(center: Vec2, scale: f32) {
    let petal = Color::from_rgba(244, 233, 189, 238);
    let petal_shadow = Color::from_rgba(180, 138, 104, 128);
    let core = Color::from_rgba(241, 188, 72, 245);
    let radius = 4.5 * scale;
    for index in 0..5 {
        let angle = index as f32 * std::f32::consts::TAU / 5.0 - 0.3;
        let point = center + vec2(angle.cos(), angle.sin()) * (7.0 * scale);
        draw_circle(
            point.x + 0.8 * scale,
            point.y + 1.1 * scale,
            radius,
            petal_shadow,
        );
        draw_circle(point.x, point.y, radius, petal);
    }
    draw_circle(center.x, center.y, 3.2 * scale, core);
}

pub(super) fn draw_leaf_cluster(center: Vec2, mirrored: bool) {
    draw_leaf_cluster_scaled(center, mirrored, 1.0);
}

pub(super) fn draw_leaf_cluster_scaled(center: Vec2, mirrored: bool, scale: f32) {
    let sign = if mirrored { -1.0 } else { 1.0 };
    let leaf = Color::from_rgba(91, 142, 76, 230);
    let light = Color::from_rgba(150, 190, 105, 230);
    draw_triangle(
        center + vec2(0.0, -12.0) * scale,
        center + vec2(sign * 20.0, -4.0) * scale,
        center + vec2(sign * 3.0, 4.0) * scale,
        leaf,
    );
    draw_triangle(
        center + vec2(sign * 3.0, 0.0) * scale,
        center + vec2(sign * 26.0, 13.0) * scale,
        center + vec2(sign * 2.0, 16.0) * scale,
        light,
    );
    draw_circle(
        center.x - sign * 8.0 * scale,
        center.y + 4.0 * scale,
        4.0 * scale,
        Color::from_rgba(239, 226, 172, 245),
    );
}

pub(super) fn draw_sun_icon(center: Vec2, radius: f32) {
    let color = Color::from_rgba(242, 173, 56, 255);
    for index in 0..8 {
        let angle = index as f32 * std::f32::consts::TAU / 8.0;
        let inner = center + vec2(angle.cos(), angle.sin()) * (radius + 4.0);
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius + 12.0);
        draw_line(inner.x, inner.y, outer.x, outer.y, 2.0, color);
    }
    draw_circle(center.x, center.y, radius, color);
    draw_circle(
        center.x - 4.0,
        center.y - 4.0,
        radius * 0.35,
        Color::from_rgba(255, 232, 143, 230),
    );
}

pub(super) fn fill_slate() -> Color {
    Color::from_rgba(34, 34, 45, 228)
}

pub(super) fn slot_glow(index: usize) -> Color {
    match index % 5 {
        0 => Color::from_rgba(223, 77, 70, 255),
        1 => Color::from_rgba(75, 158, 232, 255),
        2 => Color::from_rgba(112, 203, 115, 255),
        3 => Color::from_rgba(232, 184, 76, 255),
        _ => Color::from_rgba(180, 92, 224, 255),
    }
}

pub(super) fn bright_ink() -> Color {
    Color::from_rgba(246, 238, 213, 255)
}

pub(super) fn muted_ink() -> Color {
    Color::from_rgba(186, 174, 145, 255)
}

pub(super) fn parchment() -> Color {
    Color::from_rgba(226, 204, 162, 255)
}

pub(super) fn brass() -> Color {
    Color::from_rgba(189, 140, 69, 255)
}

pub(super) fn brass_light() -> Color {
    Color::from_rgba(242, 205, 126, 255)
}

pub(super) fn shadow() -> Color {
    Color::from_rgba(0, 0, 0, 108)
}
