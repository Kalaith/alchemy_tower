use super::gameplay_alchemy_types::SLOT_COUNT;
use super::gameplay_alchemy_mouse_layout::{
    alchemy_slot_rect, brew_rect, catalyst_rect, clear_rect, heat_down_rect, heat_up_rect,
    material_row_rect, repeat_rect, sort_rect, stirs_rect, timing_rect,
};
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::content::ui_format;
use crate::data::{GameData, StationDefinition};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn handle_alchemy_mouse_inputs(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        items: &[String],
        audio: &AudioAssets,
    ) {
        let mouse: Vec2 = mouse_position().into();

        if self.select_alchemy_material_row(items, mouse) {
            return;
        }
        if self.toggle_alchemy_slot(data, items, mouse) {
            return;
        }
        if self.toggle_alchemy_catalyst(data, items, mouse) {
            return;
        }
        if self.handle_alchemy_control_click(data, station, audio, mouse) {
            return;
        }
    }

    pub(super) fn handle_alchemy_mouse_removals(&mut self) {
        let mouse: Vec2 = mouse_position().into();
        for slot in 0..SLOT_COUNT {
            if alchemy_slot_rect(slot).contains(mouse) {
                self.alchemy.slots[slot] = None;
                return;
            }
        }
        if catalyst_rect().contains(mouse) {
            self.alchemy.catalyst = None;
            self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
        }
    }

    fn select_alchemy_material_row(&mut self, items: &[String], mouse: Vec2) -> bool {
        for (index, _) in items.iter().enumerate() {
            if material_row_rect(index).contains(mouse) {
                self.alchemy.index = index;
                return true;
            }
        }
        false
    }

    fn toggle_alchemy_slot(&mut self, data: &GameData, items: &[String], mouse: Vec2) -> bool {
        for slot in 0..SLOT_COUNT {
            if !alchemy_slot_rect(slot).contains(mouse) {
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

    fn toggle_alchemy_catalyst(&mut self, data: &GameData, items: &[String], mouse: Vec2) -> bool {
        if !catalyst_rect().contains(mouse) {
            return false;
        }
        if self.alchemy.catalyst.is_some() {
            self.alchemy.catalyst = None;
            self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
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
        mouse: Vec2,
    ) -> bool {
        if heat_down_rect().contains(mouse) {
            self.alchemy.heat = (self.alchemy.heat - 1).max(1);
            return true;
        }
        if heat_up_rect().contains(mouse) {
            self.alchemy.heat = (self.alchemy.heat + 1).min(3);
            return true;
        }
        if stirs_rect().contains(mouse) {
            self.increment_alchemy_stirs(audio);
            return true;
        }
        if timing_rect().contains(mouse) {
            self.cycle_alchemy_timing();
            return true;
        }
        if sort_rect().contains(mouse) {
            self.cycle_inventory_sort_mode();
            self.alchemy.index = 0;
            return true;
        }
        if clear_rect().contains(mouse) {
            self.clear_alchemy_setup();
            return true;
        }
        if repeat_rect().contains(mouse) {
            self.repeat_last_brew_setup(data);
            return true;
        }
        if brew_rect().contains(mouse) {
            self.brew_selected(data, station, audio);
            return true;
        }
        false
    }
}
