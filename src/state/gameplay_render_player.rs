use super::gameplay_world_types::PLAYER_RADIUS;
use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::EffectKind;
use crate::ui::draw_player_world_marker;
use macroquad::prelude::Vec2;

impl GameplayState {
    pub(super) fn draw_player(&self, offset: Vec2, art: &ArtAssets) {
        let center = offset + self.world.player.position;
        draw_player_world_marker(
            center,
            PLAYER_RADIUS,
            self.world.player.facing,
            self.world.player.moving,
            self.effect_active(EffectKind::Glow),
            art,
        );
    }
}
