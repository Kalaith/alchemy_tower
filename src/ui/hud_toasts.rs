//! The event banners: what just happened, in the words the caller wrote.
//!
//! Every celebratory moment in the game already raised one of these — a beat
//! recorded, a request delivered, a route reopened, a formula worked out, a
//! commission funded — and every one of them was formatted, passed into
//! `push_event_toast_with_icon`, and dropped on the floor. The struct behind it
//! held nothing but a countdown, six icons were generated for it and never
//! loaded, and the two `ui_art.json` keys that named them were quietly
//! discarded by serde. This is the whole channel, drawn.

use super::hud_primitives::*;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::view_models::hud::HudView;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

/// Sat above the status strip, which is itself above the belt. Stacking upward
/// from there keeps the newest banner closest to the eye and clear of the goal
/// note on the left and the bag on the right.
const TOAST_BOTTOM_OFFSET: f32 = 196.0;
const TOAST_WIDTH: f32 = 680.0;
const ICON_SIZE: f32 = 26.0;
pub(crate) const TOAST_FONT: f32 = 18.0;
pub(crate) const TOAST_LINE_HEIGHT: f32 = 21.0;
/// A banner holds a sentence, and a sentence that needs a third line belongs in
/// the journal rather than in something that leaves after two seconds.
pub(crate) const TOAST_MAX_LINES: usize = 2;
const TOAST_PADDING: f32 = 19.0;
const TOAST_GAP: f32 = 6.0;

/// The room a banner's words have, at the reference window. Exported for the
/// guard that keeps authored lines inside it.
pub(crate) fn toast_text_width(hud_width: f32) -> f32 {
    TOAST_WIDTH.min(hud_width - 320.0) - 48.0 - 14.0
}

pub(super) fn draw_event_toasts(view: &HudView, art: &ArtAssets) {
    let mut bottom = super::hud_h() - TOAST_BOTTOM_OFFSET;
    for toast in view.toasts.iter() {
        let width = TOAST_WIDTH.min(super::hud_w() - 320.0);
        let text_width = toast_text_width(super::hud_w());
        // A banner is as tall as what it has to say. Several of these lines run
        // past one line at this width, and a fixed box cut them mid-sentence.
        let lines = super::super::text::wrapped_lines(&toast.text, text_width, TOAST_FONT);
        let shown = lines.len().clamp(1, TOAST_MAX_LINES);
        let height = shown as f32 * TOAST_LINE_HEIGHT + TOAST_PADDING;
        let x = super::hud_w() * 0.5 - width * 0.5;
        let y = bottom - height;
        bottom = y - TOAST_GAP;
        let rect = Rect::new(x, y, width, height);
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

        let ink = Color::new(bright_ink().r, bright_ink().g, bright_ink().b, toast.alpha);
        for (line_index, line) in lines.iter().take(shown).enumerate() {
            // Drawn as wrapped, not re-measured. Wrapping and truncation
            // disagree about how wide a string is — the wrap applies the UI
            // scale to the font size and the truncate does not — so putting a
            // wrapped line back through `truncate_text_to_width` elided a line
            // that already fitted, and the banner showed one cut-off sentence
            // in a box with room for two lines.
            let overflowed = line_index + 1 == shown && lines.len() > shown;
            let text = if overflowed {
                format!("{line}…")
            } else {
                line.clone()
            };
            draw_ui_text(
                &text,
                text_x,
                rect.y + TOAST_PADDING + line_index as f32 * TOAST_LINE_HEIGHT,
                TOAST_FONT,
                ink,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{toast_text_width, TOAST_MAX_LINES};

    /// Measured off `screenshots/hud/event_toasts.png`: the banner's text column
    /// is 618 wide at the reference window and wraps at about 60 characters per
    /// line at font 18. Deliberately generous — a long word wraps early.
    const CHARS_PER_LINE: usize = 60;
    /// The shortest window the game is laid out for.
    const REFERENCE_SCREEN_WIDTH: f32 = 1280.0;

    /// A banner leaves after two seconds and holds two lines. A townsperson's
    /// remark that needs a third is one nobody will finish reading, and the
    /// renderer would cut it mid-sentence — the same failure the epilogue and
    /// the route pane each shipped once.
    #[test]
    fn what_a_townsperson_says_about_good_work_fits_the_banner() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let budget = TOAST_MAX_LINES * CHARS_PER_LINE;
        assert!(
            toast_text_width(REFERENCE_SCREEN_WIDTH) > 0.0,
            "the banner has no room for text at all"
        );

        let overrun = data
            .npcs
            .iter()
            .filter(|npc| npc.exceptional_delivery_line.len() > budget)
            .map(|npc| {
                format!(
                    "{}: {} characters against {budget}",
                    npc.id,
                    npc.exceptional_delivery_line.len()
                )
            })
            .collect::<Vec<_>>();

        assert!(
            overrun.is_empty(),
            "remarks that will not fit the banner they are raised in: {overrun:#?}"
        );
    }
}
