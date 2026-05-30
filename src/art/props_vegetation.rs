use super::color_from_option;
use crate::data::AreaDefinition;
use macroquad::prelude::*;

pub(super) fn draw_grass_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let base = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(104, 146, 82, 255),
    );
    let light = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(146, 186, 104, 255),
    );
    let flower = color_from_option(
        area.render.blocker_alt,
        Color::from_rgba(236, 218, 132, 255),
    );

    draw_rectangle(x, y, w, h, base);
    for tuft in 0..6 {
        let tx = x + 18.0 + tuft as f32 * ((w - 36.0) / 6.0);
        let sway = if tuft % 2 == 0 { -6.0 } else { 6.0 };
        draw_triangle(
            vec2(tx - 10.0, y + h),
            vec2(tx, y + 18.0 + (tuft % 3) as f32 * 8.0),
            vec2(tx + 10.0 + sway, y + h - 10.0),
            light,
        );
        if tuft % 2 == 0 {
            draw_circle(tx + 6.0, y + 26.0 + tuft as f32 * 3.0, 4.0, flower);
        }
    }
}

pub(super) fn draw_forest_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let trunk = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(84, 58, 42, 255),
    );
    let canopy = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(58, 102, 74, 255),
    );
    let dark = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(34, 56, 42, 255),
    );
    let glow = color_from_option(
        area.render.blocker_alt,
        Color::from_rgba(168, 224, 178, 180),
    );

    draw_rectangle(x, y, w, h, dark);
    for tree in 0..3 {
        let tx = x + w * (0.22 + tree as f32 * 0.28);
        draw_rectangle(tx - 8.0, y + h * 0.45, 16.0, h * 0.38, trunk);
        draw_circle(tx, y + h * 0.34, h.min(w) * 0.16, canopy);
        draw_circle(tx - 18.0, y + h * 0.38, h.min(w) * 0.12, canopy);
        draw_circle(tx + 18.0, y + h * 0.38, h.min(w) * 0.12, canopy);
    }
    draw_circle(x + w * 0.72, y + h * 0.28, 10.0, glow);
}

pub(super) fn draw_rainforest_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let root = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(92, 70, 54, 255),
    );
    let leaf = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(46, 118, 82, 255),
    );
    let deep = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(26, 74, 52, 255),
    );
    let mist = color_from_option(
        area.render.blocker_alt,
        Color::from_rgba(182, 232, 214, 120),
    );

    draw_rectangle(x, y, w, h, deep);
    for cluster in 0..3 {
        let cx = x + w * (0.24 + cluster as f32 * 0.28);
        draw_rectangle(cx - 10.0, y + h * 0.46, 20.0, h * 0.36, root);
        draw_circle(cx, y + h * 0.28, h.min(w) * 0.18, leaf);
        draw_triangle(
            vec2(cx - 26.0, y + h * 0.56),
            vec2(cx - 4.0, y + h * 0.44),
            vec2(cx - 12.0, y + h * 0.76),
            root,
        );
        draw_triangle(
            vec2(cx + 26.0, y + h * 0.56),
            vec2(cx + 4.0, y + h * 0.44),
            vec2(cx + 12.0, y + h * 0.76),
            root,
        );
    }
    draw_circle(x + w * 0.82, y + h * 0.26, 12.0, mist);
}
