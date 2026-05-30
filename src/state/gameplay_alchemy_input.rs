use super::{GameplayState, ALCHEMY_TIMINGS, SLOT_COUNT};
use crate::audio::AudioAssets;
use crate::content::{ui_format, ui_text};
use crate::data::{GameData, StationDefinition, StationKind};
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

        if is_key_pressed(KeyCode::Tab)
            || is_key_pressed(KeyCode::Escape)
            || is_key_pressed(KeyCode::E)
        {
            self.clear_overlay();
            self.runtime.status_text = ui_text().statuses.closed_alchemy.clone();
            return;
        }

        let items = self.alchemy_materials(data);
        if !items.is_empty() {
            if is_key_pressed(KeyCode::Up) {
                self.alchemy.index = self.alchemy.index.saturating_sub(1);
            }
            if is_key_pressed(KeyCode::Down) {
                self.alchemy.index = (self.alchemy.index + 1).min(items.len().saturating_sub(1));
            }
        }
        if is_key_pressed(KeyCode::Left) {
            self.alchemy.heat = (self.alchemy.heat - 1).max(1);
        }
        if is_key_pressed(KeyCode::Right) {
            self.alchemy.heat = (self.alchemy.heat + 1).min(3);
        }
        if is_key_pressed(KeyCode::V) {
            self.cycle_inventory_sort_mode();
            self.alchemy.index = 0;
        }
        if is_key_pressed(KeyCode::S) {
            self.increment_alchemy_stirs(audio);
        }
        if is_key_pressed(KeyCode::T) {
            self.cycle_alchemy_timing();
        }

        if !items.is_empty() {
            for (slot, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3]
                .iter()
                .enumerate()
            {
                if is_key_pressed(*key) {
                    self.fill_slot(data, &items, slot);
                }
            }
        }
        for (slot, key) in [KeyCode::Q, KeyCode::W, KeyCode::E].iter().enumerate() {
            if is_key_pressed(*key) {
                self.alchemy.slots[slot] = None;
            }
        }
        if !items.is_empty() && is_key_pressed(KeyCode::F) {
            self.fill_catalyst(data, &items);
        }
        if is_key_pressed(KeyCode::R) {
            self.alchemy.catalyst = None;
            self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
        }
        if is_key_pressed(KeyCode::C) {
            self.clear_alchemy_setup();
        }
        if is_key_pressed(KeyCode::Y) {
            self.repeat_last_brew_setup(data);
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::B) {
            self.brew_selected(data, &station, audio);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            self.handle_alchemy_mouse_inputs(data, &station, &items, audio);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.handle_alchemy_mouse_removals();
        }
    }

    fn handle_alchemy_mouse_inputs(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        items: &[String],
        audio: &AudioAssets,
    ) {
        let x = 80.0;
        let y = 64.0;
        let mouse: Vec2 = mouse_position().into();

        for (index, _) in items.iter().enumerate() {
            let row_rect = Rect::new(x + 18.0, y + 58.0 + index as f32 * 58.0, 286.0, 52.0);
            if row_rect.contains(mouse) {
                self.alchemy.index = index;
                return;
            }
        }

        for slot in 0..SLOT_COUNT {
            let slot_rect = Rect::new(x + 340.0 + slot as f32 * 140.0, y + 120.0, 120.0, 100.0);
            if slot_rect.contains(mouse) {
                if self.alchemy.slots[slot].is_some() {
                    self.alchemy.slots[slot] = None;
                } else if !items.is_empty() {
                    self.fill_slot(data, items, slot);
                }
                return;
            }
        }

        let catalyst_rect = Rect::new(x + 760.0, y + 120.0, 160.0, 100.0);
        if catalyst_rect.contains(mouse) {
            if self.alchemy.catalyst.is_some() {
                self.alchemy.catalyst = None;
                self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
            } else if !items.is_empty() {
                self.fill_catalyst(data, items);
            }
            return;
        }

        if Rect::new(x + 520.0, y + 88.0, 28.0, 24.0).contains(mouse) {
            self.alchemy.heat = (self.alchemy.heat - 1).max(1);
            return;
        }
        if Rect::new(x + 552.0, y + 88.0, 28.0, 24.0).contains(mouse) {
            self.alchemy.heat = (self.alchemy.heat + 1).min(3);
            return;
        }
        if Rect::new(x + 612.0, y + 88.0, 92.0, 24.0).contains(mouse) {
            self.increment_alchemy_stirs(audio);
            return;
        }
        if Rect::new(x + 716.0, y + 88.0, 156.0, 24.0).contains(mouse) {
            self.cycle_alchemy_timing();
            return;
        }
        if Rect::new(x + 20.0, y + 368.0, 82.0, 28.0).contains(mouse) {
            self.cycle_inventory_sort_mode();
            self.alchemy.index = 0;
            return;
        }
        if Rect::new(x + 114.0, y + 368.0, 82.0, 28.0).contains(mouse) {
            self.clear_alchemy_setup();
            return;
        }
        if Rect::new(x + 208.0, y + 368.0, 90.0, 28.0).contains(mouse) {
            self.repeat_last_brew_setup(data);
            return;
        }
        if Rect::new(x + 310.0, y + 368.0, 90.0, 28.0).contains(mouse) {
            self.brew_selected(data, station, audio);
        }
    }

    fn handle_alchemy_mouse_removals(&mut self) {
        let x = 80.0;
        let y = 64.0;
        let mouse: Vec2 = mouse_position().into();
        for slot in 0..SLOT_COUNT {
            let slot_rect = Rect::new(x + 340.0 + slot as f32 * 140.0, y + 120.0, 120.0, 100.0);
            if slot_rect.contains(mouse) {
                self.alchemy.slots[slot] = None;
                return;
            }
        }
        let catalyst_rect = Rect::new(x + 760.0, y + 120.0, 160.0, 100.0);
        if catalyst_rect.contains(mouse) {
            self.alchemy.catalyst = None;
            self.runtime.status_text = ui_format("alchemy_removed_catalyst", &[]);
        }
    }

    fn increment_alchemy_stirs(&mut self, audio: &AudioAssets) {
        self.alchemy.stirs += 1;
        audio.play_alchemy_stir();
        self.runtime.status_text = ui_format(
            "alchemy_stirred",
            &[("count", &self.alchemy.stirs.to_string())],
        );
    }

    fn cycle_alchemy_timing(&mut self) {
        self.alchemy.timing_index = (self.alchemy.timing_index + 1) % ALCHEMY_TIMINGS.len();
        self.runtime.status_text =
            ui_format("alchemy_timing_set", &[("timing", self.alchemy_timing())]);
    }

    fn clear_alchemy_setup(&mut self) {
        self.alchemy.slots = [None, None, None];
        self.alchemy.catalyst = None;
        self.alchemy.stirs = 0;
        self.alchemy.timing_index = 0;
        self.runtime.status_text = ui_format("alchemy_cleared", &[]);
    }
}
