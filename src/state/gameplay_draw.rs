use super::GameplayState;
use crate::art::ArtAssets;
use crate::content::ui_copy;
use crate::data::GameData;
use macroquad::prelude::*;

impl GameplayState {
    pub(crate) fn draw(&self, data: &GameData, art: &ArtAssets) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            draw_text(ui_copy("gameplay_missing_area"), 40.0, 80.0, 32.0, RED);
            return;
        };

        let offset = self.camera_offset(area);
        self.draw_area(area, offset, data, art);
        self.draw_player(offset, art);
        self.draw_hud(area, data, art);
        self.draw_overlay_or_prompt(area, offset, data, art);
        self.draw_sleep_flash_overlay();
    }
}
