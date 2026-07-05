use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::input::{
    cancel_pressed, confirm_pressed, journal_pressed, left_mouse_pressed, mouse_position_point,
    rect_contains_point, switch_next_pressed, switch_previous_pressed,
};
use crate::journal_layout::{journal_close_rect, journal_tab_rect};

#[path = "gameplay_overlay_input_text.rs"]
mod overlay_input_text;

impl GameplayState {
    pub(super) fn handle_active_overlay_inputs(
        &mut self,
        data: &GameData,
        audio: &AudioAssets,
    ) -> bool {
        let Some(overlay) = self.overlay().cloned() else {
            return false;
        };

        // Overlay hit-testing reads the mouse in UI design space, so enable the
        // scale transform for the duration of this dispatch.
        crate::ui_scale::set_overlay_mouse(true);
        match overlay {
            OverlayScreen::Dialogue(_) => self.handle_dialogue_inputs(data),
            OverlayScreen::Shop => self.handle_shop_inputs(data),
            OverlayScreen::Rune => self.handle_rune_inputs(data),
            OverlayScreen::Archive => self.handle_archive_inputs(data),
            OverlayScreen::Ending => {
                if cancel_pressed() || confirm_pressed() {
                    self.clear_overlay();
                }
            }
            OverlayScreen::QuestBoard => self.handle_quest_board_inputs(data),
            OverlayScreen::Journal => self.handle_journal_overlay_inputs(),
            OverlayScreen::Alchemy => self.handle_alchemy_inputs(data, audio),
        }
        crate::ui_scale::set_overlay_mouse(false);

        true
    }

    fn handle_journal_overlay_inputs(&mut self) {
        let journal_tab_count = self.journal_tabs().len();
        self.ui.journal_tab = self.ui.journal_tab.min(journal_tab_count.saturating_sub(1));
        if left_mouse_pressed() {
            let mouse = mouse_position_point();
            if rect_contains_point(journal_close_rect(), mouse) {
                self.clear_overlay();
                self.runtime.status_text = overlay_input_text::closed_journal();
                return;
            }
            for index in 0..journal_tab_count {
                if rect_contains_point(journal_tab_rect(index, journal_tab_count), mouse) {
                    self.ui.journal_tab = index;
                    break;
                }
            }
        }
        if switch_previous_pressed() {
            self.ui.journal_tab = self.ui.journal_tab.saturating_sub(1);
        }
        if switch_next_pressed() {
            self.ui.journal_tab =
                (self.ui.journal_tab + 1).min(journal_tab_count.saturating_sub(1));
        }
        if journal_pressed() {
            self.clear_overlay();
            self.runtime.status_text = overlay_input_text::closed_journal();
        }
    }
}
