use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::GameData;
use crate::ui::draw_missing_area_message;

impl GameplayState {
    pub(crate) fn draw(&self, data: &GameData, art: &ArtAssets) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            draw_missing_area_message(self.missing_area_message());
            return;
        };

        let offset = self.camera_offset(area);
        self.draw_area(area, offset, data, art);
        self.draw_player(offset, art);
        self.draw_hud(area, data, art);
        self.draw_overlay_or_prompt(area, data, art);
        self.draw_sleep_flash_overlay();
    }
}
