use super::draw_wrapped_text;
use super::hud_chrome::*;
use super::hud_primitives::*;
use super::HudView;
use macroquad::prelude::*;

#[path = "hud_minimap_frame.rs"]
mod hud_minimap_frame;

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

pub(super) fn draw_minimap_frame(view: &HudView) {
    hud_minimap_frame::draw_minimap_frame(view);
}
