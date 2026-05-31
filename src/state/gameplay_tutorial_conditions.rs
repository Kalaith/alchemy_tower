use super::GameplayState;
use crate::data::{GameData, StationKind};

impl GameplayState {
    pub(super) fn tutorial_near_alchemy_station(&self, data: &GameData) -> bool {
        self.nearby_station(data)
            .map(|station| station.kind == StationKind::Alchemy)
            .unwrap_or(false)
    }

    pub(super) fn tutorial_near_quest_npc(&self, data: &GameData) -> bool {
        self.nearby_npc(data)
            .and_then(|npc| self.npc_world_label(data, npc))
            .is_some()
    }

    pub(super) fn tutorial_near_available_gather_node(&self, data: &GameData) -> bool {
        data.area(&self.world.current_area_id)
            .map(|area| {
                area.gather_nodes.iter().any(|node| {
                    !self.world.gathered_nodes.contains(&node.id)
                        && self.node_is_available(node)
                        && self.player_distance_to(node.position)
                            <= node.radius + data.config.interaction_range + 36.0
                })
            })
            .unwrap_or(false)
    }

    pub(super) fn tutorial_unlockable_warp_here(&self, data: &GameData) -> bool {
        data.area(&self.world.current_area_id)
            .map(|area| {
                area.warps
                    .iter()
                    .any(|warp| !self.warp_is_unlocked(warp) && self.can_unlock_warp(warp))
            })
            .unwrap_or(false)
    }

    pub(super) fn tutorial_delivery_ready(&self, data: &GameData) -> bool {
        self.progression
            .started_quests
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .any(|quest| self.quest_requirements_met(data, quest))
    }
}
