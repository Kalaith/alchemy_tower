use super::*;
use crate::content::{ui_copy, ui_format};
use crate::ui::draw_interaction_prompt;

struct WorldPromptView {
    position: Vec2,
    text: String,
}

impl GameplayState {
    pub(super) fn draw_prompt(&self, area: &AreaDefinition, offset: Vec2, data: &GameData) {
        if let Some(prompt) = self.build_world_prompt(area, offset, data) {
            draw_world_prompt(&prompt);
        }
    }

    fn build_world_prompt(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
    ) -> Option<WorldPromptView> {
        if let Some(npc) = self.nearby_npc(data) {
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
            return Some(WorldPromptView {
                position: vec2(offset.x + npc_pos.x, offset.y + npc_pos.y - 42.0),
                text,
            });
        }

        if let Some(station) = self.nearby_station(data) {
            let position = vec2(
                offset.x + station.position[0],
                offset.y + station.position[1] - 42.0,
            );
            let text = if station.kind == StationKind::Alchemy {
                format!("E/{}: {}", input_bindings().alchemy.open, ui_text().prompts.open_alchemy)
            } else if station.kind == StationKind::RestBed {
                format!(
                    "{}: {}",
                    input_bindings().global.interact,
                    ui_text().prompts.sleep_in_bed
                )
            } else if station.kind == StationKind::Shop {
                format!(
                    "{}: {}",
                    input_bindings().global.interact,
                    ui_text().prompts.browse_shop
                )
            } else if station.kind == StationKind::RuneWorkshop {
                format!(
                    "{}: {}",
                    input_bindings().global.interact,
                    ui_text().prompts.open_rune_workshop
                )
            } else if station.kind == StationKind::ArchiveConsole {
                format!(
                    "{}: {}",
                    input_bindings().global.interact,
                    ui_text().prompts.reconstruct_archives
                )
            } else if station.kind == StationKind::EndingFocus {
                format!(
                    "{}: {}",
                    input_bindings().global.interact,
                    ui_text().prompts.focus_observatory
                )
            } else if station.kind == StationKind::QuestBoard {
                if self.available_board_quests(data).is_empty() {
                    format!(
                        "{}: {}",
                        input_bindings().global.interact,
                        ui_text().prompts.read_request_board
                    )
                } else {
                    ui_copy("world_prompt_board_new").to_owned()
                }
            } else if station.kind == StationKind::Planter {
                self.progression
                    .planter_states
                    .get(&station.id)
                    .map(|state| {
                        if state.ready {
                            ui_copy("world_prompt_planter_harvest").to_owned()
                        } else if state.planted_item_id.is_empty() {
                            ui_copy("world_prompt_planter_plant").to_owned()
                        } else if state.tended_day != self.world.day_index {
                            ui_copy("world_prompt_planter_tend").to_owned()
                        } else {
                            ui_copy("world_prompt_planter_check").to_owned()
                        }
                    })
                    .unwrap_or_else(|| ui_copy("world_prompt_planter_plant").to_owned())
            } else if station.kind == StationKind::Habitat {
                self.progression
                    .habitat_states
                    .get(&station.id)
                    .map(|state| {
                        if state.creature_item_id.is_empty() {
                            ui_copy("world_prompt_habitat_place").to_owned()
                        } else if self.world.day_index
                            >= state
                                .last_harvest_day
                                .saturating_add(station.habitat_harvest_days.max(1))
                        {
                            ui_copy("world_prompt_habitat_harvest").to_owned()
                        } else {
                            ui_copy("world_prompt_habitat_check").to_owned()
                        }
                    })
                    .unwrap_or_else(|| ui_copy("world_prompt_habitat_place").to_owned())
            } else {
                String::new()
            };

            if !text.is_empty() {
                return Some(WorldPromptView { position, text });
            }
        }

        if let Some(warp) = area
            .warps
            .iter()
            .find(|warp| warp.rect.contains_point(self.world.player.position))
        {
            return Some(WorldPromptView {
                position: vec2(
                    offset.x + self.world.player.position.x,
                    offset.y + self.world.player.position.y - 36.0,
                ),
                text: if self.warp_is_unlocked(warp) {
                    ui_format("world_prompt_enter", &[("label", &warp.label)])
                } else if self.can_unlock_warp(warp) {
                    ui_format("world_prompt_restore", &[("label", &warp.label)])
                } else {
                    self.warp_lock_text(data, warp)
                },
            });
        }

        area.gather_nodes
            .iter()
            .find(|node| {
                !self.world.gathered_nodes.contains(&node.id)
                    && self.node_is_available(node)
                    && self
                        .world
                        .player
                        .position
                        .distance(vec2(node.position[0], node.position[1]))
                        <= node.radius + data.config.interaction_range + 28.0
            })
            .map(|node| WorldPromptView {
                position: vec2(
                    offset.x + node.position[0],
                    offset.y + node.position[1] - node.radius - 18.0,
                ),
                text: ui_format("world_prompt_gather", &[("name", &node.name)]),
            })
    }
}

fn draw_world_prompt(prompt: &WorldPromptView) {
    draw_interaction_prompt(prompt.position, &prompt.text);
}
