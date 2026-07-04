use super::hud::{draw_beveled_rect, draw_beveled_rect_lines, draw_panel_texture, fill_slate};
use super::{draw_action_button, draw_overlay_section_box, draw_overlay_section_title};
use crate::alchemy_layout::{
    alchemy_slot_rect_at, catalyst_rect_at, heat_down_rect_at, heat_up_rect_at, right_column_width,
    stirs_rect_at, timing_rect_at, AL_PROC_READOUT_Y, AL_RX, AL_SLOT_BOX_H, AL_SLOT_BOX_Y,
    AL_SLOT_TITLE_Y,
};
use crate::view_models::alchemy::AlchemySlotsPanelView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_alchemy_slots_panel_view(view: &AlchemySlotsPanelView, x: f32, y: f32, w: f32) {
    let rw = right_column_width(w);
    draw_overlay_section_title(x + AL_RX, y + AL_SLOT_TITLE_Y, view.title, None);
    draw_overlay_section_box(x + AL_RX - 2.0, y + AL_SLOT_BOX_Y, rw, AL_SLOT_BOX_H);

    // Process readout, then the adjustment buttons directly beneath it.
    draw_ui_text(
        &view.process_text,
        x + AL_RX + 8.0,
        y + AL_PROC_READOUT_Y,
        18.0,
        dark::TEXT_BRIGHT,
    );
    draw_action_button(heat_down_rect_at(x, y), "-", 0.0);
    draw_action_button(heat_up_rect_at(x, y), "+", 0.0);
    draw_action_button(stirs_rect_at(x, y), view.stir_label, 0.0);
    draw_action_button(timing_rect_at(x, y), view.timing_label, 0.0);

    for (slot, slot_view) in view.slots.iter().enumerate() {
        draw_slot_box(
            alchemy_slot_rect_at(x, y, slot),
            Color::from_rgba(176, 226, 255, 110),
            &slot_view.label,
            &slot_view.item_name,
            slot_view.action_text,
        );
    }

    draw_slot_box(
        catalyst_rect_at(x, y),
        Color::from_rgba(255, 214, 132, 120),
        view.catalyst_label,
        &view.catalyst.item_name,
        view.catalyst.action_text,
    );
}

fn draw_slot_box(rect: Rect, accent: Color, label: &str, item_name: &str, action_text: &str) {
    let bevel = 5.0;
    draw_beveled_rect(rect, bevel, Color::from_rgba(24, 26, 34, 236));
    draw_panel_texture(rect, bevel, fill_slate(), 0.35);
    draw_beveled_rect_lines(rect, bevel, 1.5, Color::from_rgba(223, 184, 111, 120));
    draw_rectangle(rect.x + 2.0, rect.y + 9.0, 4.0, rect.h - 18.0, accent);
    draw_ui_text(label, rect.x + 14.0, rect.y + 26.0, 20.0, dark::TEXT_BRIGHT);
    draw_ui_text(item_name, rect.x + 12.0, rect.y + 60.0, 18.0, dark::TEXT);
    draw_ui_text(
        action_text,
        rect.x + 12.0,
        rect.y + 82.0,
        15.0,
        dark::TEXT_DIM,
    );
}
