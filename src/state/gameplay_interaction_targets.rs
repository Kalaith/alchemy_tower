use super::GameplayState;
use crate::data::{AreaDefinition, GameData, GatherNodeDefinition, WarpDefinition};
use macroquad::prelude::{vec2, Vec2};

impl GameplayState {
    pub(super) fn interaction_warp<'a>(
        &self,
        area: &'a AreaDefinition,
    ) -> Option<&'a WarpDefinition> {
        area.warps
            .iter()
            .find(|warp| warp.rect.contains_point(self.world.player.position))
    }

    pub(super) fn interaction_gather_node<'a>(
        &self,
        area: &'a AreaDefinition,
        data: &GameData,
    ) -> Option<&'a GatherNodeDefinition> {
        area.gather_nodes
            .iter()
            .find(|node| self.gather_node_in_reach(node, data, 0.0))
    }

    pub(super) fn world_prompt_warp<'a>(
        &self,
        area: &'a AreaDefinition,
    ) -> Option<&'a WarpDefinition> {
        self.interaction_warp(area)
    }

    pub(super) fn world_prompt_gather_node<'a>(
        &self,
        area: &'a AreaDefinition,
        data: &GameData,
    ) -> Option<&'a GatherNodeDefinition> {
        area.gather_nodes
            .iter()
            .find(|node| self.node_is_available(node) && self.gather_node_in_reach(node, data, 28.0))
    }

    pub(super) fn player_prompt_position(&self, offset: Vec2, y_offset: f32) -> Vec2 {
        let player_position = self.world.player.position;
        vec2(
            offset.x + player_position.x,
            offset.y + player_position.y + y_offset,
        )
    }

    fn gather_node_in_reach(
        &self,
        node: &GatherNodeDefinition,
        data: &GameData,
        extra_range: f32,
    ) -> bool {
        !self.world.gathered_nodes.contains(&node.id)
            && self
                .world
                .player
                .position
                .distance(vec2(node.position[0], node.position[1]))
                <= node.radius + data.config.interaction_range + extra_range
    }
}
