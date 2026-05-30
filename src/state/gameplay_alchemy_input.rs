use super::gameplay_alchemy_types::ALCHEMY_TIMINGS;
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::content::{ui_format, ui_text};
use crate::data::{GameData, StationKind};
use crate::input::{
    alchemy_brew_pressed, alchemy_catalyst_pressed, alchemy_clear_pressed,
    alchemy_clear_slot_pressed, alchemy_fill_slot_pressed, alchemy_open_pressed,
    alchemy_heat_decrease_pressed, alchemy_heat_increase_pressed, alchemy_remove_catalyst_pressed,
    alchemy_repeat_pressed, alchemy_stir_pressed, alchemy_timing_pressed, cancel_pressed,
    interact_pressed, select_next_pressed, select_previous_pressed, sort_pressed,
};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn handle_alchemy_inputs(&mut self, data: &GameData, audio: &AudioAssets) {
        let Some(station) = self.nearby_station(data).cloned() else {
            self.clear_overlay();
            return;
        };
        if station.kind != StationKind::Alchemy {
            self.clear_overlay();
            return;
        }

        if alchemy_open_pressed() || cancel_pressed() || interact_pressed() {
            self.clear_overlay();
            self.runtime.status_text = ui_text().statuses.closed_alchemy.clone();
            return;
        }

        let items = self.alchemy_materials(data);
        if !items.is_empty() {
            if select_previous_pressed() {
                self.alchemy.index = self.alchemy.index.saturating_sub(1);
            }
            if select_next_pressed() {
                self.alchemy.index = (self.alchemy.index + 1).min(items.len().saturating_sub(1));
            }
        }
        if alchemy_heat_decrease_pressed() {
            self.alchemy.heat = (self.alchemy.heat - 1).max(1);
        }
        if alchemy_heat_increase_pressed() {
            self.alchemy.heat = (self.alchemy.heat + 1).min(3);
        }
        if sort_pressed() {
            self.cycle_inventory_sort_mode();
            self.alchemy.index = 0;
        }
        if alchemy_stir_pressed() {
            self.increment_alchemy_stirs(audio);
        }
        if alchemy_timing_pressed() {
            self.cycle_alchemy_timing();
        }

        if !items.is_empty() {
            for slot in 0..3 {
                if alchemy_fill_slot_pressed(slot) {
                    self.fill_slot(data, &items, slot);
                }
            }
        }
        for slot in 0..3 {
            if alchemy_clear_slot_pressed(slot) {
                self.alchemy.slots[slot] = None;
            }
        }
        if !items.is_empty() && alchemy_catalyst_pressed() {
            self.fill_catalyst(data, &items);
        }
        if alchemy_remove_catalyst_pressed() {
            self.alchemy.catalyst = None;
            self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
        }
        if alchemy_clear_pressed() {
            self.clear_alchemy_setup();
        }
        if alchemy_repeat_pressed() {
            self.repeat_last_brew_setup(data);
        }
        if alchemy_brew_pressed() {
            self.brew_selected(data, &station, audio);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            self.handle_alchemy_mouse_inputs(data, &station, &items, audio);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.handle_alchemy_mouse_removals();
        }
    }

    pub(super) fn increment_alchemy_stirs(&mut self, audio: &AudioAssets) {
        self.alchemy.stirs += 1;
        audio.play_alchemy_stir();
        self.runtime.status_text = ui_format(
            "alchemy_stirred",
            &[("count", &self.alchemy.stirs.to_string())],
        );
    }

    pub(super) fn cycle_alchemy_timing(&mut self) {
        self.alchemy.timing_index = (self.alchemy.timing_index + 1) % ALCHEMY_TIMINGS.len();
        self.runtime.status_text =
            ui_format("alchemy_timing_set", &[("timing", self.alchemy_timing())]);
    }

    pub(super) fn clear_alchemy_setup(&mut self) {
        self.alchemy.slots = [None, None, None];
        self.alchemy.catalyst = None;
        self.alchemy.stirs = 0;
        self.alchemy.timing_index = 0;
        self.runtime.status_text = ui_format("alchemy_cleared", &[]);
    }
}
