use super::hud_decor::*;
use super::hud_palette::*;
use super::hud_primitives::*;
use super::HudPotionSlot;
use crate::art::{draw_texture_centered, ArtAssets};
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

#[path = "hud_belt_slot_art.rs"]
mod hud_belt_slot_art;

pub(super) fn draw_hotbar_slot(rect: Rect, slot: &HudPotionSlot, art: &ArtAssets, index: usize) {
    draw_empty_hotbar_slot(rect, index);
    if !slot.key_label.is_empty() {
        draw_keycap(
            Rect::new(rect.x + 5.0, rect.y + 5.0, 20.0, 18.0),
            slot.key_label,
            true,
        );
    }
    if let Some(icon_id) = &slot.icon_id {
        hud_belt_slot_art::draw_glass_potion_bottle(rect, slot_glow(index), 0.28);
        if let Some(texture) = art.item_icon(icon_id) {
            draw_texture_centered(
                texture,
                vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.55),
                vec2(46.0, 46.0),
                WHITE,
            );
        }
        hud_belt_slot_art::draw_slot_sparkle(rect, slot_glow(index), 0.42);
        draw_ui_text(
            &slot.amount.to_string(),
            rect.x + rect.w - 17.0,
            rect.y + rect.h - 9.0,
            18.0,
            bright_ink(),
        );
    } else {
        hud_belt_slot_art::draw_glass_potion_bottle(rect, slot_glow(index), 0.34);
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
    hud_belt_slot_art::draw_slot_corner_dots(rect, glow);
    hud_belt_slot_art::draw_glass_potion_bottle(rect, glow, 0.22);
}
