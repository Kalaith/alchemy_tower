use super::GameplayState;

impl GameplayState {
    pub(super) fn player_distance_to(&self, point: [f32; 2]) -> f32 {
        let dx = self.world.player.position.x - point[0];
        let dy = self.world.player.position.y - point[1];
        (dx * dx + dy * dy).sqrt()
    }
}
