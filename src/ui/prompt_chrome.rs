use macroquad::prelude::*;

use super::prompt_shapes::{draw_beveled_rect, draw_beveled_rect_lines};

pub(super) fn draw_prompt_backplate(label_rect: Rect, key_center: Vec2) {
    let back = Rect::new(
        label_rect.x - 12.0,
        label_rect.y - 7.0,
        label_rect.w + 82.0,
        label_rect.h + 16.0,
    );
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        12.0,
        Color::from_rgba(0, 0, 0, 88),
    );
    draw_beveled_rect(back, 12.0, Color::from_rgba(101, 67, 34, 138));
    draw_beveled_rect_lines(back, 12.0, 1.0, Color::from_rgba(236, 195, 116, 128));
    for offset in [-1.0, 1.0] {
        draw_poly(
            key_center.x + offset * 35.0,
            key_center.y,
            4,
            4.0,
            45.0,
            Color::from_rgba(242, 205, 126, 158),
        );
    }
}

pub(super) fn draw_prompt_flourish(x: f32, center_y: f32, width: f32) {
    let brass = Color::from_rgba(221, 177, 96, 176);
    draw_line(x - 58.0, center_y, x + 24.0, center_y, 2.0, brass);
    draw_line(
        x + width - 74.0,
        center_y,
        x + width - 14.0,
        center_y,
        1.6,
        Color::from_rgba(221, 177, 96, 146),
    );
    draw_circle_lines(
        x - 20.0,
        center_y,
        10.0,
        1.4,
        Color::from_rgba(221, 177, 96, 160),
    );
    draw_circle_lines(
        x + width - 86.0,
        center_y,
        8.0,
        1.2,
        Color::from_rgba(221, 177, 96, 132),
    );
    for point in [vec2(x + 3.0, center_y), vec2(x + width - 72.0, center_y)] {
        draw_poly(
            point.x,
            point.y,
            4,
            5.0,
            45.0,
            Color::from_rgba(242, 205, 126, 172),
        );
    }
}

pub(super) fn draw_key_medallion(center: Vec2) {
    draw_poly(
        center.x + 4.0,
        center.y + 6.0,
        4,
        33.0,
        45.0,
        Color::from_rgba(0, 0, 0, 84),
    );
    draw_poly(
        center.x,
        center.y,
        4,
        33.0,
        45.0,
        Color::from_rgba(94, 70, 36, 186),
    );
    draw_poly(
        center.x,
        center.y,
        4,
        26.0,
        45.0,
        Color::from_rgba(39, 75, 110, 238),
    );
    draw_poly_lines(
        center.x,
        center.y,
        4,
        33.0,
        45.0,
        2.0,
        Color::from_rgba(242, 205, 126, 224),
    );
    draw_poly_lines(
        center.x,
        center.y,
        4,
        23.0,
        45.0,
        1.0,
        Color::from_rgba(185, 255, 244, 126),
    );
    draw_circle(
        center.x - 7.0,
        center.y - 8.0,
        4.0,
        Color::from_rgba(174, 247, 235, 82),
    );
}
