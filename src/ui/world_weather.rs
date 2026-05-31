use crate::data::AreaDefinition;
use macroquad::prelude::*;

pub(crate) fn draw_environment_overlay_view(
    area: &AreaDefinition,
    offset: Vec2,
    time_window: &str,
    weather: &str,
) {
    let time_tint = match time_window {
        "morning" => Color::from_rgba(255, 220, 170, 24),
        "day" => Color::from_rgba(255, 255, 255, 0),
        "evening" => Color::from_rgba(255, 184, 120, 38),
        _ => Color::from_rgba(72, 92, 150, 72),
    };
    if time_tint.a > 0.0 {
        draw_rectangle(offset.x, offset.y, area.size[0], area.size[1], time_tint);
    }

    match weather {
        "mist" => draw_mist_overlay(area, offset),
        "rain" => draw_rain_overlay(area, offset),
        "windy" => draw_windy_overlay(area, offset),
        _ => {}
    }
}

fn draw_mist_overlay(area: &AreaDefinition, offset: Vec2) {
    draw_rectangle(
        offset.x,
        offset.y,
        area.size[0],
        area.size[1],
        Color::from_rgba(220, 228, 240, 28),
    );
    for index in 0..10 {
        let drift = ((get_time() as f32 * 0.4) + index as f32 * 0.6).sin() * 18.0;
        let x = offset.x + 80.0 + index as f32 * 110.0 + drift;
        let y = offset.y + 60.0 + (index % 4) as f32 * 120.0;
        draw_circle(
            x,
            y,
            42.0 + (index % 3) as f32 * 12.0,
            Color::from_rgba(240, 244, 248, 20),
        );
    }
}

fn draw_rain_overlay(area: &AreaDefinition, offset: Vec2) {
    draw_rectangle(
        offset.x,
        offset.y,
        area.size[0],
        area.size[1],
        Color::from_rgba(90, 126, 168, 26),
    );
    for index in 0..28 {
        let wave = ((get_time() as f32 * 2.8) + index as f32 * 0.4).fract();
        let x = offset.x + (index as f32 * 48.0).rem_euclid(area.size[0]);
        let y = offset.y + wave * area.size[1];
        draw_line(
            x,
            y,
            x - 8.0,
            y + 16.0,
            2.0,
            Color::from_rgba(200, 224, 255, 120),
        );
    }
}

fn draw_windy_overlay(area: &AreaDefinition, offset: Vec2) {
    for index in 0..16 {
        let wave = ((get_time() as f32 * 1.4) + index as f32 * 0.33).fract();
        let x = offset.x + wave * area.size[0];
        let y = offset.y + 30.0 + index as f32 * 34.0;
        draw_line(
            x - 10.0,
            y,
            x + 22.0,
            y - 6.0,
            2.0,
            Color::from_rgba(232, 232, 210, 64),
        );
    }
}
