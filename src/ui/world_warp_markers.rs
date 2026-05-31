use crate::art::{draw_texture_centered, ArtAssets};
use macroquad::prelude::*;

pub(crate) fn draw_unlock_ready_warp_glow(
    rect: Rect,
    center: Vec2,
    offset: Vec2,
    art: &ArtAssets,
) {
    let pulse = ((get_time() as f32 * 3.0) + rect.x * 0.01).sin() * 0.5 + 0.5;
    if let Some(texture) = art.effect("warp_glow_effect") {
        draw_texture_centered(
            texture,
            center,
            vec2(74.0 + pulse * 12.0, 74.0 + pulse * 12.0),
            Color::new(1.0, 1.0, 1.0, 0.55 + pulse * 0.2),
        );
    }
    draw_rectangle(
        offset.x + rect.x,
        offset.y + rect.y,
        rect.w,
        rect.h,
        Color::new(
            188.0 / 255.0,
            255.0 / 255.0,
            220.0 / 255.0,
            0.10 + pulse * 0.08,
        ),
    );
    draw_circle_lines(
        center.x,
        center.y,
        20.0 + pulse * 8.0,
        2.0,
        Color::from_rgba(188, 255, 220, 220),
    );
}

pub(crate) fn draw_warp_marker(
    rect: Rect,
    center: Vec2,
    offset: Vec2,
    unlock_ready: bool,
    art: &ArtAssets,
) {
    if unlock_ready {
        draw_unlock_ready_warp_glow(rect, center, offset, art);
    }
    draw_rectangle_lines(
        offset.x + rect.x,
        offset.y + rect.y,
        rect.w,
        rect.h,
        3.0,
        if unlock_ready {
            Color::from_rgba(188, 255, 220, 255)
        } else {
            Color::from_rgba(255, 245, 160, 255)
        },
    );
}
