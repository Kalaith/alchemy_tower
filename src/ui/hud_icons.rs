use super::hud_palette::*;
pub(super) use super::hud_botanical_icons::{
    draw_flower, draw_leaf_cluster, draw_leaf_cluster_scaled,
};
use macroquad::prelude::*;

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

pub(super) fn draw_bottle_silhouette(rect: Rect, alpha: f32) {
    let color = Color::new(parchment().r, parchment().g, parchment().b, alpha);
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 17.0;
    draw_rectangle(cx - 5.0, top, 10.0, 13.0, color);
    draw_rectangle(cx - 12.0, top + 13.0, 24.0, 23.0, color);
    draw_circle(cx, top + 37.0, 12.0, color);
    draw_rectangle_lines(
        cx - 12.0,
        top + 13.0,
        24.0,
        24.0,
        1.0,
        Color::new(
            brass_light().r,
            brass_light().g,
            brass_light().b,
            alpha * 0.8,
        ),
    );
}
