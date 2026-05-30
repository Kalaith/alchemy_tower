use super::GameplayState;
use crate::data::GameData;
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
}
