use super::GameplayState;
use crate::content::ui_copy;
use crate::data::{GameData, StationKind};
use macroquad::prelude::{vec2, Color};

impl GameplayState {
    pub(super) fn update_tutorial_hints(&mut self, data: &GameData, frame_time: f32) {
        if self.runtime.tutorial.next_hint_delay_seconds > 0.0 {
            self.runtime.tutorial.next_hint_delay_seconds =
                (self.runtime.tutorial.next_hint_delay_seconds - frame_time).max(0.0);
        }
        if self.runtime.tutorial.next_hint_delay_seconds > 0.0
            || !self.runtime.gather_toasts.is_empty()
            || self.overlay().is_some()
        {
            return;
        }

        let near_alchemy = self
            .nearby_station(data)
            .map(|station| station.kind == StationKind::Alchemy)
            .unwrap_or(false);
        let near_quest_npc = self
            .nearby_npc(data)
            .and_then(|npc| self.npc_world_label(data, npc))
            .is_some();
        let nearby_available_node = data
            .area(&self.world.current_area_id)
            .map(|area| {
                area.gather_nodes.iter().any(|node| {
                    !self.world.gathered_nodes.contains(&node.id)
                        && self.node_is_available(node)
                        && self
                            .world
                            .player
                            .position
                            .distance(vec2(node.position[0], node.position[1]))
                            <= node.radius + data.config.interaction_range + 36.0
                })
            })
            .unwrap_or(false);
        let unlockable_warp_here = data
            .area(&self.world.current_area_id)
            .map(|area| {
                area.warps
                    .iter()
                    .any(|warp| !self.warp_is_unlocked(warp) && self.can_unlock_warp(warp))
            })
            .unwrap_or(false);
        let has_quick_potions = !self.quick_potions(data).is_empty();
        let next_hint = if !self.runtime.tutorial.crow_intro_hint_shown {
            self.runtime.tutorial.crow_intro_hint_shown = true;
            Some((
                ui_copy("tutorial_crow_intro").to_owned(),
                Color::from_rgba(176, 226, 255, 255),
            ))
        } else if !self.runtime.tutorial.save_hint_shown {
            self.runtime.tutorial.save_hint_shown = true;
            Some((
                ui_copy("tutorial_save").to_owned(),
                Color::from_rgba(176, 226, 255, 255),
            ))
        } else if !self.runtime.tutorial.journal_hint_shown {
            self.runtime.tutorial.journal_hint_shown = true;
            Some((
                ui_copy("tutorial_journal").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.alchemy_hint_shown && near_alchemy {
            self.runtime.tutorial.alchemy_hint_shown = true;
            Some((
                ui_copy("tutorial_alchemy_open").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.brew_goal_hint_shown
            && self.progression.total_brews == 0
            && near_alchemy
        {
            self.runtime.tutorial.brew_goal_hint_shown = true;
            Some((
                ui_copy("tutorial_brew_goal").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.potion_hint_shown && has_quick_potions {
            self.runtime.tutorial.potion_hint_shown = true;
            Some((
                ui_copy("tutorial_potions").to_owned(),
                Color::from_rgba(255, 214, 132, 255),
            ))
        } else if !self.runtime.tutorial.gather_hint_shown
            && self.progression.herb_memories.is_empty()
            && nearby_available_node
        {
            self.runtime.tutorial.gather_hint_shown = true;
            Some((
                ui_copy("tutorial_gather").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.mira_hint_shown
            && self.progression.total_brews > 0
            && !self
                .progression
                .completed_quests
                .contains("healing_for_mira")
        {
            self.runtime.tutorial.mira_hint_shown = true;
            Some((
                ui_copy("tutorial_mira_delivery").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.rowan_hint_shown
            && self
                .progression
                .completed_quests
                .contains("healing_for_mira")
            && !self.progression.completed_quests.contains("glow_for_rowan")
        {
            self.runtime.tutorial.rowan_hint_shown = true;
            Some((
                ui_copy("tutorial_rowan_goal").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.quest_hint_shown
            && self.progression.started_quests.is_empty()
            && self.progression.completed_quests.is_empty()
            && (near_quest_npc || !self.available_board_quests(data).is_empty())
        {
            self.runtime.tutorial.quest_hint_shown = true;
            Some((
                ui_copy("tutorial_quest").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.delivery_hint_shown
            && self
                .progression
                .started_quests
                .iter()
                .filter_map(|quest_id| data.quest(quest_id))
                .any(|quest| self.quest_requirements_met(data, quest))
        {
            self.runtime.tutorial.delivery_hint_shown = true;
            Some((
                ui_copy("tutorial_delivery_ready").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.route_hint_shown && unlockable_warp_here {
            self.runtime.tutorial.route_hint_shown = true;
            Some((
                ui_copy("tutorial_route_ready").to_owned(),
                Color::from_rgba(176, 226, 255, 255),
            ))
        } else {
            None
        };

        if let Some((text, color)) = next_hint {
            self.push_event_toast(text, color);
            self.runtime.tutorial.next_hint_delay_seconds = 6.0;
        }
    }
}
