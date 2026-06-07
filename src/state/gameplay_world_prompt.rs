use super::gameplay_station_prompt::{alchemy_prompt_text, station_prompt_text};
use super::GameplayState;
use crate::content::ui_copy;
use crate::data::{AreaDefinition, GameData, StationKind};
use crate::view_models::prompt::WorldPromptView;

impl GameplayState {
    pub(super) fn world_prompt_view(
        &self,
        area: &AreaDefinition,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        if let Some(prompt) = self.npc_prompt_view(data) {
            return Some(prompt);
        }
        if let Some(prompt) = self.nearby_station_prompt_view(data) {
            return Some(prompt);
        }
        if let Some(prompt) = self.visible_station_prompt_view(area, data) {
            return Some(prompt);
        }
        if let Some(prompt) = self.warp_prompt_view(area, data) {
            return Some(prompt);
        }
        self.gather_prompt_view(area, data)
    }

    fn npc_prompt_view(&self, data: &GameData) -> Option<WorldPromptView> {
        let npc = self.nearby_npc(data)?;
        let text = if let Some((label, _)) = self.npc_world_label(data, npc) {
            if label == ui_copy("world_marker_turn_in") {
                self.interact_prompt_copy("world_prompt_talk_ready", &[("name", &npc.name)])
            } else if label == ui_copy("world_marker_request") {
                self.interact_prompt_copy("world_prompt_talk_request", &[("name", &npc.name)])
            } else {
                self.interact_prompt_copy("world_prompt_talk", &[("name", &npc.name)])
            }
        } else {
            self.interact_prompt_copy("world_prompt_talk", &[("name", &npc.name)])
        };
        Some(WorldPromptView { text })
    }

    fn nearby_station_prompt_view(&self, data: &GameData) -> Option<WorldPromptView> {
        let station = self.nearby_station(data)?;
        let text = station_prompt_text(self, data, station);
        if text.is_empty() {
            return None;
        }
        Some(WorldPromptView { text })
    }

    fn visible_station_prompt_view(
        &self,
        area: &AreaDefinition,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        self.visible_stations(data)
            .into_iter()
            .filter(|station| station.area_id == area.id)
            .find(|station| {
                station.kind == StationKind::Alchemy
                    && self.station_world_label(data, station).is_some()
            })?;
        Some(WorldPromptView {
            text: alchemy_prompt_text(self),
        })
    }

    fn warp_prompt_view(&self, area: &AreaDefinition, data: &GameData) -> Option<WorldPromptView> {
        let warp = self.world_prompt_warp(area)?;
        Some(WorldPromptView {
            text: if self.warp_is_unlocked(warp) {
                self.interact_prompt_copy("world_prompt_enter", &[("label", &warp.label)])
            } else if self.can_unlock_warp(warp) {
                self.interact_prompt_copy("world_prompt_restore", &[("label", &warp.label)])
            } else {
                self.warp_lock_text(data, warp)
            },
        })
    }

    fn gather_prompt_view(
        &self,
        area: &AreaDefinition,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        self.world_prompt_gather_node(area, data)
            .map(|node| WorldPromptView {
                text: self.interact_prompt_copy("world_prompt_gather", &[("name", &node.name)]),
            })
    }
}
