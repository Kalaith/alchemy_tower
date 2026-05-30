use super::hud_decor::*;
use super::hud_palette::*;
use super::hud_primitives::*;
use super::HudPotionSlot;
use crate::art::{draw_texture_centered, ArtAssets};
use macroquad::prelude::*;

pub(super) fn draw_hotbar_slot(rect: Rect, slot: &HudPotionSlot, art: &ArtAssets, index: usize) {
    draw_empty_hotbar_slot(rect, index);
    draw_keycap(
        Rect::new(rect.x + 5.0, rect.y + 5.0, 20.0, 18.0),
        slot.key_label,
        true,
    );
    if let Some(icon_id) = &slot.icon_id {
        draw_glass_potion_bottle(rect, slot_glow(index), 0.28);
        if let Some(texture) = art.item_icon(icon_id) {
            draw_texture_centered(
                texture,
                vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.55),
                vec2(46.0, 46.0),
                WHITE,
            );
        }
        draw_slot_sparkle(rect, slot_glow(index), 0.42);
        draw_text(
            &slot.amount.to_string(),
            rect.x + rect.w - 17.0,
            rect.y + rect.h - 9.0,
            18.0,
            bright_ink(),
        );
    } else {
        draw_glass_potion_bottle(rect, slot_glow(index), 0.34);
    }
}

pub(super) fn draw_empty_hotbar_slot(rect: Rect, index: usize) {
    let glow = slot_glow(index);
    let bevel = 7.0;
    draw_beveled_rect(
        Rect::new(rect.x + 3.0, rect.y + 5.0, rect.w, rect.h),
        bevel,
        Color::from_rgba(4, 4, 6, 72),
    );
    draw_beveled_rect(rect, bevel, Color::from_rgba(38, 32, 27, 202));
    draw_panel_texture(rect, bevel, Color::from_rgba(38, 32, 27, 202), 0.7);
    draw_beveled_rect_lines(rect, bevel, 1.5, Color::from_rgba(223, 184, 111, 154));
    draw_beveled_rect_lines(
        Rect::new(rect.x + 2.0, rect.y + 2.0, rect.w - 4.0, rect.h - 4.0),
        5.0,
        1.0,
        Color::new(glow.r, glow.g, glow.b, 0.25),
    );
    draw_beveled_rect_lines(
        Rect::new(rect.x + 5.0, rect.y + 5.0, rect.w - 10.0, rect.h - 10.0),
        4.0,
        1.0,
        Color::from_rgba(255, 236, 180, 42),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.52,
        18.0,
        Color::new(glow.r, glow.g, glow.b, 0.08),
    );
    draw_line(
        rect.x + 12.0,
        rect.y + 12.0,
        rect.x + rect.w - 12.0,
        rect.y + 8.0,
        1.0,
        Color::new(glow.r, glow.g, glow.b, 0.2),
    );
    draw_slot_corner_dots(rect, glow);
    draw_glass_potion_bottle(rect, glow, 0.22);
}

fn draw_slot_corner_dots(rect: Rect, glow: Color) {
    let color = Color::new(glow.r, glow.g, glow.b, 0.34);
    for point in [
        vec2(rect.x + 7.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 7.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 7.0, rect.y + rect.h - 7.0),
        vec2(rect.x + 7.0, rect.y + rect.h - 7.0),
    ] {
        draw_circle(point.x, point.y, 1.5, color);
    }
}

fn draw_glass_potion_bottle(rect: Rect, liquid: Color, alpha: f32) {
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 16.0;
    let glass = Color::new(parchment().r, parchment().g, parchment().b, alpha * 0.9);
    let outline = Color::new(
        brass_light().r,
        brass_light().g,
        brass_light().b,
        alpha * 0.8,
    );
    let liquid_color = Color::new(liquid.r, liquid.g, liquid.b, alpha);

    draw_rectangle(cx - 5.0, top, 10.0, 13.0, glass);
    draw_rectangle(
        cx - 8.0,
        top - 3.0,
        16.0,
        5.0,
        Color::new(0.44, 0.33, 0.24, alpha),
    );
    draw_rectangle(cx - 13.0, top + 15.0, 26.0, 21.0, glass);
    draw_circle(cx, top + 38.0, 14.0, glass);
    draw_rectangle(cx - 10.0, top + 29.0, 20.0, 11.0, liquid_color);
    draw_circle(cx, top + 38.0, 10.5, liquid_color);
    draw_circle(
        cx - 5.0,
        top + 26.0,
        3.0,
        Color::new(1.0, 1.0, 1.0, alpha * 0.72),
    );
    draw_line(cx - 9.0, top + 17.0, cx - 12.0, top + 35.0, 1.2, outline);
    draw_line(cx + 9.0, top + 17.0, cx + 12.0, top + 35.0, 1.2, outline);
    draw_circle_lines(cx, top + 38.0, 14.0, 1.2, outline);
}

fn draw_slot_sparkle(rect: Rect, color: Color, alpha: f32) {
    let center = vec2(rect.x + rect.w - 12.0, rect.y + 15.0);
    let tint = Color::new(color.r, color.g, color.b, alpha);
    draw_line(
        center.x - 5.0,
        center.y,
        center.x + 5.0,
        center.y,
        1.0,
        tint,
    );
    draw_line(
        center.x,
        center.y - 5.0,
        center.x,
        center.y + 5.0,
        1.0,
        tint,
    );
    draw_circle(center.x, center.y, 1.5, bright_ink());
}
