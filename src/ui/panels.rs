use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use super::hud::{
    brass_light, bright_ink, draw_ornate_panel, draw_panel_filigree, fill_slate,
};
use super::{draw_wrapped_text, truncate_text_to_width};
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_panel(x: f32, y: f32, width: f32, height: f32, title: &str) {
    // Same beveled, textured, gilded framing the HUD uses, so overlays read as
    // part of the same ornate UI rather than flat dialog boxes.
    let rect = Rect::new(x, y, width, height);
    draw_ornate_panel(rect, fill_slate(), 0.96);
    draw_panel_filigree(rect, 0.7);
    let underline = brass_light();
    draw_line(
        x + 16.0,
        y + 42.0,
        x + width - 16.0,
        y + 42.0,
        1.0,
        Color::new(underline.r, underline.g, underline.b, 0.5),
    );
    draw_ui_text(
        &truncate_text_to_width(title, width - 28.0, 22.0),
        x + 16.0,
        y + 26.0,
        22.0,
        bright_ink(),
    );
}

pub(crate) fn draw_panel_frame(panel: Rect) {
    let style =
        macroquad_toolkit::ui::SurfaceStyle::new(dark::PANEL).with_border(2.0, dark::ACCENT);
    macroquad_toolkit::ui::draw_surface(panel, &style);
}

pub(crate) fn draw_overlay_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 150),
    );
}

pub(crate) fn draw_overlay_subtitle(x: f32, y: f32, text: &str) {
    let width = screen_width() - x * 2.0 - 40.0;
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(Color::from_rgba(16, 18, 26, 176))
        .with_border(1.0, Color::from_rgba(160, 170, 190, 52));
    macroquad_toolkit::ui::draw_surface(Rect::new(x + 16.0, y + 46.0, width, 36.0), &surface);
    draw_wrapped_text(
        text,
        x + 28.0,
        y + 59.0,
        width - 24.0,
        20.0,
        20.0,
        dark::TEXT_DIM,
    );
}

pub(crate) fn draw_overlay_footer(x: f32, y: f32, w: f32, h: f32, text: &str) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(Color::from_rgba(16, 18, 26, 172))
        .with_border(1.0, Color::from_rgba(160, 170, 190, 44));
    macroquad_toolkit::ui::draw_surface(
        Rect::new(x + 16.0, y + h - 48.0, w - 32.0, 34.0),
        &surface,
    );
    draw_ui_text(
        &truncate_text_to_width(text, w - 48.0, 18.0),
        x + 24.0,
        y + h - 23.0,
        18.0,
        dark::TEXT_DIM,
    );
}
