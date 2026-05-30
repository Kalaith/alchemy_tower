use macroquad::prelude::*;

pub(super) fn draw_title_plaque_caps(rect: Rect) {
    for side in [-1.0, 1.0] {
        let center = vec2(
            if side < 0.0 {
                rect.x - 6.0
            } else {
                rect.x + rect.w + 6.0
            },
            rect.y + rect.h * 0.5,
        );
        draw_poly(
            center.x + side * 2.0,
            center.y + 2.0,
            4,
            9.0,
            45.0,
            Color::from_rgba(0, 0, 0, 76),
        );
        draw_poly(
            center.x,
            center.y,
            4,
            9.0,
            45.0,
            Color::from_rgba(176, 124, 62, 194),
        );
        draw_poly(
            center.x,
            center.y,
            4,
            5.0,
            45.0,
            Color::from_rgba(242, 205, 126, 178),
        );
    }
}

pub(super) fn draw_banner_inner_hardware(rect: Rect) {
    let brass = Color::from_rgba(242, 205, 126, 116);
    let dark = Color::from_rgba(0, 0, 0, 72);
    let y_top = rect.y + 10.0;
    let y_bottom = rect.y + rect.h - 10.0;
    draw_banner_rails(rect, y_top, y_bottom, brass, dark);
    draw_banner_side_medallions(rect, brass);
    draw_banner_corner_rivets(rect);
}

fn draw_banner_rails(rect: Rect, y_top: f32, y_bottom: f32, brass: Color, dark: Color) {
    draw_line(
        rect.x + 44.0,
        y_top + 1.0,
        rect.x + rect.w - 44.0,
        y_top + 1.0,
        1.8,
        dark,
    );
    draw_line(
        rect.x + 44.0,
        y_top,
        rect.x + rect.w - 44.0,
        y_top,
        1.0,
        brass,
    );
    draw_line(
        rect.x + 44.0,
        y_bottom,
        rect.x + rect.w - 44.0,
        y_bottom,
        0.9,
        Color::from_rgba(242, 205, 126, 74),
    );
}

fn draw_banner_side_medallions(rect: Rect, brass: Color) {
    for side in [-1.0, 1.0] {
        let x = if side < 0.0 {
            rect.x + 30.0
        } else {
            rect.x + rect.w - 30.0
        };
        let center = vec2(x, rect.y + rect.h * 0.5);
        draw_circle(
            center.x + side * 1.0,
            center.y + 2.0,
            12.0,
            Color::from_rgba(0, 0, 0, 70),
        );
        draw_circle(center.x, center.y, 11.0, Color::from_rgba(101, 72, 42, 182));
        draw_circle_lines(
            center.x,
            center.y,
            11.0,
            1.0,
            Color::from_rgba(242, 205, 126, 164),
        );
        draw_poly(
            center.x,
            center.y,
            4,
            5.0,
            45.0,
            Color::from_rgba(80, 219, 205, 140),
        );
        draw_line(
            center.x + side * 12.0,
            center.y,
            center.x + side * 34.0,
            center.y,
            1.0,
            brass,
        );
    }
}

fn draw_banner_corner_rivets(rect: Rect) {
    for point in [
        vec2(rect.x + 60.0, rect.y + 14.0),
        vec2(rect.x + rect.w - 60.0, rect.y + 14.0),
        vec2(rect.x + 60.0, rect.y + rect.h - 14.0),
        vec2(rect.x + rect.w - 60.0, rect.y + rect.h - 14.0),
    ] {
        draw_circle(point.x, point.y, 2.1, Color::from_rgba(242, 205, 126, 142));
        draw_circle(
            point.x - 0.6,
            point.y - 0.6,
            0.8,
            Color::from_rgba(255, 238, 181, 150),
        );
    }
}
