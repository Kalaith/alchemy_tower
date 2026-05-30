use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};
use macroquad::prelude::Vec2;

impl GameplayState {
    pub(super) fn draw_overlay_or_prompt(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
        art: &ArtAssets,
    ) {
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
    }
}
