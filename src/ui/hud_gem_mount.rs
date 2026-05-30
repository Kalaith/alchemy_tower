use macroquad::prelude::*;

pub(super) fn draw_gem_mount(center: Vec2) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        19.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_circle(center.x, center.y, 18.0, Color::from_rgba(95, 67, 37, 178));
    draw_circle_lines(
        center.x,
        center.y,
        18.0,
        1.2,
        Color::from_rgba(242, 205, 126, 166),
    );
    for angle in [
        0.0_f32,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    ] {
        let point = center + vec2(angle.cos(), angle.sin()) * 16.0;
        draw_circle(point.x, point.y, 1.8, Color::from_rgba(242, 205, 126, 148));
    }
}
