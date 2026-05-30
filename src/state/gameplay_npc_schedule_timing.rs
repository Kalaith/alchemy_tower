use super::gameplay_npc_pathing::schedule_start_minutes;
use super::gameplay_npc_types::TravelSegment;
use super::GameplayState;
use crate::data::{GameData, NpcDefinition};

impl GameplayState {
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

    pub(super) fn npc_travel_duration_seconds(
        &self,
        route: &[TravelSegment],
        walk_speed: f32,
    ) -> f32 {
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
}
