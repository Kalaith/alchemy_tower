use super::{color_from_option, rgba};
use crate::data::AreaDefinition;
use macroquad::prelude::*;

pub(super) fn draw_shelf_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let wood = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(124, 92, 70, 255),
    );
    let top = color_from_option(
        area.render.blocker_secondary,
        Color::from_rgba(158, 122, 94, 255),
    );
    let detail = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(92, 62, 46, 255),
    );
    let bottle_a = color_from_option(
        area.render.blocker_alt,
        Color::from_rgba(170, 222, 210, 255),
    );

    draw_rectangle(x, y, w, h, wood);
    draw_rectangle(x + 6.0, y + 6.0, w - 12.0, h - 12.0, top);
    for shelf in 0..(h / 26.0).max(1.0) as i32 {
        let sy = y + 14.0 + shelf as f32 * 26.0;
        if sy < y + h - 10.0 {
            draw_line(x + 10.0, sy, x + w - 10.0, sy, 2.0, detail);
        }
    }
    for bottle in 0..3 {
        let bx = x + 18.0 + bottle as f32 * ((w - 36.0) / 3.0);
        draw_rectangle(bx, y + 14.0, 10.0, 18.0, bottle_a);
        draw_rectangle(
            bx + 2.0,
            y + 34.0,
            6.0,
            12.0,
            Color::from_rgba(255, 214, 132, 255),
        );
    }
}

pub(super) fn draw_house_blocker(
    area: &AreaDefinition,
    index: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let wall = color_from_option(
        area.render.blocker_primary,
        Color::from_rgba(204, 184, 150, 255),
    );
    let roof = if index.is_multiple_of(2) {
        color_from_option(
            area.render.blocker_secondary,
            Color::from_rgba(160, 104, 78, 255),
        )
    } else {
        color_from_option(area.render.blocker_alt, Color::from_rgba(142, 118, 82, 255))
    };
    let doorway = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(120, 94, 72, 255),
    );

    draw_rectangle(x, y, w, h, wall);
    draw_rectangle(x - 4.0, y - 8.0, w + 8.0, 18.0, roof);
    draw_rectangle(x + 12.0, y + 18.0, w - 24.0, h - 28.0, doorway);
}

pub(super) fn draw_panel_blocker(area: &AreaDefinition, x: f32, y: f32, w: f32, h: f32) {
    let outer = color_from_option(area.render.blocker_primary, rgba(area.accent));
    let inner = color_from_option(
        area.render.blocker_secondary,
        Color::new(
            (outer.r + 0.12).min(1.0),
            (outer.g + 0.12).min(1.0),
            (outer.b + 0.12).min(1.0),
            1.0,
        ),
    );
    let detail = color_from_option(
        area.render.blocker_detail,
        Color::from_rgba(240, 238, 220, 100),
    );

    draw_rectangle(x, y, w, h, outer);
    draw_rectangle(x + 6.0, y + 6.0, w - 12.0, h - 12.0, inner);
    draw_rectangle_lines(x, y, w, h, 2.0, detail);
}
