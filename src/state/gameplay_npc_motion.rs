use super::gameplay_npc_pathing::schedule_start_minutes;
use super::{GameplayState, NpcMotionTracker, TravelSegment};
use crate::data::{GameData, NpcDefinition};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn initial_npc_motion_tracker(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
        walk_speed: f32,
    ) -> NpcMotionTracker {
        let schedule_index = self.active_schedule_index(npc);
        let Some(current_index) = schedule_index else {
            return NpcMotionTracker {
                area_id: npc.area_id.clone(),
                position: vec2(npc.position[0], npc.position[1]),
                direction: Vec2::ZERO,
                moving: false,
                target_area_id: None,
                schedule_index: None,
                target_schedule_index: None,
                route_segments: Vec::new(),
                route_segment_index: 0,
            };
        };

        let next_index = (current_index + 1) % npc.schedule.len();
        let current_entry = &npc.schedule[current_index];
        let next_entry = &npc.schedule[next_index];
        let current_position = vec2(current_entry.position[0], current_entry.position[1]);
        let next_position = vec2(next_entry.position[0], next_entry.position[1]);
        let route = self
            .travel_segments(
                data,
                &current_entry.area_id,
                current_position,
                &next_entry.area_id,
                next_position,
            )
            .unwrap_or_default();
        let (elapsed_seconds, interval_seconds) =
            self.schedule_window_progress_seconds(data, npc, current_index);
        let travel_duration = self.npc_travel_duration_seconds(&route, walk_speed);
        let departure_seconds = (interval_seconds - travel_duration).max(0.0);

        let mut tracker = NpcMotionTracker {
            area_id: current_entry.area_id.clone(),
            position: current_position,
            direction: Vec2::ZERO,
            moving: false,
            target_area_id: None,
            schedule_index: Some(current_index),
            target_schedule_index: None,
            route_segments: Vec::new(),
            route_segment_index: 0,
        };

        if !route.is_empty() && elapsed_seconds >= departure_seconds {
            tracker.target_area_id = Some(next_entry.area_id.clone());
            tracker.target_schedule_index = Some(next_index);
            tracker.route_segments = route;
            tracker.route_segment_index = 0;
            tracker.moving = true;
            let pre_advance = elapsed_seconds - departure_seconds;
            self.advance_npc_tracker(&mut tracker, walk_speed, pre_advance);
        }

        tracker
    }

    pub(super) fn schedule_window_progress_seconds(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
        current_index: usize,
    ) -> (f32, f32) {
        let current_entry = &npc.schedule[current_index];
        let next_index = (current_index + 1) % npc.schedule.len();
        let next_entry = &npc.schedule[next_index];
        let current_start = schedule_start_minutes(&current_entry.time_window);
        let mut next_start = schedule_start_minutes(&next_entry.time_window);
        if next_start <= current_start {
            next_start += 24.0 * 60.0;
        }
        let mut current_minutes = self.current_clock_minutes();
        if current_minutes < current_start {
            current_minutes += 24.0 * 60.0;
        }
        let interval_minutes = next_start - current_start;
        let elapsed_minutes = (current_minutes - current_start).clamp(0.0, interval_minutes);
        let seconds_per_minute = data.config.day_length_seconds / (24.0 * 60.0);
        (
            elapsed_minutes * seconds_per_minute,
            interval_minutes * seconds_per_minute,
        )
    }

    pub(super) fn npc_travel_duration_seconds(&self, route: &[TravelSegment], walk_speed: f32) -> f32 {
        if route.is_empty() {
            0.0
        } else {
            route
                .iter()
                .map(|segment| segment.start.distance(segment.end))
                .sum::<f32>()
                / walk_speed.max(1.0)
        }
    }

    pub(super) fn advance_npc_tracker(
        &self,
        tracker: &mut NpcMotionTracker,
        walk_speed: f32,
        frame_time: f32,
    ) {
        let mut remaining = walk_speed * frame_time;
        while remaining > 0.0 && tracker.route_segment_index < tracker.route_segments.len() {
            let segment = &tracker.route_segments[tracker.route_segment_index];
            let to_end = segment.end - tracker.position;
            let distance = to_end.length();

            if distance <= 0.5 {
                tracker.position = segment.end;
                tracker.area_id = segment.area_id.clone();
                tracker.route_segment_index += 1;
                if let Some(next_segment) = tracker.route_segments.get(tracker.route_segment_index)
                {
                    if next_segment.area_id != segment.area_id {
                        tracker.area_id = next_segment.area_id.clone();
                        tracker.position = next_segment.start;
                        tracker.direction = Vec2::ZERO;
                        tracker.moving = true;
                        break;
                    }
                }
                continue;
            }

            let step = remaining.min(distance);
            let direction = to_end / distance;
            tracker.position += direction * step;
            tracker.direction = direction;
            tracker.area_id = segment.area_id.clone();
            tracker.moving = true;
            remaining -= step;

            if step >= distance - 0.01 {
                tracker.position = segment.end;
                tracker.route_segment_index += 1;
                if let Some(next_segment) = tracker.route_segments.get(tracker.route_segment_index)
                {
                    if next_segment.area_id != segment.area_id {
                        tracker.area_id = next_segment.area_id.clone();
                        tracker.position = next_segment.start;
                        tracker.direction = Vec2::ZERO;
                        tracker.moving = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }

        if tracker.route_segment_index >= tracker.route_segments.len() {
            tracker.route_segments.clear();
            tracker.route_segment_index = 0;
            tracker.moving = false;
            tracker.direction = Vec2::ZERO;
            tracker.schedule_index = tracker.target_schedule_index.or(tracker.schedule_index);
            tracker.target_schedule_index = None;
            tracker.target_area_id = None;
        }
    }
}
