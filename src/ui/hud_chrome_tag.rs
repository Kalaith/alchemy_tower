use super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_tag_panel(rect: Rect) {
    let left_tab = [
        vec2(rect.x - 6.0, rect.y + 7.0),
        vec2(rect.x + 8.0, rect.y + rect.h * 0.5),
        vec2(rect.x - 6.0, rect.y + rect.h - 7.0),
    ];
    let right_tab = [
        vec2(rect.x + rect.w + 6.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 8.0, rect.y + rect.h * 0.5),
        vec2(rect.x + rect.w + 6.0, rect.y + rect.h - 7.0),
    ];
    draw_beveled_rect(
        Rect::new(rect.x + 4.0, rect.y + 5.0, rect.w, rect.h),
        7.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_triangle(
        left_tab[0],
        left_tab[1],
        left_tab[2],
        Color::from_rgba(143, 111, 71, 212),
    );
    draw_triangle(
        right_tab[0],
        right_tab[1],
        right_tab[2],
        Color::from_rgba(143, 111, 71, 212),
    );
    draw_beveled_rect(rect, 7.0, Color::from_rgba(181, 156, 112, 228));
    draw_beveled_rect_lines(rect, 7.0, 1.5, Color::from_rgba(235, 201, 137, 220));
    draw_beveled_rect_lines(
        Rect::new(rect.x + 4.0, rect.y + 4.0, rect.w - 8.0, rect.h - 8.0),
        5.0,
        1.0,
        Color::from_rgba(76, 51, 32, 92),
    );
    draw_flourish_line(
        rect.x + rect.w - 31.0,
        rect.y + rect.h * 0.5,
        rect.x + rect.w - 9.0,
        rect.y + rect.h * 0.5,
        false,
    );
    for x in [rect.x + 8.0, rect.x + rect.w - 8.0] {
        draw_circle(
            x,
            rect.y + rect.h * 0.5,
            2.2,
            Color::from_rgba(76, 51, 32, 128),
        );
        draw_circle(
            x - 0.7,
            rect.y + rect.h * 0.5 - 0.7,
            1.0,
            Color::from_rgba(255, 229, 158, 138),
        );
    }
}
