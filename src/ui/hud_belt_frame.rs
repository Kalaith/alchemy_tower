use super::hud_primitives::*;
use super::HOTBAR_SLOT_COUNT;
use macroquad::prelude::*;

pub(super) fn draw_belt_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 20.0, rect.y - 6.0, rect.w + 40.0, rect.h + 10.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        13.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 13.0, Color::from_rgba(112, 78, 43, 138));
    draw_beveled_rect_lines(back, 13.0, 1.2, Color::from_rgba(235, 196, 118, 142));
    draw_panel_texture(back, 13.0, Color::from_rgba(112, 78, 43, 138), 0.72);
}

pub(super) fn draw_belt_hardware(rect: Rect, slot_size: f32, gap: f32) {
    let rail = Rect::new(rect.x + 26.0, rect.y + 10.0, rect.w - 52.0, rect.h - 25.0);
    draw_beveled_rect(rail, 10.0, Color::from_rgba(24, 19, 16, 92));
    draw_beveled_rect_lines(rail, 10.0, 0.9, Color::from_rgba(238, 196, 119, 76));
    draw_beveled_rect_lines(
        Rect::new(rail.x + 5.0, rail.y + 5.0, rail.w - 10.0, rail.h - 10.0),
        7.0,
        0.8,
        Color::from_rgba(255, 238, 181, 38),
    );

    for side in [-1.0, 1.0] {
        let center = vec2(
            if side < 0.0 {
                rect.x + 16.0
            } else {
                rect.x + rect.w - 16.0
            },
            rect.y + rect.h * 0.5,
        );
        draw_circle(
            center.x + side * 2.0,
            center.y + 3.0,
            12.0,
            Color::from_rgba(0, 0, 0, 58),
        );
        draw_poly(
            center.x,
            center.y,
            6,
            12.0,
            30.0,
            Color::from_rgba(116, 78, 41, 164),
        );
        draw_poly_lines(
            center.x,
            center.y,
            6,
            12.0,
            30.0,
            1.1,
            Color::from_rgba(242, 205, 126, 164),
        );
        draw_small_diamond(center, Color::from_rgba(85, 222, 207, 146));
    }

    for index in 1..HOTBAR_SLOT_COUNT {
        let x = rect.x + 40.0 + index as f32 * slot_size + (index as f32 - 0.5) * gap;
        draw_line(
            x,
            rect.y + 20.0,
            x,
            rect.y + rect.h - 35.0,
            0.8,
            Color::from_rgba(242, 205, 126, 46),
        );
        draw_circle(
            x,
            rect.y + rect.h - 23.0,
            2.0,
            Color::from_rgba(242, 205, 126, 96),
        );
    }

    draw_flourish_line(
        rect.x + rect.w * 0.5 - 52.0,
        rect.y + rect.h - 12.0,
        rect.x + rect.w * 0.5 - 10.0,
        rect.y + rect.h - 12.0,
        true,
    );
    draw_flourish_line(
        rect.x + rect.w * 0.5 + 10.0,
        rect.y + rect.h - 12.0,
        rect.x + rect.w * 0.5 + 52.0,
        rect.y + rect.h - 12.0,
        false,
    );
}
