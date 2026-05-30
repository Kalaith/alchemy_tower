use macroquad::prelude::*;

pub(super) fn draw_medallion_backplate(center: Vec2, radius: f32) {
    draw_circle(
        center.x + 5.0,
        center.y + 8.0,
        radius + 16.0,
        Color::from_rgba(0, 0, 0, 78),
    );
    draw_circle(
        center.x,
        center.y,
        radius + 12.0,
        Color::from_rgba(94, 70, 36, 156),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius + 12.0,
        2.0,
        Color::from_rgba(242, 205, 126, 178),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius + 6.0,
        1.0,
        Color::from_rgba(45, 30, 18, 124),
    );
    for index in 0..8 {
        let angle = index as f32 * std::f32::consts::TAU / 8.0 + 0.12;
        let base = center + vec2(angle.cos(), angle.sin()) * (radius + 10.0);
        draw_poly(
            base.x,
            base.y,
            4,
            if index % 2 == 0 { 5.0 } else { 3.5 },
            45.0,
            Color::from_rgba(242, 205, 126, 170),
        );
    }
}

pub(super) fn draw_medallion_ticks(center: Vec2, radius: f32) {
    let tick = Color::from_rgba(242, 205, 126, 132);
    for index in 0..20 {
        let angle = index as f32 * std::f32::consts::TAU / 20.0;
        let inner = center + vec2(angle.cos(), angle.sin()) * (radius - 13.0);
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius - 8.0);
        draw_line(inner.x, inner.y, outer.x, outer.y, 1.0, tick);
    }
    for index in 0..5 {
        let angle = index as f32 * std::f32::consts::TAU / 5.0 - 0.2;
        let point = center + vec2(angle.cos(), angle.sin()) * (radius - 24.0);
        draw_circle(point.x, point.y, 1.5, Color::from_rgba(102, 226, 190, 150));
    }
}
