use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::content::{ui_format, ui_text};

impl GameplayState {
    pub(super) fn closed_overlay_status(&self, overlay: &OverlayScreen) -> String {
        match overlay {
            OverlayScreen::Alchemy => ui_text().statuses.closed_alchemy.clone(),
            OverlayScreen::Shop => ui_text().statuses.closed_shop.clone(),
            OverlayScreen::Rune => ui_text().statuses.closed_rune.clone(),
            OverlayScreen::Archive => ui_text().statuses.closed_archive.clone(),
            OverlayScreen::Ending => ui_format("gameplay_observatory_back", &[]),
            OverlayScreen::Dialogue(_) => ui_format("gameplay_conversation_ended", &[]),
            OverlayScreen::Journal => ui_text().statuses.closed_journal.clone(),
            OverlayScreen::QuestBoard => ui_text().statuses.closed_quest_board.clone(),
        }
    }
}
