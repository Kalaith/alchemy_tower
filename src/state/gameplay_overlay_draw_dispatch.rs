use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_overlay_or_prompt(
        &self,
        area: &AreaDefinition,
        data: &GameData,
        art: &ArtAssets,
    ) {
        let Some(overlay) = self.overlay() else {
            self.draw_prompt(area, data);
            return;
        };

        // Dim the letterbox margins around the scaled overlay so the bare world
        // does not show brightly beside it on small windows.
        if crate::ui_scale::is_scaling() {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::from_rgba(0, 0, 0, 150),
            );
        }

        let scaled = crate::ui_scale::begin_ui_camera();
        crate::ui_scale::set_overlay_mouse(true);
        match overlay {
            OverlayScreen::Dialogue(_) => self.draw_dialogue_overlay(data),
            OverlayScreen::Shop => self.draw_shop_overlay(data),
            OverlayScreen::Rune => self.draw_rune_overlay(data),
            OverlayScreen::Archive => self.draw_archive_overlay(data),
            OverlayScreen::Ending => self.draw_ending_overlay(),
            OverlayScreen::QuestBoard => self.draw_quest_board_overlay(data),
            OverlayScreen::Journal => self.draw_field_journal(data, art),
            OverlayScreen::Alchemy => self.draw_alchemy_overlay(data, art),
        }
        crate::ui_scale::set_overlay_mouse(false);
        crate::ui_scale::end_ui_camera(scaled);
    }
}
