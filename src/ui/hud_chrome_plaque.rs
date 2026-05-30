use super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_journal_note_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 8.0, rect.y - 7.0, rect.w + 16.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        11.0,
        Color::from_rgba(0, 0, 0, 82),
    );
    draw_beveled_rect(back, 11.0, Color::from_rgba(93, 66, 37, 146));
    draw_panel_texture(back, 11.0, Color::from_rgba(93, 66, 37, 146), 0.76);
    draw_beveled_rect_lines(back, 11.0, 1.2, Color::from_rgba(235, 196, 118, 126));
    for point in [
        vec2(back.x + 15.0, back.y + 15.0),
        vec2(back.x + back.w - 15.0, back.y + 15.0),
        vec2(back.x + back.w - 15.0, back.y + back.h - 15.0),
        vec2(back.x + 15.0, back.y + back.h - 15.0),
    ] {
        draw_circle(point.x, point.y, 2.4, Color::from_rgba(244, 204, 128, 158));
        draw_circle(point.x, point.y, 1.0, Color::from_rgba(49, 33, 20, 140));
    }
}

pub(super) fn draw_small_plaque_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 14.0, rect.y - 6.0, rect.w + 28.0, rect.h + 12.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        12.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 12.0, Color::from_rgba(120, 82, 42, 136));
    draw_panel_texture(back, 12.0, Color::from_rgba(120, 82, 42, 136), 0.68);
    draw_beveled_rect_lines(back, 12.0, 1.1, Color::from_rgba(235, 196, 118, 132));
}

pub(super) fn draw_vertical_plaque_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 8.0, rect.y - 8.0, rect.w + 16.0, rect.h + 16.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        11.0,
        Color::from_rgba(0, 0, 0, 78),
    );
    draw_beveled_rect(back, 11.0, Color::from_rgba(95, 62, 32, 116));
    draw_panel_texture(back, 11.0, Color::from_rgba(95, 62, 32, 116), 0.62);
    draw_beveled_rect_lines(back, 11.0, 1.0, Color::from_rgba(235, 196, 118, 112));
}
