use super::gameplay_npc_pathing::schedule_start_minutes;
use super::{GameplayState, NpcRuntimeState};
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, NpcDefinition, QuestDefinition};
use macroquad::prelude::{vec2, Vec2};

pub(super) use super::gameplay_npc_pathing::npc_motion_seed;

impl GameplayState {
    pub(super) fn initialize_npc_motion_states(&mut self, data: &GameData) {
        self.runtime.npc_motion_states.clear();
        let walk_speed = (data.config.move_speed * 0.16).max(24.0);
        for npc in &data.npcs {
            let tracker = self.initial_npc_motion_tracker(data, npc, walk_speed);
            self.runtime
                .npc_motion_states
                .insert(npc.id.clone(), tracker);
        }
    }

    pub(super) fn update_npc_motion(&mut self, data: &GameData, frame_time: f32) {
        let walk_speed = (data.config.move_speed * 0.16).max(24.0);
        for npc in &data.npcs {
            let Some(current_index) = self.active_schedule_index(npc) else {
                continue;
            };
            let next_index = (current_index + 1) % npc.schedule.len();
            let mut tracker = self
                .runtime
                .npc_motion_states
                .remove(&npc.id)
                .unwrap_or_else(|| self.initial_npc_motion_tracker(data, npc, walk_speed));

            let (elapsed_seconds, interval_seconds) =
                self.schedule_window_progress_seconds(data, npc, current_index);
            let current_entry = &npc.schedule[current_index];
            let next_entry = &npc.schedule[next_index];
            let current_position = vec2(current_entry.position[0], current_entry.position[1]);
            let next_position = vec2(next_entry.position[0], next_entry.position[1]);

            if tracker.target_schedule_index.is_none() {
                let anchored_to_current = tracker.schedule_index == Some(current_index);
                let already_at_next = tracker.schedule_index == Some(next_index);
                if !anchored_to_current && !already_at_next {
                    let recovery_route = self
                        .travel_segments(
                            data,
                            &tracker.area_id,
                            tracker.position,
                            &current_entry.area_id,
                            current_position,
                        )
                        .unwrap_or_default();
                    if recovery_route.is_empty() {
                        tracker.area_id = current_entry.area_id.clone();
                        tracker.position = current_position;
                        tracker.direction = Vec2::ZERO;
                        tracker.moving = false;
                        tracker.target_area_id = None;
                        tracker.schedule_index = Some(current_index);
                    } else {
                        tracker.target_area_id = Some(current_entry.area_id.clone());
                        tracker.target_schedule_index = Some(current_index);
                        tracker.route_segments = recovery_route;
                        tracker.route_segment_index = 0;
                        tracker.moving = true;
                    }
                } else {
                    let route_from_anchor = self
                        .travel_segments(
                            data,
                            &current_entry.area_id,
                            current_position,
                            &next_entry.area_id,
                            next_position,
                        )
                        .unwrap_or_default();
                    let travel_duration =
                        self.npc_travel_duration_seconds(&route_from_anchor, walk_speed);
                    let departure_seconds = (interval_seconds - travel_duration).max(0.0);

                    if anchored_to_current && elapsed_seconds >= departure_seconds {
                        let route = self
                            .travel_segments(
                                data,
                                &tracker.area_id,
                                tracker.position,
                                &next_entry.area_id,
                                next_position,
                            )
                            .unwrap_or_default();
                        if route.is_empty() {
                            tracker.area_id = next_entry.area_id.clone();
                            tracker.position = next_position;
                            tracker.direction = Vec2::ZERO;
                            tracker.moving = false;
                            tracker.target_area_id = None;
                            tracker.schedule_index = Some(next_index);
                        } else {
                            tracker.target_area_id = Some(next_entry.area_id.clone());
                            tracker.target_schedule_index = Some(next_index);
                            tracker.route_segments = route;
                            tracker.route_segment_index = 0;
                            tracker.moving = true;
                        }
                    }
                }
            }

            self.advance_npc_tracker(&mut tracker, walk_speed, frame_time);
            self.runtime
                .npc_motion_states
                .insert(npc.id.clone(), tracker);
        }
    }

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

    pub(super) fn npc_schedule_area_for_time<'a>(
        &self,
        npc: &'a NpcDefinition,
        time_window: &str,
    ) -> Option<&'a str> {
        npc.schedule
            .iter()
            .find(|entry| entry.time_window == time_window)
            .map(|entry| entry.area_id.as_str())
    }

    pub(super) fn npc_now_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let runtime = self.npc_runtime_state(data, npc);
        let area_name = data
            .area(&runtime.area_id)
            .map(|area| area.name.as_str())
            .unwrap_or(ui_copy("npc_hint_somewhere"));
        if runtime.moving {
            let target_name = runtime
                .target_area_id
                .as_ref()
                .and_then(|area_id| data.area(area_id))
                .map(|area| area.name.as_str())
                .unwrap_or(area_name);
            ui_format(
                "npc_travelling",
                &[("area", area_name), ("target", target_name)],
            )
        } else {
            ui_format(
                "npc_hint_here_now",
                &[("area", area_name), ("time", self.current_time_window())],
            )
        }
    }

    pub(super) fn npc_later_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(current_index) = self.active_schedule_index(npc) else {
            return ui_copy("npc_hint_routine_unclear").to_owned();
        };
        let later_windows = ["day", "evening", "night", "morning"];
        let current_window = self.current_time_window();
        let next_window = later_windows
            .iter()
            .skip_while(|window| **window != current_window)
            .nth(1)
            .copied()
            .unwrap_or("morning");
        let later_area = self
            .npc_schedule_area_for_time(npc, next_window)
            .and_then(|area_id| data.area(area_id))
            .map(|area| area.name.as_str())
            .unwrap_or_else(|| {
                let next_index = (current_index + 1) % npc.schedule.len();
                data.area(&npc.schedule[next_index].area_id)
                    .map(|area| area.name.as_str())
                    .unwrap_or(ui_copy("npc_hint_unknown"))
            });
        ui_format(
            "npc_hint_later",
            &[("area", later_area), ("time", next_window)],
        )
    }

    pub(super) fn npc_usual_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        ["morning", "day", "evening"]
            .iter()
            .filter_map(|time_window| {
                self.npc_schedule_area_for_time(npc, time_window)
                    .and_then(|area_id| data.area(area_id))
                    .map(|area| {
                        ui_format(
                            "npc_hint_usual",
                            &[("time", time_window), ("area", &area.name)],
                        )
                    })
            })
            .collect::<Vec<_>>()
            .join("  |  ")
    }

    pub(super) fn npc_runtime_state(
        &self,
        _data: &GameData,
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

        let tracker = self.initial_npc_motion_tracker(_data, npc, 24.0);
        NpcRuntimeState {
            area_id: tracker.area_id,
            position: tracker.position,
            direction: tracker.direction,
            moving: tracker.moving,
            target_area_id: tracker.target_area_id,
        }
    }

    pub(super) fn quest_location_hint(&self, data: &GameData, quest: &QuestDefinition) -> String {
        data.npc(&quest.giver_npc_id)
            .map(|npc| {
                ui_format(
                    "npc_quest_location",
                    &[
                        ("now", &self.npc_now_hint(data, npc)),
                        ("later", &self.npc_later_hint(data, npc)),
                    ],
                )
            })
            .unwrap_or_else(|| ui_copy("npc_quest_location_fallback").to_owned())
    }

    pub(super) fn npc_context_line(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let relationship = self
            .progression
            .relationships
            .get(&npc.id)
            .copied()
            .unwrap_or_default();
        let base = ui_format(
            "npc_context_line",
            &[
                ("now", &self.npc_now_hint(data, npc)),
                ("later", &self.npc_later_hint(data, npc)),
                ("usual", &self.npc_usual_hint(data, npc)),
                ("rapport", &relationship.to_string()),
                (
                    "role",
                    if npc.role.is_empty() {
                        ui_copy("overlay_rapport_empty")
                    } else {
                        npc.role.as_str()
                    },
                ),
            ],
        );
        self.append_npc_story_line(&npc.id, base)
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
