use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::AreaDefinition;
use crate::ui::draw_warp_marker;
use macroquad::prelude::{vec2, Rect, Vec2};

impl GameplayState {
    pub(super) fn draw_area_warps(&self, area: &AreaDefinition, offset: Vec2, art: &ArtAssets) {
        for warp in &area.warps {
            let center = vec2(
                offset.x + warp.rect.x + warp.rect.w * 0.5,
                offset.y + warp.rect.y + warp.rect.h * 0.5,
            );
            let unlock_ready = !self.warp_is_unlocked(warp) && self.can_unlock_warp(warp);
            let rect = Rect::new(warp.rect.x, warp.rect.y, warp.rect.w, warp.rect.h);
            draw_warp_marker(rect, center, offset, unlock_ready, art);
        }
    }
}
