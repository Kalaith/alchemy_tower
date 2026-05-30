use super::hud_banner::*;
use super::hud_primitives::*;
use super::{HudPotionSlot, HudView, HOTBAR_SLOT_COUNT};
use crate::art::{draw_texture_centered, ArtAssets};
use crate::ui::truncate_text_to_width;
use macroquad::prelude::*;

pub(super) fn draw_potion_belt(view: &HudView, art: &ArtAssets) {
    let slot_size = 58.0;
    let gap = 12.0;
    let width = 40.0 + slot_size * HOTBAR_SLOT_COUNT as f32 + gap * 7.0 + 40.0;
    let height = 96.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 108.0;
    let rect = Rect::new(x, y, width, height);
    draw_belt_backplate(rect);
    draw_ornate_panel(rect, Color::from_rgba(43, 35, 28, 224), 0.92);
    draw_flourish_line(
        rect.x - 26.0,
        rect.y + rect.h * 0.5,
        rect.x + 20.0,
        rect.y + rect.h * 0.5,
        true,
    );
    draw_flourish_line(
        rect.x + rect.w - 20.0,
        rect.y + rect.h * 0.5,
        rect.x + rect.w + 26.0,
        rect.y + rect.h * 0.5,
        false,
    );
    draw_line(
        rect.x + 28.0,
        rect.y + 13.0,
        rect.x + rect.w - 28.0,
        rect.y + 13.0,
        1.0,
        Color::from_rgba(255, 236, 180, 48),
    );
    draw_line(
        rect.x + 28.0,
        rect.y + rect.h - 18.0,
        rect.x + rect.w - 28.0,
        rect.y + rect.h - 18.0,
        1.0,
        Color::from_rgba(0, 0, 0, 92),
    );
    draw_belt_hardware(rect, slot_size, gap);
    draw_gem(vec2(rect.x + rect.w * 0.5, rect.y + rect.h + 2.0), 9.0);

    for index in 0..HOTBAR_SLOT_COUNT {
        let slot_rect = Rect::new(
            x + 40.0 + index as f32 * (slot_size + gap),
            y + 16.0,
            slot_size,
            slot_size,
        );
        if let Some(slot) = view.potions.get(index) {
            draw_hotbar_slot(slot_rect, slot, art, index);
        } else {
            draw_empty_hotbar_slot(slot_rect, index);
        }
        draw_centered_text_shadowed(
            &(index + 1).to_string(),
            slot_rect.x,
            slot_rect.y + 85.0,
            slot_rect.w,
            20.0,
            bright_ink(),
        );
    }
}

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

pub(super) fn draw_slot_corner_dots(rect: Rect, glow: Color) {
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

pub(super) fn draw_bottle_silhouette(rect: Rect, alpha: f32) {
    let color = Color::new(parchment().r, parchment().g, parchment().b, alpha);
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 17.0;
    draw_rectangle(cx - 5.0, top, 10.0, 13.0, color);
    draw_rectangle(cx - 12.0, top + 13.0, 24.0, 23.0, color);
    draw_circle(cx, top + 37.0, 12.0, color);
    draw_rectangle_lines(
        cx - 12.0,
        top + 13.0,
        24.0,
        24.0,
        1.0,
        Color::new(
            brass_light().r,
            brass_light().g,
            brass_light().b,
            alpha * 0.8,
        ),
    );
}

pub(super) fn draw_glass_potion_bottle(rect: Rect, liquid: Color, alpha: f32) {
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

pub(super) fn draw_slot_sparkle(rect: Rect, color: Color, alpha: f32) {
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

pub(super) fn draw_status_strip(view: &HudView) {
    if view.status_text.is_empty() {
        return;
    }

    let width = (screen_width() - 320.0).min(560.0);
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 148.0;
    let rect = Rect::new(x, y, width, 34.0);
    draw_ornate_panel(rect, Color::from_rgba(17, 17, 19, 168), 0.66);
    draw_text(
        &truncate_text_to_width(&view.status_text, width - 24.0, 17.0),
        x + 12.0,
        y + 22.0,
        17.0,
        muted_ink(),
    );
}
