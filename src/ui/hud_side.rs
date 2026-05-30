use super::hud_chrome::*;
use super::hud_primitives::*;
use super::HudView;
use crate::content::{input_bindings, ui_copy};
use macroquad::prelude::*;

pub(super) fn draw_side_status_panel(view: &HudView) {
    let rect = Rect::new(screen_width() - 104.0, 214.0, 84.0, 238.0);
    draw_vertical_plaque_backplate(rect);
    draw_ornate_panel(rect, Color::from_rgba(27, 25, 20, 218), 0.9);
    draw_panel_filigree(rect, 0.58);
    draw_side_status_hardware(rect);
    draw_centered_text(
        ui_copy("hud_drawer_inventory"),
        rect.x,
        rect.y + 25.0,
        rect.w,
        15.0,
        brass_light(),
    );
    draw_status_icon_medallion(
        vec2(rect.x + 30.0, rect.y + 53.0),
        Color::from_rgba(222, 174, 112, 84),
    );
    draw_bag_icon(vec2(rect.x + 29.0, rect.y + 52.0), 0.82);
    draw_text(
        &view.inventory_count.to_string(),
        rect.x + 48.0,
        rect.y + 58.0,
        18.0,
        bright_ink(),
    );
    draw_side_status_divider(rect, rect.y + 78.0);
    draw_centered_text(
        ui_copy("hud_drawer_effects"),
        rect.x,
        rect.y + 103.0,
        rect.w,
        15.0,
        brass_light(),
    );
    if view.effect_count > 0 {
        draw_status_icon_medallion(
            vec2(rect.x + 31.0, rect.y + 128.0),
            Color::from_rgba(85, 222, 207, 82),
        );
        draw_spark_icon(vec2(rect.x + 31.0, rect.y + 128.0), 0.9);
        draw_centered_text_shadowed(
            &view.effect_count.to_string(),
            rect.x,
            rect.y + 127.0,
            rect.w,
            20.0,
            bright_ink(),
        );
    } else {
        draw_centered_text(
            ui_copy("overlay_none"),
            rect.x,
            rect.y + 129.0,
            rect.w,
            17.0,
            bright_ink(),
        );
    }
    draw_side_status_divider(rect, rect.y + 156.0);
    draw_centered_text(
        ui_copy("hud_drawer_journal"),
        rect.x,
        rect.y + 188.0,
        rect.w,
        16.0,
        brass_light(),
    );
    draw_status_icon_medallion(
        vec2(rect.x + 30.0, rect.y + 213.0),
        Color::from_rgba(84, 124, 110, 88),
    );
    draw_book_icon(vec2(rect.x + 21.0, rect.y + 212.0), 0.8);
    draw_keycap(
        Rect::new(rect.x + 28.0, rect.y + 197.0, 30.0, 30.0),
        "J",
        true,
    );
}

pub(super) fn draw_side_status_hardware(rect: Rect) {
    let rail = Color::from_rgba(238, 196, 119, 116);
    let dark = Color::from_rgba(0, 0, 0, 82);
    for x in [rect.x - 5.0, rect.x + rect.w + 5.0] {
        draw_line(
            x + 1.0,
            rect.y + 25.0,
            x + 1.0,
            rect.y + rect.h - 24.0,
            2.0,
            dark,
        );
        draw_line(x, rect.y + 25.0, x, rect.y + rect.h - 24.0, 1.1, rail);
        draw_circle_lines(x, rect.y + 22.0, 5.0, 1.0, rail);
        draw_circle_lines(x, rect.y + rect.h - 21.0, 5.0, 1.0, rail);
    }

    for point in [
        vec2(rect.x + rect.w * 0.5, rect.y + 4.0),
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h - 4.0),
    ] {
        draw_poly(
            point.x + 1.0,
            point.y + 2.0,
            4,
            7.0,
            45.0,
            Color::from_rgba(0, 0, 0, 72),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            7.0,
            45.0,
            Color::from_rgba(242, 205, 126, 174),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            4.0,
            45.0,
            Color::from_rgba(52, 130, 124, 148),
        );
    }

    for point in [
        vec2(rect.x + 13.0, rect.y + 13.0),
        vec2(rect.x + rect.w - 13.0, rect.y + 13.0),
        vec2(rect.x + rect.w - 13.0, rect.y + rect.h - 13.0),
        vec2(rect.x + 13.0, rect.y + rect.h - 13.0),
    ] {
        draw_circle(point.x, point.y, 2.2, Color::from_rgba(242, 205, 126, 146));
        draw_circle(
            point.x - 0.7,
            point.y - 0.7,
            0.8,
            Color::from_rgba(255, 238, 182, 160),
        );
    }
}

pub(super) fn draw_status_icon_medallion(center: Vec2, tint: Color) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        16.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_circle(center.x, center.y, 15.0, Color::from_rgba(91, 62, 36, 166));
    draw_circle(center.x, center.y, 11.0, tint);
    draw_circle_lines(
        center.x,
        center.y,
        15.0,
        1.1,
        Color::from_rgba(242, 205, 126, 146),
    );
    draw_circle_lines(
        center.x,
        center.y,
        9.0,
        0.8,
        Color::from_rgba(255, 238, 181, 72),
    );
}

pub(super) fn draw_side_status_divider(rect: Rect, y: f32) {
    let color = Color::from_rgba(221, 174, 91, 92);
    let center = rect.x + rect.w * 0.5;
    draw_line(rect.x + 13.0, y, center - 7.0, y, 1.0, color);
    draw_line(center + 7.0, y, rect.x + rect.w - 13.0, y, 1.0, color);
    draw_small_diamond(vec2(center, y), Color::from_rgba(242, 205, 126, 128));
    draw_circle_lines(center - 18.0, y, 3.0, 0.8, color);
    draw_circle_lines(center + 18.0, y, 3.0, 0.8, color);
}

pub(super) fn draw_control_tags() {
    let x = 22.0;
    let y = screen_height() - 184.0;
    let rows = [
        (
            input_bindings().alchemy.open.as_str(),
            ui_copy("hud_control_alchemy"),
        ),
        (
            input_bindings().global.journal.as_str(),
            ui_copy("hud_drawer_journal"),
        ),
        ("V", ui_copy("hud_control_sort")),
        (
            input_bindings().global.cancel.as_str(),
            ui_copy("hud_control_pause"),
        ),
    ];
    for (index, (key, label)) in rows.iter().enumerate() {
        draw_control_tag(
            Rect::new(x, y + index as f32 * 40.0, 158.0, 32.0),
            key,
            label,
        );
    }
}

pub(super) fn draw_control_tag(rect: Rect, key: &str, label: &str) {
    draw_tag_panel(rect);
    draw_keycap(
        Rect::new(rect.x + 12.0, rect.y + 5.0, 40.0, 22.0),
        key,
        false,
    );
    draw_text(
        label,
        rect.x + 64.0,
        rect.y + 22.0,
        19.0,
        Color::from_rgba(44, 34, 26, 255),
    );
    draw_small_diamond(vec2(rect.x + rect.w - 9.0, rect.y + rect.h * 0.5), brass());
}
