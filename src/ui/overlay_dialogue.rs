use super::{draw_panel, draw_wrapped_text};
use crate::view_models::dialogue::DialogueOverlayView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_dialogue_overlay_view(view: &DialogueOverlayView) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 130),
    );

    // Bottom-anchored speech panel, centered and clamped so it never inverts its
    // width on a narrow window or slides off the top of a short one.
    let h: f32 = 226.0;
    let w = (screen_width() - 360.0).clamp(420.0, 960.0).min(screen_width() - 32.0);
    let x = ((screen_width() - w) * 0.5).max(0.0);
    let y = (screen_height() - h - 40.0).max(16.0);
    draw_panel(x, y, w, h, &view.title);

    draw_ui_text(&view.now_text, x + 20.0, y + 34.0, 18.0, dark::TEXT_DIM);
    draw_ui_text(&view.later_text, x + 20.0, y + 54.0, 18.0, dark::TEXT_DIM);
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

    // A dim divider plus a highlighted prompt makes the exit/continue controls
    // an obvious, always-visible affordance rather than an easy-to-miss line.
    let footer_y = y + h - 34.0;
    draw_line(
        x + 20.0,
        footer_y - 6.0,
        x + w - 20.0,
        footer_y - 6.0,
        1.0,
        Color::from_rgba(223, 184, 111, 70),
    );
    draw_ui_text(&view.footer, x + 20.0, footer_y + 14.0, 18.0, dark::TEXT_BRIGHT);
}
