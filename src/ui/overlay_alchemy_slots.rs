use crate::content::ui_copy;
use crate::view_models::alchemy::AlchemySlotsPanelView;
use super::{draw_action_button, draw_overlay_section_box, draw_overlay_section_title};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_slots_panel_view(
    view: &AlchemySlotsPanelView,
    x: f32,
    y: f32,
    w: f32,
) {
        draw_overlay_section_title(x + 340.0, y + 84.0, ui_copy("overlay_slots"), None);
        draw_overlay_section_box(x + 340.0, y + 98.0, w - 360.0, 134.0);
        draw_text(
            &view.process_text,
            x + 340.0,
            y + 106.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_action_button(Rect::new(x + 520.0, y + 88.0, 28.0, 28.0), "-", 0.0);
        draw_action_button(Rect::new(x + 552.0, y + 88.0, 28.0, 28.0), "+", 0.0);
        draw_action_button(
            Rect::new(x + 612.0, y + 88.0, 92.0, 28.0),
            ui_copy("overlay_alchemy_stir_button"),
            0.0,
        );
        draw_action_button(
            Rect::new(x + 716.0, y + 88.0, 156.0, 28.0),
            ui_copy("overlay_alchemy_timing_button"),
            0.0,
        );
        for (slot, slot_view) in view.slots.iter().enumerate() {
            let sx = x + 340.0 + slot as f32 * 140.0;
            draw_rectangle(
                sx,
                y + 120.0,
                120.0,
                100.0,
                Color::from_rgba(28, 32, 42, 255),
            );
            draw_rectangle(
                sx,
                y + 120.0,
                4.0,
                100.0,
                Color::from_rgba(176, 226, 255, 96),
            );
            draw_rectangle_lines(
                sx,
                y + 120.0,
                120.0,
                100.0,
                1.5,
                Color::from_rgba(160, 170, 190, 58),
            );
            draw_text(
                &slot_view.label,
                sx + 16.0,
                y + 146.0,
                22.0,
                dark::TEXT_BRIGHT,
            );
            draw_text(
                &slot_view.item_name,
                sx + 12.0,
                y + 188.0,
                20.0,
                dark::TEXT,
            );
            draw_text(
                slot_view.action_text,
                sx + 12.0,
                y + 206.0,
                16.0,
                dark::TEXT_DIM,
            );
        }
        draw_rectangle(
            x + 760.0,
            y + 120.0,
            160.0,
            100.0,
            Color::from_rgba(28, 32, 42, 255),
        );
        draw_rectangle(
            x + 760.0,
            y + 120.0,
            4.0,
            100.0,
            Color::from_rgba(255, 214, 132, 96),
        );
        draw_rectangle_lines(
            x + 760.0,
            y + 120.0,
            160.0,
            100.0,
            1.5,
            Color::from_rgba(160, 170, 190, 58),
        );
        draw_text(
            ui_copy("overlay_catalyst"),
            x + 776.0,
            y + 146.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &view.catalyst.item_name,
            x + 772.0,
            y + 188.0,
            20.0,
            dark::TEXT,
        );
        draw_text(
            view.catalyst.action_text,
            x + 772.0,
            y + 206.0,
            16.0,
            dark::TEXT_DIM,
        );
}
