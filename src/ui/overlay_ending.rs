use super::{draw_panel, draw_wrapped_text};
use crate::view_models::ending::EndingOverlayView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_ending_overlay_view(view: &EndingOverlayView) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 180),
    );
    let w = (screen_width() - 340.0)
        .clamp(480.0, 900.0)
        .min(screen_width() - 32.0);
    let h = (screen_height() - 220.0)
        .clamp(280.0, 620.0)
        .min(screen_height() - 32.0);
    let x = ((screen_width() - w) * 0.5).max(0.0);
    let y = ((screen_height() - h) * 0.5).max(0.0);
    draw_panel(x, y, w, h, &view.title);
    draw_wrapped_text(
        &view.body,
        x + 24.0,
        y + 60.0,
        w - 48.0,
        22.0,
        28.0,
        dark::TEXT_BRIGHT,
    );
    draw_ui_text(&view.footer, x + 24.0, y + h - 24.0, 18.0, dark::TEXT_DIM);
}
