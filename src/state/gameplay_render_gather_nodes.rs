use super::gameplay_support::rgba;
use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};
use crate::ui::draw_gather_node_world_marker;
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_area_gather_nodes(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
        art: &ArtAssets,
    ) {
        for node in &area.gather_nodes {
            if self.world.gathered_nodes.contains(&node.id) {
                continue;
            }
            let available = self.node_is_available(node);
            if !available {
                continue;
            }
            let color = rgba(node.color);
            let center = vec2(offset.x + node.position[0], offset.y + node.position[1]);
            draw_gather_node_world_marker(
                node,
                data.item(&node.item_id).map(|item| item.category),
                center,
                color,
                available,
                art,
            );
        }
    }
}
