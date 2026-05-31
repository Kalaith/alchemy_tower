use super::super::hud_primitives::*;
use macroquad::prelude::*;

pub(super) fn draw_time_panel_hardware(rect: Rect) {
    let text_band = Rect::new(rect.x + 9.0, rect.y + 8.0, rect.w - 72.0, rect.h - 16.0);
    draw_beveled_rect(text_band, 8.0, Color::from_rgba(10, 11, 13, 58));
    draw_beveled_rect_lines(text_band, 8.0, 0.9, Color::from_rgba(242, 205, 126, 54));
    for y in [rect.y + 35.0, rect.y + 70.0] {
        draw_line(
            text_band.x + 12.0,
            y,
            text_band.x + text_band.w - 12.0,
            y,
            0.9,
            Color::from_rgba(242, 205, 126, 54),
        );
        draw_small_diamond(
            vec2(text_band.x + text_band.w * 0.5, y),
            Color::from_rgba(242, 205, 126, 86),
        );
    }

    let sun_center = vec2(rect.x + rect.w - 34.0, rect.y + 32.0);
    draw_circle(
        sun_center.x + 3.0,
        sun_center.y + 4.0,
        25.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_circle(
        sun_center.x,
        sun_center.y,
        24.0,
        Color::from_rgba(92, 62, 35, 172),
    );
    draw_circle_lines(
        sun_center.x,
        sun_center.y,
        24.0,
        1.1,
        Color::from_rgba(242, 205, 126, 154),
    );
    draw_circle_lines(
        sun_center.x,
        sun_center.y,
        18.0,
        0.8,
        Color::from_rgba(255, 238, 181, 62),
    );
    for index in 0..12 {
        let angle = index as f32 * std::f32::consts::TAU / 12.0;
        let point = sun_center + vec2(angle.cos(), angle.sin()) * 22.0;
        draw_circle(point.x, point.y, 1.1, Color::from_rgba(242, 205, 126, 122));
    }

    for point in [
        vec2(rect.x + 13.0, rect.y + 12.0),
        vec2(rect.x + rect.w - 13.0, rect.y + 12.0),
        vec2(rect.x + 13.0, rect.y + rect.h - 12.0),
        vec2(rect.x + rect.w - 13.0, rect.y + rect.h - 12.0),
    ] {
        draw_circle(point.x, point.y, 2.0, Color::from_rgba(242, 205, 126, 120));
        draw_circle(
            point.x - 0.6,
            point.y - 0.6,
            0.8,
            Color::from_rgba(255, 238, 181, 144),
        );
    }
}
