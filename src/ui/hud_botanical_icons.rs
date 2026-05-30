use macroquad::prelude::*;

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
