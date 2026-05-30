use crate::art::{draw_blocker_prop, ArtAssets};
use crate::data::AreaDefinition;
use macroquad::prelude::*;

pub(crate) fn draw_area_background(area: &AreaDefinition, offset: Vec2, art: &ArtAssets) {
    if let Some(texture) = art.background(&area.id) {
        draw_texture_ex(
            texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(area.size[0], area.size[1])),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(
            offset.x,
            offset.y,
            area.size[0],
            area.size[1],
            rgba(area.background),
        );
    }
}

pub(crate) fn draw_area_blockers(area: &AreaDefinition, offset: Vec2) {
    for (index, blocker) in area.blockers.iter().enumerate() {
        draw_blocker_prop(area, blocker, index, offset);
    }
}

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

pub(crate) fn draw_phase1_story_flourishes_view(
    area: &AreaDefinition,
    offset: Vec2,
    healing_for_mira_complete: bool,
    glow_for_rowan_complete: bool,
    town_recovery_reached: bool,
    cultivation_for_brin_complete: bool,
) {
    match area.id.as_str() {
        "town_square" => {
            if healing_for_mira_complete {
                draw_town_square_apothecary_shelf(offset);
            }
            if glow_for_rowan_complete {
                draw_town_square_glow_points(offset);
            }
            if town_recovery_reached {
                draw_town_square_recovery_flowers(offset);
            }
        }
        "greenhouse_floor" => {
            if cultivation_for_brin_complete {
                draw_greenhouse_recovery_growth(offset);
            }
        }
        _ => {}
    }
}

fn draw_town_square_apothecary_shelf(offset: Vec2) {
    let shelf = Color::from_rgba(122, 88, 66, 255);
    let bottle = Color::from_rgba(176, 226, 255, 255);
    draw_rectangle(offset.x + 684.0, offset.y + 670.0, 72.0, 18.0, shelf);
    draw_rectangle(offset.x + 694.0, offset.y + 652.0, 10.0, 18.0, bottle);
    draw_rectangle(
        offset.x + 714.0,
        offset.y + 646.0,
        12.0,
        24.0,
        Color::from_rgba(255, 214, 132, 255),
    );
    draw_rectangle(offset.x + 736.0, offset.y + 654.0, 10.0, 16.0, bottle);
}

fn draw_town_square_glow_points(offset: Vec2) {
    for (x, y) in [(536.0, 540.0), (610.0, 470.0), (696.0, 404.0)] {
        let pulse = ((get_time() as f32 * 2.2) + x * 0.01).sin() * 0.5 + 0.5;
        draw_circle(
            offset.x + x,
            offset.y + y,
            10.0 + pulse * 2.5,
            Color::from_rgba(255, 228, 150, 120),
        );
        draw_circle(
            offset.x + x,
            offset.y + y,
            5.0,
            Color::from_rgba(255, 244, 188, 255),
        );
    }
}

fn draw_town_square_recovery_flowers(offset: Vec2) {
    for (x, y, color) in [
        (598.0, 760.0, Color::from_rgba(126, 220, 158, 255)),
        (640.0, 744.0, Color::from_rgba(239, 205, 90, 255)),
        (676.0, 764.0, Color::from_rgba(188, 255, 220, 255)),
    ] {
        draw_circle(offset.x + x, offset.y + y, 8.0, color);
        draw_line(
            offset.x + x,
            offset.y + y + 8.0,
            offset.x + x,
            offset.y + y + 18.0,
            2.0,
            Color::from_rgba(88, 152, 102, 255),
        );
    }
}

fn draw_greenhouse_recovery_growth(offset: Vec2) {
    for (x, y) in [(690.0, 190.0), (742.0, 174.0), (794.0, 190.0)] {
        draw_circle(
            offset.x + x,
            offset.y + y,
            10.0,
            Color::from_rgba(126, 220, 158, 255),
        );
        draw_circle(
            offset.x + x + 10.0,
            offset.y + y - 4.0,
            7.0,
            Color::from_rgba(239, 205, 90, 255),
        );
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

fn rgba(color: [u8; 4]) -> Color {
    Color::from_rgba(color[0], color[1], color[2], color[3])
}
