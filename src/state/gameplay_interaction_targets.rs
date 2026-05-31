use super::GameplayState;
use crate::data::{AreaDefinition, GameData, GatherNodeDefinition, WarpDefinition};

impl GameplayState {
    pub(super) fn interaction_warp<'a>(
        &self,
        area: &'a AreaDefinition,
    ) -> Option<&'a WarpDefinition> {
        area.warps
            .iter()
            .find(|warp| {
                let position = self.world.player.position;
                warp.rect.contains_xy(position.x, position.y)
            })
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

    fn gather_node_in_reach(
        &self,
        node: &GatherNodeDefinition,
        data: &GameData,
        extra_range: f32,
    ) -> bool {
        !self.world.gathered_nodes.contains(&node.id)
            && self.player_distance_to(node.position)
                <= node.radius + data.config.interaction_range + extra_range
    }
}
