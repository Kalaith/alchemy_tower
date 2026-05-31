use super::GameplayState;
use macroquad::prelude::vec2;

impl GameplayState {
    pub(super) fn set_player_position(&mut self, position: [f32; 2]) {
        self.world.player.position = vec2(position[0], position[1]);
    }

    pub(super) fn set_player_facing(&mut self, facing: [f32; 2]) {
        self.world.player.facing = vec2(facing[0], facing[1]);
    }

    pub(super) fn stop_player_motion(&mut self) {
        self.world.player.moving = false;
    }
}
