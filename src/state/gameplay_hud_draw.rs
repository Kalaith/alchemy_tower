use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};

impl GameplayState {
    pub(super) fn draw_hud(&self, area: &AreaDefinition, data: &GameData, art: &ArtAssets) {
        let view = self.build_hud_view(area, data);
        crate::ui::draw_hud_view(&view, art);
    }
}
