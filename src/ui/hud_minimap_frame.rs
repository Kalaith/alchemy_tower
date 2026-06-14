use super::super::hud_compass::*;
use super::super::hud_primitives::*;
use super::super::HudView;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub(super) fn draw_minimap_frame(view: &HudView) {
    let center = vec2(screen_width() - 62.0, 82.0);
    let radius = 62.0;
    draw_compass_backplate(center, radius);
    draw_circle(center.x + 5.0, center.y + 8.0, radius, shadow());
    draw_circle(
        center.x,
        center.y,
        radius,
        Color::from_rgba(72, 56, 42, 178),
    );
    draw_circle_lines(center.x, center.y, radius, 4.0, brass());
    draw_circle_lines(
        center.x,
        center.y,
        radius - 11.0,
        1.5,
        Color::from_rgba(230, 204, 150, 132),
    );
    draw_line(
        center.x - 42.0,
        center.y,
        center.x + 42.0,
        center.y,
        1.0,
        Color::from_rgba(230, 204, 150, 54),
    );
    draw_line(
        center.x,
        center.y - 42.0,
        center.x,
        center.y + 42.0,
        1.0,
        Color::from_rgba(230, 204, 150, 54),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius - 25.0,
        1.0,
        Color::from_rgba(230, 204, 150, 42),
    );
    draw_compass_ticks(center, radius);
    draw_compass_map_texture(center, radius);
    draw_line(
        center.x - 30.0,
        center.y - 30.0,
        center.x + 30.0,
        center.y + 30.0,
        1.0,
        Color::from_rgba(230, 204, 150, 36),
    );
    draw_line(
        center.x - 30.0,
        center.y + 30.0,
        center.x + 30.0,
        center.y - 30.0,
        1.0,
        Color::from_rgba(230, 204, 150, 36),
    );
    for marker in [
        vec2(center.x, center.y - radius + 6.0),
        vec2(center.x + radius - 6.0, center.y),
        vec2(center.x, center.y + radius - 6.0),
        vec2(center.x - radius + 6.0, center.y),
    ] {
        draw_small_diamond(marker, brass_light());
    }
    draw_ui_text(
        &view.minimap_north_label,
        center.x - 6.0,
        center.y - radius + 17.0,
        17.0,
        parchment(),
    );
    draw_compass_rosette(center);
    draw_triangle(
        vec2(center.x, center.y - 8.0),
        vec2(center.x - 7.0, center.y + 12.0),
        vec2(center.x + 7.0, center.y + 12.0),
        bright_ink(),
    );
    draw_leaf_cluster_scaled(center + vec2(38.0, 47.0), true, 0.72);
}
