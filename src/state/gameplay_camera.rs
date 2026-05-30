use super::gameplay_npc::npc_motion_seed;
use super::{GameplayState, NpcRuntimeState, CAMERA_PADDING, PLAYER_RADIUS};
use crate::data::{AreaDefinition, NpcDefinition};
use macroquad::prelude::{get_time, screen_height, screen_width, vec2, Vec2};

impl GameplayState {
    pub(super) fn resolve_area_collision(&self, area: &AreaDefinition, candidate: Vec2) -> Vec2 {
        let clamped = vec2(
            candidate
                .x
                .clamp(PLAYER_RADIUS, area.size[0] - PLAYER_RADIUS),
            candidate
                .y
                .clamp(PLAYER_RADIUS, area.size[1] - PLAYER_RADIUS),
        );
        if !self.collides(area, clamped) {
            return clamped;
        }
        let x_only = vec2(clamped.x, self.world.player.position.y);
        if !self.collides(area, x_only) {
            return x_only;
        }
        let y_only = vec2(self.world.player.position.x, clamped.y);
        if !self.collides(area, y_only) {
            return y_only;
        }
        self.world.player.position
    }

    pub(super) fn collides(&self, area: &AreaDefinition, point: Vec2) -> bool {
        area.blockers.iter().any(|blocker| {
            let nearest = vec2(
                point.x.clamp(blocker.x, blocker.x + blocker.w),
                point.y.clamp(blocker.y, blocker.y + blocker.h),
            );
            point.distance_squared(nearest) < PLAYER_RADIUS * PLAYER_RADIUS
        })
    }

    pub(super) fn camera_offset(&self, area: &AreaDefinition) -> Vec2 {
        let half = vec2(screen_width() * 0.5, screen_height() * 0.5);
        let unclamped = half - self.world.player.position;
        let min_x = screen_width() - area.size[0] - CAMERA_PADDING;
        let min_y = screen_height() - area.size[1] - CAMERA_PADDING;
        let mut offset = vec2(
            unclamped.x.clamp(min_x.min(CAMERA_PADDING), CAMERA_PADDING),
            unclamped.y.clamp(min_y.min(CAMERA_PADDING), CAMERA_PADDING),
        );
        if self.runtime.camera_shake_seconds > 0.0 && self.runtime.camera_shake_intensity > 0.0 {
            let t = get_time() as f32 * 45.0;
            let shake = vec2(t.sin(), (t * 1.37).cos())
                * self.runtime.camera_shake_intensity
                * (self.runtime.camera_shake_seconds / 0.25).min(1.0);
            offset += shake;
        }
        offset
    }

    pub(super) fn npc_draw_position(&self, npc: &NpcDefinition, runtime: &NpcRuntimeState) -> Vec2 {
        if !runtime.moving || runtime.direction.length_squared() <= 0.0 {
            return runtime.position;
        }
        let perpendicular = vec2(-runtime.direction.y, runtime.direction.x);
        let seed = npc_motion_seed(&npc.id);
        let sway = ((get_time() as f32 * 4.5) + seed).sin() * 1.6;
        runtime.position + perpendicular * sway
    }
}
