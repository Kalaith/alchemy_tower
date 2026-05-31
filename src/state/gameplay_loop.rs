use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::input::{cancel_pressed, journal_pressed, sort_pressed};
use crate::state::StateTransition;
use macroquad::prelude::get_frame_time;

#[path = "gameplay_loop_status_text.rs"]
mod loop_status_text;

impl GameplayState {
    pub(crate) fn update(
        &mut self,
        data: &GameData,
        audio: &AudioAssets,
    ) -> Option<StateTransition> {
        if cancel_pressed() {
            if let Some(overlay) = self.overlay().cloned() {
                self.clear_overlay();
                self.runtime.status_text = self.closed_overlay_status(&overlay);
                return None;
            }
            return Some(StateTransition::Pause);
        }

        let frame_time = get_frame_time();
        self.world.day_clock_seconds += frame_time;
        while self.world.day_clock_seconds >= data.config.day_length_seconds {
            self.world.day_clock_seconds -= data.config.day_length_seconds;
            self.advance_to_next_day(data, true);
        }
        self.handle_sleep_pressure(data);
        self.update_area_banner(data, frame_time);
        self.update_active_effects(frame_time);
        self.update_gather_feedback(frame_time);
        self.update_npc_motion(data, frame_time);
        self.update_tutorial_hints(data, frame_time);

        if !self.handle_active_overlay_inputs(data, audio) {
            self.handle_exploration_inputs(data, audio, frame_time);
        }

        self.handle_save_shortcuts(data);

        None
    }

    fn handle_exploration_inputs(&mut self, data: &GameData, audio: &AudioAssets, frame_time: f32) {
        if journal_pressed() {
            self.set_overlay(OverlayScreen::Journal);
            self.ui.journal_tab = 0;
            self.runtime.status_text = loop_status_text::open_journal();
        }
        if sort_pressed() {
            self.cycle_inventory_sort_mode();
        }
        if self.runtime.gather_pause_seconds <= 0.0 {
            self.update_movement(data, frame_time);
            self.update_footstep_audio(audio, frame_time);
            self.handle_potion_inputs(data);
            self.handle_interactions(data, audio);
        }
    }

}
