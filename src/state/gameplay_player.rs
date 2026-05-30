use super::GameplayState;
use crate::audio::AudioAssets;
use crate::data::GameData;
use crate::input::{interact_pressed, movement_direction};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn update_movement(&mut self, data: &GameData, frame_time: f32) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };
        let direction = movement_direction();
        if direction.length_squared() == 0.0 {
            self.world.player.moving = false;
            return;
        }

        let normalized = direction.normalize();
        let speed = data.config.move_speed * self.move_speed_multiplier();
        let previous = self.world.player.position;
        let candidate = self.world.player.position + normalized * speed * frame_time;
        self.world.player.position = self.resolve_area_collision(area, candidate);
        self.world.player.facing = normalized;
        self.world.player.moving = self.world.player.position.distance_squared(previous) > 0.01;
    }

    pub(super) fn update_footstep_audio(&mut self, audio: &AudioAssets, frame_time: f32) {
        if !self.world.player.moving {
            self.runtime.footstep_cooldown_seconds =
                self.runtime.footstep_cooldown_seconds.min(0.06);
            return;
        }
        let step_interval = (0.34 / self.move_speed_multiplier()).clamp(0.16, 0.34);
        if self.runtime.footstep_cooldown_seconds > 0.0 {
            self.runtime.footstep_cooldown_seconds =
                (self.runtime.footstep_cooldown_seconds - frame_time).max(0.0);
            return;
        }
        audio.play_footstep_for_area(&self.world.current_area_id);
        self.runtime.footstep_cooldown_seconds = step_interval;
    }

    pub(super) fn handle_interactions(&mut self, data: &GameData, audio: &AudioAssets) {
        if self.try_open_nearby_alchemy_shortcut(data, audio) {
            return;
        }

        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };
        if !interact_pressed() {
            return;
        }

        if let Some(npc) = self.nearby_npc(data) {
            self.handle_npc_interaction(npc);
            return;
        }

        if let Some(station) = self.nearby_station(data) {
            if self.handle_station_interaction(data, station) {
                return;
            }
        }

        if let Some(warp) = self.interaction_warp(area) {
            self.handle_warp_interaction(data, warp);
            return;
        }

        if let Some(node) = self.interaction_gather_node(area, data) {
            self.handle_gather_node_interaction(data, audio, node);
        }
    }

}
