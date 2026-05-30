use crate::view_models::dialogue::DialogueOverlayView;
use super::{draw_panel, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_dialogue_overlay_view(view: &DialogueOverlayView) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 130),
    );
    let x = 180.0;
    let y = screen_height() - 286.0;
    let w = screen_width() - 360.0;
    let h = 226.0;
    draw_panel(x, y, w, h, &view.title);
    draw_text(&view.now_text, x + 20.0, y + 34.0, 18.0, dark::TEXT_DIM);
    draw_text(
        &view.later_text,
        x + 20.0,
        y + 54.0,
        18.0,
        dark::TEXT_DIM,
    );
    draw_wrapped_text(
        &view.usually_text,
        x + 20.0,
        y + 72.0,
        w - 40.0,
        16.0,
        18.0,
        dark::TEXT_DIM,
    );
    draw_wrapped_text(
        &view.body,
        x + 20.0,
        y + 104.0,
        w - 40.0,
        20.0,
        24.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(&view.footer, x + 20.0, y + h - 28.0, 20.0, dark::TEXT_DIM);
}
