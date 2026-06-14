use super::hud_belt_frame::*;
use super::hud_belt_slots::*;
use super::hud_primitives::*;
use super::truncate_text_to_width;
use super::HudView;
use crate::art::ArtAssets;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub(super) fn draw_potion_belt(view: &HudView, art: &ArtAssets) {
    let slot_size = 58.0;
    let gap = 12.0;
    let slot_count = view.potions.len();
    let width =
        40.0 + slot_size * slot_count as f32 + gap * slot_count.saturating_sub(1) as f32 + 40.0;
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

    for (index, slot) in view.potions.iter().enumerate() {
        let slot_rect = Rect::new(
            x + 40.0 + index as f32 * (slot_size + gap),
            y + 16.0,
            slot_size,
            slot_size,
        );
        draw_hotbar_slot(slot_rect, slot, art, index);
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

pub(super) fn draw_status_strip(view: &HudView) {
    if view.status_text.is_empty() {
        return;
    }

    let width = (screen_width() - 320.0).min(560.0);
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 148.0;
    let rect = Rect::new(x, y, width, 34.0);
    draw_ornate_panel(rect, Color::from_rgba(17, 17, 19, 168), 0.66);
    draw_ui_text(
        &truncate_text_to_width(&view.status_text, width - 24.0, 17.0),
        x + 12.0,
        y + 22.0,
        17.0,
        muted_ink(),
    );
}
