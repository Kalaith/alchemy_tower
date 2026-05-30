use super::GameplayState;
use super::gameplay_station_prompt::{alchemy_prompt_text, station_prompt_text};
use crate::content::{ui_copy, ui_format};
use crate::data::{AreaDefinition, GameData, StationDefinition, StationKind};
use macroquad::prelude::*;

pub(in super) struct WorldPromptView {
    pub(in super) position: Vec2,
    pub(in super) text: String,
}

impl GameplayState {
    pub(super) fn world_prompt_view(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        if let Some(prompt) = self.npc_prompt_view(offset, data) {
            return Some(prompt);
        }
        if let Some(prompt) = self.nearby_station_prompt_view(offset, data) {
            return Some(prompt);
        }
        if let Some(prompt) = self.visible_station_prompt_view(area, offset, data) {
            return Some(prompt);
        }
        if let Some(prompt) = self.warp_prompt_view(area, offset, data) {
            return Some(prompt);
        }
        self.gather_prompt_view(area, offset, data)
    }

    fn npc_prompt_view(&self, offset: Vec2, data: &GameData) -> Option<WorldPromptView> {
        let npc = self.nearby_npc(data)?;
        let npc_pos = self.npc_runtime_state(data, npc).position;
        let text = if let Some((label, _)) = self.npc_world_label(data, npc) {
            if label == ui_copy("world_marker_turn_in") {
                ui_format("world_prompt_talk_ready", &[("name", &npc.name)])
            } else if label == ui_copy("world_marker_request") {
                ui_format("world_prompt_talk_request", &[("name", &npc.name)])
            } else {
                ui_format("world_prompt_talk", &[("name", &npc.name)])
            }
        } else {
            ui_format("world_prompt_talk", &[("name", &npc.name)])
        };
        Some(WorldPromptView {
            position: vec2(offset.x + npc_pos.x, offset.y + npc_pos.y - 42.0),
            text,
        })
    }

    fn nearby_station_prompt_view(&self, offset: Vec2, data: &GameData) -> Option<WorldPromptView> {
        let station = self.nearby_station(data)?;
        let text = station_prompt_text(self, data, station);
        if text.is_empty() {
            return None;
        }
        Some(WorldPromptView {
            position: station_prompt_position(offset, station),
            text,
        })
    }

    fn visible_station_prompt_view(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        let station = self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.area_id == area.id)
            .find(|station| {
                station.kind == StationKind::Alchemy
                    && self.station_world_label(data, station).is_some()
            })?;
        Some(WorldPromptView {
            position: station_prompt_position(offset, station),
            text: alchemy_prompt_text(),
        })
    }

    fn warp_prompt_view(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        let warp = self.world_prompt_warp(area)?;
        Some(WorldPromptView {
            position: self.player_prompt_position(offset, -36.0),
            text: if self.warp_is_unlocked(warp) {
                ui_format("world_prompt_enter", &[("label", &warp.label)])
            } else if self.can_unlock_warp(warp) {
                ui_format("world_prompt_restore", &[("label", &warp.label)])
            } else {
                self.warp_lock_text(data, warp)
            },
        })
    }

    fn gather_prompt_view(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        self.world_prompt_gather_node(area, data)
            .map(|node| WorldPromptView {
                position: vec2(
                    offset.x + node.position[0],
                    offset.y + node.position[1] - node.radius - 18.0,
                ),
                text: ui_format("world_prompt_gather", &[("name", &node.name)]),
            })
    }
}

fn station_prompt_position(offset: Vec2, station: &StationDefinition) -> Vec2 {
    vec2(
        offset.x + station.position[0],
        offset.y + station.position[1] - 42.0,
    )
}
