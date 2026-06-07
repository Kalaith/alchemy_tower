use super::{draw_action_button, draw_overlay_section_box, draw_overlay_section_title};
use crate::alchemy_layout::{
    alchemy_slot_rect_at, catalyst_rect_at, heat_down_rect_at, heat_up_rect_at, stirs_rect_at,
    timing_rect_at,
};
use crate::view_models::alchemy::AlchemySlotsPanelView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_slots_panel_view(view: &AlchemySlotsPanelView, x: f32, y: f32, w: f32) {
    draw_overlay_section_title(x + 340.0, y + 84.0, view.title, None);
    draw_overlay_section_box(x + 340.0, y + 98.0, w - 360.0, 134.0);
    draw_text(
        &view.process_text,
        x + 340.0,
        y + 106.0,
        20.0,
        dark::TEXT_DIM,
    );
    draw_action_button(heat_down_rect_at(x, y), "-", 0.0);
    draw_action_button(heat_up_rect_at(x, y), "+", 0.0);
    draw_action_button(stirs_rect_at(x, y), view.stir_label, 0.0);
    draw_action_button(timing_rect_at(x, y), view.timing_label, 0.0);
    for (slot, slot_view) in view.slots.iter().enumerate() {
        let rect = alchemy_slot_rect_at(x, y, slot);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::from_rgba(28, 32, 42, 255),
        );
        draw_rectangle(
            rect.x,
            rect.y,
            4.0,
            rect.h,
            Color::from_rgba(176, 226, 255, 96),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.5,
            Color::from_rgba(160, 170, 190, 58),
        );
        draw_text(
            &slot_view.label,
            rect.x + 16.0,
            rect.y + 26.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &slot_view.item_name,
            rect.x + 12.0,
            rect.y + 68.0,
            20.0,
            dark::TEXT,
        );
        draw_text(
            slot_view.action_text,
            rect.x + 12.0,
            rect.y + 86.0,
            16.0,
            dark::TEXT_DIM,
        );
    }
    let catalyst = catalyst_rect_at(x, y);
    draw_rectangle(
        catalyst.x,
        catalyst.y,
        catalyst.w,
        catalyst.h,
        Color::from_rgba(28, 32, 42, 255),
    );
    draw_rectangle(
        catalyst.x,
        catalyst.y,
        4.0,
        catalyst.h,
        Color::from_rgba(255, 214, 132, 96),
    );
    draw_rectangle_lines(
        catalyst.x,
        catalyst.y,
        catalyst.w,
        catalyst.h,
        1.5,
        Color::from_rgba(160, 170, 190, 58),
    );
    draw_text(
        view.catalyst_label,
        catalyst.x + 16.0,
        catalyst.y + 26.0,
        22.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        &view.catalyst.item_name,
        catalyst.x + 12.0,
        catalyst.y + 68.0,
        20.0,
        dark::TEXT,
    );
    draw_text(
        view.catalyst.action_text,
        catalyst.x + 12.0,
        catalyst.y + 86.0,
        16.0,
        dark::TEXT_DIM,
    );
}
