//! The event banners: what just happened, in the words the caller wrote.
//!
//! Every celebratory moment in the game already raised one of these — a beat
//! recorded, a request delivered, a route reopened, a formula worked out, a
//! commission funded — and every one of them was formatted, passed into
//! `push_event_toast_with_icon`, and dropped on the floor. The struct behind it
//! held nothing but a countdown, six icons were generated for it and never
//! loaded, and the two `ui_art.json` keys that named them were quietly
//! discarded by serde. This is the whole channel, drawn.

use super::super::truncate_text_to_width;
use super::hud_primitives::*;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::view_models::hud::HudView;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

/// Sat above the status strip, which is itself above the belt. Stacking upward
/// from there keeps the newest banner closest to the eye and clear of the goal
/// note on the left and the bag on the right.
const TOAST_BOTTOM_OFFSET: f32 = 196.0;
const TOAST_WIDTH: f32 = 520.0;
const TOAST_HEIGHT: f32 = 40.0;
const TOAST_STEP: f32 = 46.0;
const ICON_SIZE: f32 = 26.0;

pub(super) fn draw_event_toasts(view: &HudView, art: &ArtAssets) {
    for (index, toast) in view.toasts.iter().enumerate() {
        let width = TOAST_WIDTH.min(super::hud_w() - 320.0);
        let x = super::hud_w() * 0.5 - width * 0.5;
        let y = super::hud_h() - TOAST_BOTTOM_OFFSET - index as f32 * TOAST_STEP;
        let rect = Rect::new(x, y, width, TOAST_HEIGHT);
        let accent = Color::new(
            toast.color[0],
            toast.color[1],
            toast.color[2],
            toast.color[3] * toast.alpha,
        );

        draw_ornate_panel(
            rect,
            Color::from_rgba(17, 17, 19, (196.0 * toast.alpha) as u8),
            0.72,
        );
        // A bar of the event's own colour down the leading edge, the same
        // language the alchemy slots use for their accents.
        draw_rectangle(rect.x + 3.0, rect.y + 8.0, 4.0, rect.h - 16.0, accent);

        let mut text_x = rect.x + 16.0;
        if let Some(texture) = art.toast_icon(&toast.icon_key) {
            draw_texture_centered(
                texture,
                vec2(rect.x + 28.0, rect.y + rect.h * 0.5),
                vec2(ICON_SIZE, ICON_SIZE),
                Color::new(1.0, 1.0, 1.0, toast.alpha),
            );
            text_x = rect.x + 48.0;
        }

        let text_width = rect.x + rect.w - 14.0 - text_x;
        draw_ui_text(
            &truncate_text_to_width(&toast.text, text_width, 18.0),
            text_x,
            rect.y + 26.0,
            18.0,
            Color::new(bright_ink().r, bright_ink().g, bright_ink().b, toast.alpha),
        );
    }
}
