use super::gameplay_npc_pathing::schedule_start_minutes;
use super::gameplay_npc_types::NpcRuntimeState;
use super::GameplayState;
use crate::data::{GameData, NpcDefinition};

impl GameplayState {
    pub(super) fn nearby_npc<'a>(&self, data: &'a GameData) -> Option<&'a NpcDefinition> {
        self.visible_npcs(data).into_iter().find(|npc| {
            self.world
                .player
                .position
                .distance(self.npc_runtime_state(data, npc).position)
                <= npc.interaction_radius
        })
    }

    pub(super) fn visible_npcs<'a>(&self, data: &'a GameData) -> Vec<&'a NpcDefinition> {
        data.npcs
            .iter()
            .filter(|npc| self.npc_runtime_state(data, npc).area_id == self.world.current_area_id)
            .collect()
    }

    pub(super) fn npc_runtime_state(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
    ) -> NpcRuntimeState {
        if let Some(tracker) = self.runtime.npc_motion_states.get(&npc.id) {
            return NpcRuntimeState {
                area_id: tracker.area_id.clone(),
                position: tracker.position,
                direction: tracker.direction,
                moving: tracker.moving,
                target_area_id: tracker.target_area_id.clone(),
            };
        }

        let tracker = self.initial_npc_motion_tracker(data, npc, 24.0);
        NpcRuntimeState {
            area_id: tracker.area_id,
            position: tracker.position,
            direction: tracker.direction,
            moving: tracker.moving,
            target_area_id: tracker.target_area_id,
        }
    }

    pub(super) fn active_schedule_index(&self, npc: &NpcDefinition) -> Option<usize> {
        if npc.schedule.is_empty() {
            return None;
        }
        let current_minutes = self.current_clock_minutes();
        let mut active_index = npc.schedule.len() - 1;
        for (index, entry) in npc.schedule.iter().enumerate() {
            if current_minutes >= schedule_start_minutes(&entry.time_window) {
                active_index = index;
            }
        }
        Some(active_index)
    }
}
