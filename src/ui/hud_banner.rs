use super::hud_primitives::*;
use super::HOTBAR_SLOT_COUNT;
use macroquad::prelude::*;

pub(super) fn draw_banner_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 38.0, rect.y - 7.0, rect.w + 76.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        15.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_beveled_rect(back, 15.0, Color::from_rgba(139, 101, 58, 172));
    draw_panel_texture(back, 15.0, Color::from_rgba(139, 101, 58, 172), 0.86);
    draw_beveled_rect_lines(back, 15.0, 1.6, Color::from_rgba(244, 204, 128, 196));
    draw_beveled_rect_lines(
        Rect::new(back.x + 8.0, back.y + 8.0, back.w - 16.0, back.h - 16.0),
        10.0,
        1.0,
        Color::from_rgba(61, 40, 24, 114),
    );
    draw_banner_wing(vec2(back.x + 9.0, back.y + back.h * 0.5), -1.0);
    draw_banner_wing(vec2(back.x + back.w - 9.0, back.y + back.h * 0.5), 1.0);
    draw_leaf_cluster_scaled(vec2(back.x + 66.0, back.y + 5.0), false, 0.36);
    draw_leaf_cluster_scaled(vec2(back.x + back.w - 66.0, back.y + 5.0), true, 0.36);
}

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

pub(super) fn draw_banner_wing(center: Vec2, direction: f32) {
    let outer = Color::from_rgba(167, 119, 61, 168);
    let trim = Color::from_rgba(242, 205, 126, 190);
    draw_triangle(
        center + vec2(direction * 2.0, -27.0),
        center + vec2(direction * 52.0, -11.0),
        center + vec2(direction * 2.0, 0.0),
        outer,
    );
    draw_triangle(
        center + vec2(direction * 2.0, 27.0),
        center + vec2(direction * 52.0, 11.0),
        center + vec2(direction * 2.0, 0.0),
        outer,
    );
    draw_line(
        center.x + direction * 7.0,
        center.y - 21.0,
        center.x + direction * 43.0,
        center.y - 8.0,
        1.3,
        trim,
    );
    draw_line(
        center.x + direction * 7.0,
        center.y + 21.0,
        center.x + direction * 43.0,
        center.y + 8.0,
        1.3,
        trim,
    );
    draw_small_diamond(center + vec2(direction * 18.0, 0.0), trim);
}

pub(super) fn draw_belt_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 20.0, rect.y - 6.0, rect.w + 40.0, rect.h + 10.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        13.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 13.0, Color::from_rgba(112, 78, 43, 138));
    draw_beveled_rect_lines(back, 13.0, 1.2, Color::from_rgba(235, 196, 118, 142));
    draw_panel_texture(back, 13.0, Color::from_rgba(112, 78, 43, 138), 0.72);
}

pub(super) fn draw_belt_hardware(rect: Rect, slot_size: f32, gap: f32) {
    let rail = Rect::new(rect.x + 26.0, rect.y + 10.0, rect.w - 52.0, rect.h - 25.0);
    draw_beveled_rect(rail, 10.0, Color::from_rgba(24, 19, 16, 92));
    draw_beveled_rect_lines(rail, 10.0, 0.9, Color::from_rgba(238, 196, 119, 76));
    draw_beveled_rect_lines(
        Rect::new(rail.x + 5.0, rail.y + 5.0, rail.w - 10.0, rail.h - 10.0),
        7.0,
        0.8,
        Color::from_rgba(255, 238, 181, 38),
    );

    for side in [-1.0, 1.0] {
        let center = vec2(
            if side < 0.0 {
                rect.x + 16.0
            } else {
                rect.x + rect.w - 16.0
            },
            rect.y + rect.h * 0.5,
        );
        draw_circle(
            center.x + side * 2.0,
            center.y + 3.0,
            12.0,
            Color::from_rgba(0, 0, 0, 58),
        );
        draw_poly(
            center.x,
            center.y,
            6,
            12.0,
            30.0,
            Color::from_rgba(116, 78, 41, 164),
        );
        draw_poly_lines(
            center.x,
            center.y,
            6,
            12.0,
            30.0,
            1.1,
            Color::from_rgba(242, 205, 126, 164),
        );
        draw_small_diamond(center, Color::from_rgba(85, 222, 207, 146));
    }

    for index in 1..HOTBAR_SLOT_COUNT {
        let x = rect.x + 40.0 + index as f32 * slot_size + (index as f32 - 0.5) * gap;
        draw_line(
            x,
            rect.y + 20.0,
            x,
            rect.y + rect.h - 35.0,
            0.8,
            Color::from_rgba(242, 205, 126, 46),
        );
        draw_circle(
            x,
            rect.y + rect.h - 23.0,
            2.0,
            Color::from_rgba(242, 205, 126, 96),
        );
    }

    draw_flourish_line(
        rect.x + rect.w * 0.5 - 52.0,
        rect.y + rect.h - 12.0,
        rect.x + rect.w * 0.5 - 10.0,
        rect.y + rect.h - 12.0,
        true,
    );
    draw_flourish_line(
        rect.x + rect.w * 0.5 + 10.0,
        rect.y + rect.h - 12.0,
        rect.x + rect.w * 0.5 + 52.0,
        rect.y + rect.h - 12.0,
        false,
    );
}
