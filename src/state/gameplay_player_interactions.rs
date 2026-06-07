use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::audio::AudioAssets;
use crate::data::{GameData, GatherNodeDefinition, NpcDefinition, StationKind, WarpDefinition};
use crate::input::{alchemy_open_pressed, interact_pressed};

#[path = "gameplay_player_interaction_text.rs"]
mod player_interaction_text;

impl GameplayState {
    pub(super) fn handle_npc_interaction(&mut self, npc: &NpcDefinition) {
        self.set_overlay(OverlayScreen::Dialogue(npc.id.clone()));
        self.runtime.status_text = player_interaction_text::talking_to(&npc.name);
    }

    pub(super) fn try_open_nearby_alchemy_shortcut(
        &mut self,
        data: &GameData,
        audio: &AudioAssets,
    ) -> bool {
        let Some(station) = self.nearby_station(data) else {
            return false;
        };
        if station.kind != StationKind::Alchemy || (!alchemy_open_pressed() && !interact_pressed())
        {
            return false;
        }

        self.set_overlay(OverlayScreen::Alchemy);
        self.alchemy.index = 0;
        self.runtime.status_text = player_interaction_text::open_alchemy();
        audio.play_alchemy_open();
        true
    }

    pub(super) fn handle_warp_interaction(&mut self, data: &GameData, warp: &WarpDefinition) {
        if !self.warp_is_unlocked(warp) {
            if self.can_unlock_warp(warp) {
                self.restore_warp_route(warp);
            } else {
                self.runtime.status_text = self.warp_lock_text(data, warp);
                return;
            }
        }
        self.world.current_area_id = warp.target_area.clone();
        self.set_player_position(warp.target_position);
        self.stop_player_motion();
        self.refresh_available_nodes(data);
        self.trigger_warp_travel_feedback(warp.target_position);
    }

    pub(super) fn handle_gather_node_interaction(
        &mut self,
        data: &GameData,
        audio: &AudioAssets,
        node: &GatherNodeDefinition,
    ) {
        if self.node_is_available(node) {
            self.world.gathered_nodes.insert(node.id.clone());
            *self.inventory.entry(node.item_id.clone()).or_insert(0) += 1;
            self.note_inventory_observation(data, &node.item_id);
            let discovery = self.record_field_discovery(data, node);
            self.trigger_gather_feedback(data, node, &discovery);
            self.runtime.status_text = self.gather_status_text(data, node, &discovery);
            audio.play_gather_pickup();
        } else {
            self.runtime.status_text = self.gather_attempt_status(data, node);
        }
    }

    fn restore_warp_route(&mut self, warp: &WarpDefinition) {
        self.pay_warp_costs(warp);
        self.progression.unlocked_warps.insert(warp.id.clone());
        for milestone in &warp.unlock_milestones {
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
        self.trigger_route_restored_feedback(
            player_interaction_text::route_restored(&warp.label),
            [
                warp.rect.x + warp.rect.w * 0.5,
                warp.rect.y + warp.rect.h * 0.5,
            ],
        );
        self.runtime.status_text = player_interaction_text::repaired_access(&warp.label);
    }
}
