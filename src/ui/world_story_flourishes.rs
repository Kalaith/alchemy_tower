use crate::data::AreaDefinition;
use macroquad::prelude::*;

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
        "greenhouse_floor" if cultivation_for_brin_complete => {
            draw_greenhouse_recovery_growth(offset);
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
