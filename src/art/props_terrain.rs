use super::color_from_option;
use crate::data::AreaDefinition;
use macroquad::prelude::*;

pub(super) fn draw_quarry_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let stone = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(116, 112, 102, 255),
    );
    let face = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(156, 148, 132, 255),
    );
    let crack = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(82, 78, 74, 255),
    );
    let mineral = color_from_option(
        area.render.blocker_alt,
        Color::from_rgba(174, 144, 238, 255),
    );

    draw_rectangle(x, y, w, h, stone);
    draw_rectangle(x + 10.0, y + 10.0, w - 20.0, h * 0.28, face);
    draw_rectangle(x + 18.0, y + h * 0.42, w - 36.0, h * 0.22, face);
    draw_rectangle(x + 12.0, y + h * 0.72, w - 24.0, h * 0.14, face);
    draw_line(x + 18.0, y + 16.0, x + w - 24.0, y + h - 22.0, 3.0, crack);
    draw_line(
        x + w * 0.48,
        y + 14.0,
        x + w * 0.36,
        y + h - 20.0,
        2.0,
        crack,
    );
    draw_triangle(
        vec2(x + w - 34.0, y + 22.0),
        vec2(x + w - 18.0, y + 48.0),
        vec2(x + w - 50.0, y + 54.0),
        mineral,
    );
}

pub(super) fn draw_reeds_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let bank = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(116, 146, 132, 255),
    );
    let water = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(88, 142, 170, 255),
    );
    let reed = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(188, 204, 124, 255),
    );
    let stone = color_from_option(
        area.render.blocker_alt,
        Color::from_rgba(154, 164, 168, 255),
    );

    draw_rectangle(x, y, w, h, bank);
    draw_rectangle(x + 6.0, y + h * 0.38, w - 12.0, h * 0.48, water);
    for tuft in 0..8 {
        let tx = x + 14.0 + tuft as f32 * ((w - 28.0) / 8.0);
        draw_line(tx, y + h * 0.44, tx - 3.0, y + h * 0.1, 3.0, reed);
        draw_line(tx + 4.0, y + h * 0.48, tx + 7.0, y + h * 0.16, 2.0, reed);
    }
    for rock in 0..3 {
        draw_circle(
            x + 18.0 + rock as f32 * (w - 36.0) / 2.0,
            y + h * 0.78,
            10.0,
            stone,
        );
    }
}

pub(super) fn draw_dunes_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let sand = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(198, 166, 102, 255),
    );
    let ridge = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(222, 196, 130, 255),
    );
    let stone = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(138, 102, 64, 255),
    );
    let shrub = color_from_option(area.render.blocker_alt, Color::from_rgba(138, 148, 92, 255));

    draw_rectangle(x, y, w, h, sand);
    draw_circle(x + w * 0.22, y + h * 0.72, h * 0.28, ridge);
    draw_circle(x + w * 0.56, y + h * 0.62, h * 0.24, ridge);
    draw_circle(x + w * 0.84, y + h * 0.78, h * 0.22, ridge);
    draw_triangle(
        vec2(x + w * 0.52, y + h * 0.26),
        vec2(x + w * 0.64, y + h * 0.52),
        vec2(x + w * 0.42, y + h * 0.58),
        stone,
    );
    draw_circle(x + 22.0, y + h - 22.0, 8.0, shrub);
    draw_circle(x + 34.0, y + h - 26.0, 6.0, shrub);
}
