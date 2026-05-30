use super::hud_chrome::*;
use super::hud_compass::*;
use super::hud_primitives::*;
use super::HudView;
use crate::content::ui_copy;
use super::draw_wrapped_text;
use macroquad::prelude::*;

pub(super) fn draw_time_panel(view: &HudView) {
    let rect = Rect::new(screen_width() - 326.0, 22.0, 218.0, 94.0);
    draw_small_plaque_backplate(rect);
    draw_ornate_panel(rect, fill_slate(), 0.9);
    draw_panel_filigree(rect, 0.56);
    draw_time_panel_hardware(rect);
    let text_width = rect.w - 62.0;
    draw_centered_text(
        &view.season_weather_text,
        rect.x + 10.0,
        rect.y + 27.0,
        text_width,
        18.0,
        parchment(),
    );
    draw_centered_text_shadowed(
        &view.clock_text,
        rect.x + 10.0,
        rect.y + 62.0,
        text_width,
        30.0,
        bright_ink(),
    );
    draw_centered_text_shadowed(
        &view.day_text,
        rect.x + 10.0,
        rect.y + 86.0,
        text_width,
        18.0,
        bright_ink(),
    );
    draw_sun_icon(vec2(rect.x + rect.w - 34.0, rect.y + 32.0), 13.0);

    if let Some(text) = &view.sleep_warning_text {
        let warning = Rect::new(rect.x, rect.y + rect.h + 8.0, rect.w, 38.0);
        draw_ornate_panel(warning, Color::from_rgba(74, 42, 31, 214), 0.82);
        draw_wrapped_text(
            text,
            warning.x + 12.0,
            warning.y + 23.0,
            warning.w - 24.0,
            14.0,
            15.0,
            Color::from_rgba(255, 224, 168, 255),
        );
    }
}

pub(super) fn draw_minimap_frame() {
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
    draw_text(
        ui_copy("hud_minimap_north"),
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
