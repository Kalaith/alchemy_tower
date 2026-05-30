use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};
use crate::ui::{draw_area_background, draw_area_blockers};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_area(&self, area: &AreaDefinition, offset: Vec2, data: &GameData, art: &ArtAssets) {
        draw_area_background(area, offset, art);
        self.draw_environment_overlay(area, offset);
        self.draw_phase1_story_flourishes(area, offset);
        draw_area_blockers(area, offset);
        self.draw_area_warps(area, offset, art);
        self.draw_area_stations(area, offset, data, art);
        self.draw_area_npcs(area, offset, data, art);
        self.draw_area_gather_nodes(area, offset, data, art);
    }

}
