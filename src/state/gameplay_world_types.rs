use std::collections::HashSet;

use macroquad::prelude::vec2;
use macroquad::prelude::Vec2;

use crate::data::GameData;

pub(super) const PLAYER_RADIUS: f32 = 14.0;
pub(super) const CAMERA_PADDING: f32 = 160.0;

#[derive(Clone, Debug)]
pub(super) struct PlayerAvatar {
    pub(super) position: Vec2,
    pub(super) facing: Vec2,
    pub(super) moving: bool,
}

#[derive(Clone, Debug)]
pub(super) struct WorldState {
    pub(super) current_area_id: String,
    pub(super) player: PlayerAvatar,
    pub(super) day_index: u32,
    pub(super) day_clock_seconds: f32,
    pub(super) day_length_seconds: f32,
    pub(super) gathered_nodes: HashSet<String>,
    pub(super) available_nodes: HashSet<String>,
}

impl WorldState {
    pub(super) fn new(data: &GameData, day_clock_seconds: f32) -> Self {
        Self {
            current_area_id: data.config.starting_area.clone(),
            player: PlayerAvatar {
                position: vec2(
                    data.config.starting_position[0],
                    data.config.starting_position[1],
                ),
                facing: vec2(0.0, 1.0),
                moving: false,
            },
            day_index: 0,
            day_clock_seconds,
            day_length_seconds: data.config.day_length_seconds,
            gathered_nodes: HashSet::new(),
            available_nodes: HashSet::new(),
        }
    }
}
