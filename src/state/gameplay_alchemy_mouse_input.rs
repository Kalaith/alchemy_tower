use super::gameplay_alchemy_input_text as alchemy_input_text;
use super::gameplay_alchemy_types::SLOT_COUNT;
use super::GameplayState;
use crate::alchemy_layout::{
    alchemy_close_rect, alchemy_slot_rect, brew_rect, catalyst_rect, clear_rect, heat_down_rect,
    heat_up_rect, material_row_rect, repeat_rect, sort_rect, stirs_rect, timing_rect,
    AL_MAT_VISIBLE_ROWS,
};
use crate::audio::AudioAssets;
use crate::data::{GameData, StationDefinition};
use crate::input::{mouse_position_point, rect_contains_point};

impl GameplayState {
    pub(super) fn handle_alchemy_mouse_inputs(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        items: &[String],
        audio: &AudioAssets,
    ) {
        let mouse = mouse_position_point();

        if rect_contains_point(alchemy_close_rect(), mouse) {
            self.clear_overlay();
            self.runtime.status_text = alchemy_input_text::closed_alchemy();
            return;
        }
        if self.select_alchemy_material_row(items, mouse) {
            return;
        }
        if self.toggle_alchemy_slot(data, items, mouse) {
            return;
        }
        if self.toggle_alchemy_catalyst(data, items, mouse) {
            return;
        }
        self.handle_alchemy_control_click(data, station, audio, mouse);
    }

    pub(super) fn handle_alchemy_mouse_removals(&mut self) {
        let mouse = mouse_position_point();
        for slot in 0..SLOT_COUNT {
            if rect_contains_point(alchemy_slot_rect(slot), mouse) {
                self.alchemy.slots[slot] = None;
                return;
            }
        }
        if rect_contains_point(catalyst_rect(), mouse) {
            self.alchemy.catalyst = None;
            self.runtime.status_text = alchemy_input_text::removed_catalyst();
        }
    }

    fn select_alchemy_material_row(&mut self, items: &[String], mouse: [f32; 2]) -> bool {
        // Rows render inside a scroll window (see the materials panel), so the
        // visible row offset must be mapped back through the same window to the
        // real item index.
        let start = self
            .alchemy
            .index
            .saturating_sub(AL_MAT_VISIBLE_ROWS - 1)
            .min(items.len().saturating_sub(AL_MAT_VISIBLE_ROWS));
        let visible = AL_MAT_VISIBLE_ROWS.min(items.len().saturating_sub(start));
        for offset in 0..visible {
            if rect_contains_point(material_row_rect(offset), mouse) {
                self.alchemy.index = start + offset;
                return true;
            }
        }
        false
    }

    fn toggle_alchemy_slot(&mut self, data: &GameData, items: &[String], mouse: [f32; 2]) -> bool {
        for slot in 0..SLOT_COUNT {
            if !rect_contains_point(alchemy_slot_rect(slot), mouse) {
                continue;
            }
            if self.alchemy.slots[slot].is_some() {
                self.alchemy.slots[slot] = None;
            } else if !items.is_empty() {
                self.fill_slot(data, items, slot);
            }
            return true;
        }
        false
    }

    fn toggle_alchemy_catalyst(
        &mut self,
        data: &GameData,
        items: &[String],
        mouse: [f32; 2],
    ) -> bool {
        if !rect_contains_point(catalyst_rect(), mouse) {
            return false;
        }
        if self.alchemy.catalyst.is_some() {
            self.alchemy.catalyst = None;
            self.runtime.status_text = alchemy_input_text::removed_catalyst();
        } else if !items.is_empty() {
            self.fill_catalyst(data, items);
        }
        true
    }

    fn handle_alchemy_control_click(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        audio: &AudioAssets,
        mouse: [f32; 2],
    ) -> bool {
        if rect_contains_point(heat_down_rect(), mouse) {
            self.alchemy.heat = (self.alchemy.heat - 1).max(1);
            return true;
        }
        if rect_contains_point(heat_up_rect(), mouse) {
            self.alchemy.heat = (self.alchemy.heat + 1).min(3);
            return true;
        }
        if rect_contains_point(stirs_rect(), mouse) {
            self.increment_alchemy_stirs(audio);
            return true;
        }
        if rect_contains_point(timing_rect(), mouse) {
            self.cycle_alchemy_timing();
            return true;
        }
        if rect_contains_point(sort_rect(), mouse) {
            self.cycle_inventory_sort_mode();
            self.alchemy.index = 0;
            return true;
        }
        if rect_contains_point(clear_rect(), mouse) {
            self.clear_alchemy_setup();
            return true;
        }
        if rect_contains_point(repeat_rect(), mouse) {
            self.repeat_last_brew_setup(data);
            return true;
        }
        if rect_contains_point(brew_rect(), mouse) {
            self.brew_selected(data, station, audio);
            return true;
        }
        false
    }
}
