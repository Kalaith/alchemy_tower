use super::{draw_overlay_section_box, draw_overlay_section_title, GameplayState, SLOT_COUNT};
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;
use crate::ui::{
    draw_action_button, draw_selection_card, draw_state_banner, draw_wrapped_text,
};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_alchemy_materials_panel(&self, data: &GameData, x: f32, y: f32) {
        let items = self.alchemy_materials(data);
        draw_overlay_section_title(
            x + 20.0,
            y + 84.0,
            ui_copy("overlay_materials"),
            Some(&ui_format(
                "overlay_sort_mode",
                &[("mode", self.inventory_sort_label())],
            )),
        );
        draw_overlay_section_box(x + 18.0, y + 98.0, 286.0, 162.0);
        let mut iy = y + 82.0;
        if items.is_empty() {
            draw_state_banner(
                x + 30.0,
                iy - 12.0,
                262.0,
                &self.unavailable_state_text(ui_copy("overlay_alchemy_empty_materials")),
                false,
            );
        } else {
            for (index, item_id) in items.iter().enumerate() {
                let selected = index == self.alchemy.index;
                let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                let ready = amount.saturating_sub(self.reserved_count(item_id));
                let subtitle = data
                    .item(item_id)
                    .map(|item| item.description.as_str())
                    .unwrap_or("");
                draw_selection_card(
                    x + 30.0,
                    iy - 24.0,
                    262.0,
                    52.0,
                    selected,
                    ready > 0,
                    data.item_name(item_id),
                    subtitle,
                    &self.item_card_meta(
                        data,
                        item_id,
                        amount,
                        &ui_format(
                            "overlay_materials_meta",
                            &[
                                ("ready", &ready.to_string()),
                                ("reserved", &self.reserved_count(item_id).to_string()),
                                (
                                    "reference",
                                    &self.inventory_reference_summary(data, item_id),
                                ),
                            ],
                        ),
                    ),
                );
                iy += 58.0;
            }
        }

    }

    pub(super) fn draw_alchemy_controls_panel(&self, x: f32, y: f32) {
        draw_overlay_section_title(
            x + 20.0,
            y + 270.0,
            ui_copy("overlay_alchemy_controls"),
            None,
        );
        draw_overlay_section_box(x + 18.0, y + 284.0, 286.0, 74.0);
        draw_wrapped_text(
            &ui_format(
                "overlay_alchemy_controls_line1",
                &[
                    ("browse", &input_bindings().navigation.select),
                    ("heat", &input_bindings().alchemy.heat),
                ],
            ),
            x + 32.0,
            y + 304.0,
            262.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &ui_format(
                "overlay_alchemy_controls_line2",
                &[
                    ("fill", &input_bindings().alchemy.fill_slots),
                    ("catalyst", &input_bindings().alchemy.catalyst),
                ],
            ),
            x + 32.0,
            y + 334.0,
            262.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );

    }

    pub(super) fn draw_alchemy_slots_panel(&self, data: &GameData, x: f32, y: f32, w: f32) {
        draw_overlay_section_title(x + 340.0, y + 84.0, ui_copy("overlay_slots"), None);
        draw_overlay_section_box(x + 340.0, y + 98.0, w - 360.0, 134.0);
        draw_text(
            &ui_format(
                "overlay_alchemy_process",
                &[
                    ("heat", &self.alchemy.heat.to_string()),
                    ("stirs", &self.alchemy.stirs.to_string()),
                    ("timing", self.alchemy_timing()),
                ],
            ),
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
        for slot in 0..SLOT_COUNT {
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
                &ui_format("overlay_slot_label", &[("slot", &(slot + 1).to_string())]),
                sx + 16.0,
                y + 146.0,
                22.0,
                dark::TEXT_BRIGHT,
            );
            draw_text(
                self.alchemy.slots[slot]
                    .as_deref()
                    .map(|id| data.item_name(id))
                    .unwrap_or(ui_copy("overlay_alchemy_empty_slot")),
                sx + 12.0,
                y + 188.0,
                20.0,
                dark::TEXT,
            );
            draw_text(
                if self.alchemy.slots[slot].is_some() {
                    ui_copy("overlay_slot_click_clear")
                } else {
                    ui_copy("overlay_slot_click_fill")
                },
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
            self.alchemy
                .catalyst
                .as_deref()
                .map(|id| data.item_name(id))
                .unwrap_or(ui_copy("overlay_alchemy_empty_slot")),
            x + 772.0,
            y + 188.0,
            20.0,
            dark::TEXT,
        );
        draw_text(
            if self.alchemy.catalyst.is_some() {
                ui_copy("overlay_catalyst_click_clear")
            } else {
                ui_copy("overlay_catalyst_click_assign")
            },
            x + 772.0,
            y + 206.0,
            16.0,
            dark::TEXT_DIM,
        );

    }

    pub(super) fn draw_alchemy_formulae_panel(&self, data: &GameData, x: f32, y: f32) {
        draw_overlay_section_title(
            x + 20.0,
            y + 392.0,
            ui_copy("overlay_alchemy_known_formulae"),
            None,
        );
        draw_overlay_section_box(x + 18.0, y + 406.0, 286.0, 142.0);
        let mut ky = y + 424.0;
        let mut any_known = false;
        for recipe in &data.recipes {
            if self.progression.known_recipes.contains(&recipe.id) {
                any_known = true;
                draw_text(&recipe.name, x + 20.0, ky, 22.0, dark::TEXT_BRIGHT);
                ky += 22.0;
                draw_text(
                    &self.recipe_memory_meta(data, recipe),
                    x + 20.0,
                    ky,
                    18.0,
                    dark::TEXT_DIM,
                );
                ky += 20.0;
                draw_wrapped_text(
                    &self.recipe_memory_detail(data, recipe),
                    x + 20.0,
                    ky,
                    286.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                ky += 36.0;
                draw_wrapped_text(
                    &recipe.lore_note,
                    x + 20.0,
                    ky,
                    286.0,
                    16.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                ky += 32.0;
            }
        }
        if !any_known {
            draw_text(
                ui_copy("overlay_alchemy_no_formulae"),
                x + 32.0,
                ky,
                20.0,
                dark::TEXT_DIM,
            );
        }

    }

}
