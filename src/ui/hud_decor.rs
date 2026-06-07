pub(super) use super::hud_gem_icons::*;
pub(super) use super::hud_icons::*;
use super::hud_palette::*;
use super::hud_shapes::*;
use super::hud_text::*;
use crate::art::{draw_texture_centered, ArtAssets};
use macroquad::prelude::*;

pub(super) fn draw_goal_item_badge(rect: Rect, icon_id: &str, amount_text: &str, art: &ArtAssets) {
    let badge = Rect::new(rect.x + rect.w - 76.0, rect.y + 100.0, 48.0, 58.0);
    draw_beveled_rect(
        Rect::new(badge.x + 3.0, badge.y + 4.0, badge.w, badge.h),
        7.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_beveled_rect(badge, 7.0, Color::from_rgba(42, 35, 29, 204));
    draw_beveled_rect_lines(badge, 7.0, 1.2, Color::from_rgba(223, 184, 111, 150));

    if let Some(texture) = art.item_icon(icon_id) {
        draw_texture_centered(
            texture,
            vec2(badge.x + badge.w * 0.5, badge.y + 25.0),
            vec2(34.0, 34.0),
            WHITE,
        );
    } else {
        draw_bottle_silhouette(Rect::new(badge.x + 7.0, badge.y + 6.0, 34.0, 34.0), 0.44);
    }

    draw_centered_text(
        amount_text,
        badge.x,
        badge.y + badge.h - 8.0,
        badge.w,
        13.0,
        parchment(),
    );
}

pub(super) fn draw_flourish_line(x1: f32, y1: f32, x2: f32, y2: f32, left: bool) {
    let color = Color::from_rgba(221, 177, 96, 185);
    draw_line(x1, y1, x2, y2, 2.0, color);
    let sign = if left { 1.0 } else { -1.0 };
    let curl_x = if left { x1 + 22.0 } else { x1 - 22.0 };
    draw_circle_lines(curl_x, y1, 10.0, 1.5, color);
    draw_line(x2, y2, x2 + sign * 18.0, y2 - 12.0, 1.5, color);
    draw_line(x2, y2, x2 + sign * 18.0, y2 + 12.0, 1.5, color);
}

pub(super) fn draw_title_vines(x: f32, y: f32, width: f32) {
    let color = Color::from_rgba(221, 177, 96, 150);
    draw_line(x + 42.0, y + 8.0, x + 118.0, y + 8.0, 1.5, color);
    draw_line(
        x + width - 118.0,
        y + 8.0,
        x + width - 42.0,
        y + 8.0,
        1.5,
        color,
    );
    draw_circle_lines(x + 126.0, y + 8.0, 8.0, 1.2, color);
    draw_circle_lines(x + width - 126.0, y + 8.0, 8.0, 1.2, color);
    draw_leaf_cluster_scaled(vec2(x + 58.0, y + 9.0), false, 0.42);
    draw_leaf_cluster_scaled(vec2(x + width - 58.0, y + 9.0), true, 0.42);
}

pub(super) fn draw_keycap(rect: Rect, key: &str, blue: bool) {
    let fill = if blue {
        Color::from_rgba(39, 75, 110, 235)
    } else {
        Color::from_rgba(29, 31, 36, 235)
    };
    draw_rectangle(rect.x + 2.0, rect.y + 3.0, rect.w, rect.h, shadow());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        Color::from_rgba(239, 217, 174, 190),
    );
    draw_centered_text(
        key,
        rect.x,
        rect.y + rect.h - 5.0,
        rect.w,
        15.0,
        bright_ink(),
    );
}
