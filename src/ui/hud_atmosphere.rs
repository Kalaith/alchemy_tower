use macroquad::prelude::*;

pub(super) fn draw_hud_atmosphere(sw: f32, sh: f32) {
    draw_soft_circle(
        vec2(sw * 0.50, sh * 0.45),
        160.0,
        Color::from_rgba(59, 223, 211, 28),
        8,
    );
    draw_soft_circle(
        vec2(sw * 0.78, sh * 0.31),
        210.0,
        Color::from_rgba(246, 178, 92, 18),
        8,
    );
    draw_soft_circle(
        vec2(sw * 0.27, sh * 0.23),
        170.0,
        Color::from_rgba(238, 171, 84, 14),
        7,
    );
    draw_soft_circle(
        vec2(sw * 0.50, sh - 42.0),
        260.0,
        Color::from_rgba(44, 30, 18, 42),
        8,
    );
}

pub(super) fn draw_soft_circle(center: Vec2, radius: f32, color: Color, steps: usize) {
    for step in (1..=steps).rev() {
        let t = step as f32 / steps as f32;
        let alpha = color.a * (1.0 - t) * 0.42;
        draw_circle(
            center.x,
            center.y,
            radius * t,
            Color::new(color.r, color.g, color.b, alpha),
        );
    }
}

pub(super) fn draw_edge_foliage(sw: f32, sh: f32) {
    draw_foliage_silhouette(vec2(46.0, sh - 18.0), false, 1.1);
    draw_foliage_silhouette(vec2(135.0, sh - 6.0), false, 0.72);
    draw_foliage_silhouette(vec2(sw - 52.0, sh - 18.0), true, 1.08);
    draw_foliage_silhouette(vec2(sw - 142.0, sh - 8.0), true, 0.75);
}

pub(super) fn draw_foliage_silhouette(root: Vec2, mirrored: bool, scale: f32) {
    let sign = if mirrored { -1.0 } else { 1.0 };
    let leaf_dark = Color::from_rgba(19, 50, 29, 150);
    let leaf_mid = Color::from_rgba(42, 84, 45, 118);
    let stem = Color::from_rgba(45, 35, 22, 132);
    draw_line(
        root.x,
        root.y,
        root.x + sign * 54.0 * scale,
        root.y - 56.0 * scale,
        3.0 * scale,
        stem,
    );
    for (index, offset) in [
        vec2(10.0, -8.0),
        vec2(24.0, -22.0),
        vec2(38.0, -34.0),
        vec2(54.0, -50.0),
    ]
    .iter()
    .enumerate()
    {
        let base = root + vec2(sign * offset.x * scale, offset.y * scale);
        let width = (22.0 + index as f32 * 4.0) * scale;
        let height = (12.0 + index as f32 * 2.0) * scale;
        let color = if index % 2 == 0 { leaf_dark } else { leaf_mid };
        draw_triangle(
            base,
            base + vec2(sign * width, -height),
            base + vec2(sign * 7.0 * scale, height * 0.4),
            color,
        );
        draw_triangle(
            base + vec2(sign * 3.0 * scale, -3.0 * scale),
            base + vec2(-sign * width * 0.45, -height * 0.75),
            base + vec2(-sign * 5.0 * scale, height * 0.45),
            color,
        );
    }
}
