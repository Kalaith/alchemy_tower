use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use super::{draw_wrapped_text, truncate_text_to_width};

pub(crate) fn draw_panel(x: f32, y: f32, width: f32, height: f32, title: &str) {
    let style = macroquad_toolkit::ui::SurfaceStyle::new(dark::PANEL)
        .with_shadow(vec2(8.0, 10.0), Color::from_rgba(0, 0, 0, 96))
        .with_header(34.0, dark::PANEL_HEADER)
        .with_border(1.5, Color::from_rgba(160, 170, 190, 68));
    macroquad_toolkit::ui::draw_surface(Rect::new(x, y, width, height), &style);
    draw_line(
        x + 14.0,
        y + 42.0,
        x + width - 14.0,
        y + 42.0,
        1.0,
        Color::from_rgba(255, 255, 255, 28),
    );
    draw_text(
        &truncate_text_to_width(title, width - 24.0, 22.0),
        x + 12.0,
        y + 24.0,
        22.0,
        dark::TEXT_BRIGHT,
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
    draw_text(
        &truncate_text_to_width(text, w - 48.0, 18.0),
        x + 24.0,
        y + h - 23.0,
        18.0,
        dark::TEXT_DIM,
    );
}
