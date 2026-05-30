use crate::content::{narrative_text, ui_copy};
use super::{draw_panel, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_ending_overlay_view() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 180),
    );
    let x = 170.0;
    let y = 110.0;
    let w = screen_width() - 340.0;
    let h = screen_height() - 220.0;
    draw_panel(x, y, w, h, ui_copy("overlay_ending_title"));
    draw_wrapped_text(
        &narrative_text().overlays.observatory_epilogue,
        x + 24.0,
        y + 60.0,
        w - 48.0,
        22.0,
        28.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        &narrative_text().overlays.observatory_footer,
        x + 24.0,
        y + h - 24.0,
        18.0,
        dark::TEXT_DIM,
    );
}
