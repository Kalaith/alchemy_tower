use super::{draw_panel, draw_wrapped_text};
use crate::view_models::dialogue::DialogueOverlayView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_dialogue_overlay_view(view: &DialogueOverlayView) {
    let sw = crate::ui_scale::ui_w();
    let sh = crate::ui_scale::ui_h();
    draw_rectangle(0.0, 0.0, sw, sh, Color::from_rgba(0, 0, 0, 130));

    // Bottom-anchored speech panel, centered and clamped so it never inverts its
    // width on a narrow window or slides off the top of a short one.
    let h: f32 = 216.0;
    let w = (sw - 360.0).clamp(440.0, 980.0).min(sw - 32.0);
    let x = ((sw - w) * 0.5).max(0.0);
    let y = (sh - h - 40.0).max(16.0);
    draw_panel(x, y, w, h, &view.title);

    // Just the character's words — the old Now/Later/Usually schedule readout
    // lived here and made every conversation read like a debug tracker. That
    // routine info now lives in the Journal's rapport tab where it belongs.
    draw_wrapped_text(
        &view.body,
        x + 22.0,
        y + 58.0,
        w - 44.0,
        20.0,
        26.0,
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
    draw_ui_text(
        &view.footer,
        x + 20.0,
        footer_y + 14.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}
