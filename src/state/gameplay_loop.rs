use super::{GameplayState, OverlayScreen};
use crate::art::ArtAssets;
use crate::audio::AudioAssets;
use crate::content::{ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::state::StateTransition;
use macroquad::prelude::*;

impl GameplayState {
    pub fn update(&mut self, data: &GameData, audio: &AudioAssets) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Escape) {
            if let Some(overlay) = self.overlay().cloned() {
                self.clear_overlay();
                self.runtime.status_text = match overlay {
                    OverlayScreen::Alchemy => ui_text().statuses.closed_alchemy.clone(),
                    OverlayScreen::Shop => ui_text().statuses.closed_shop.clone(),
                    OverlayScreen::Rune => ui_text().statuses.closed_rune.clone(),
                    OverlayScreen::Archive => ui_text().statuses.closed_archive.clone(),
                    OverlayScreen::Ending => ui_format("gameplay_observatory_back", &[]),
                    OverlayScreen::Dialogue(_) => ui_format("gameplay_conversation_ended", &[]),
                    OverlayScreen::Journal => ui_text().statuses.closed_journal.clone(),
                    OverlayScreen::QuestBoard => ui_text().statuses.closed_quest_board.clone(),
                };
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

        if let Some(overlay) = self.overlay().cloned() {
            match overlay {
                OverlayScreen::Dialogue(_) => self.handle_dialogue_inputs(data),
                OverlayScreen::Shop => self.handle_shop_inputs(data),
                OverlayScreen::Rune => self.handle_rune_inputs(data),
                OverlayScreen::Archive => self.handle_archive_inputs(data),
                OverlayScreen::Ending => {
                    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) {
                        self.clear_overlay();
                    }
                }
                OverlayScreen::QuestBoard => self.handle_quest_board_inputs(data),
                OverlayScreen::Journal => {
                    if self.handle_journal_overlay_inputs() {
                        return None;
                    }
                }
                OverlayScreen::Alchemy => self.handle_alchemy_inputs(data, audio),
            }
        } else {
            self.handle_exploration_inputs(data, audio, frame_time);
        }

        if is_key_pressed(KeyCode::F5) {
            self.save_progress(data);
        }
        if is_key_pressed(KeyCode::F9) {
            self.load_progress(data);
        }

        None
    }

    fn handle_journal_overlay_inputs(&mut self) -> bool {
        let journal_tab_count = self.journal_tabs().len();
        self.ui.journal_tab = self.ui.journal_tab.min(journal_tab_count.saturating_sub(1));
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse = mouse_position().into();
            if self.journal_close_rect().contains(mouse) {
                self.clear_overlay();
                self.runtime.status_text = ui_text().statuses.closed_journal.clone();
                return true;
            }
            for index in 0..journal_tab_count {
                if self.journal_tab_rect(index, journal_tab_count).contains(mouse) {
                    self.ui.journal_tab = index;
                    break;
                }
            }
        }
        if is_key_pressed(KeyCode::Left) {
            self.ui.journal_tab = self.ui.journal_tab.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Right) {
            self.ui.journal_tab = (self.ui.journal_tab + 1).min(journal_tab_count.saturating_sub(1));
        }
        if is_key_pressed(KeyCode::J) {
            self.clear_overlay();
            self.runtime.status_text = ui_text().statuses.closed_journal.clone();
        }
        false
    }

    fn handle_exploration_inputs(&mut self, data: &GameData, audio: &AudioAssets, frame_time: f32) {
        if is_key_pressed(KeyCode::J) {
            self.set_overlay(OverlayScreen::Journal);
            self.ui.journal_tab = 0;
            self.runtime.status_text = ui_text().statuses.open_journal.clone();
        }
        if is_key_pressed(KeyCode::V) {
            self.cycle_inventory_sort_mode();
        }
        if self.runtime.gather_pause_seconds <= 0.0 {
            self.update_movement(data, frame_time);
            self.update_footstep_audio(audio, frame_time);
            self.handle_potion_inputs(data);
            self.handle_interactions(data, audio);
        }
    }

    pub fn draw(&self, data: &GameData, art: &ArtAssets) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            draw_text(ui_copy("gameplay_missing_area"), 40.0, 80.0, 32.0, RED);
            return;
        };

        let offset = self.camera_offset(area);
        self.draw_area(area, offset, data, art);
        self.draw_player(offset, art);
        self.draw_hud(area, data, art);
        match self.overlay() {
            Some(OverlayScreen::Dialogue(_)) => self.draw_dialogue_overlay(data),
            Some(OverlayScreen::Shop) => self.draw_shop_overlay(data),
            Some(OverlayScreen::Rune) => self.draw_rune_overlay(data),
            Some(OverlayScreen::Archive) => self.draw_archive_overlay(data),
            Some(OverlayScreen::Ending) => self.draw_ending_overlay(),
            Some(OverlayScreen::QuestBoard) => self.draw_quest_board_overlay(data),
            Some(OverlayScreen::Journal) => self.draw_field_journal(data, art),
            Some(OverlayScreen::Alchemy) => self.draw_alchemy_overlay(data, art),
            None => self.draw_prompt(area, offset, data),
        }
        self.draw_sleep_flash_overlay();
    }
}
