use super::GameplayState;
use crate::data::GameData;

#[path = "gameplay_time_status_text.rs"]
mod time_status_text;

impl GameplayState {
    pub(super) fn current_time_window(&self) -> &'static str {
        let total_minutes = self.current_clock_minutes() as i32;
        match total_minutes {
            360..=659 => "morning",
            660..=1019 => "day",
            1020..=1259 => "evening",
            _ => "night",
        }
    }

    pub(super) fn current_clock_minutes(&self) -> f32 {
        (self.world.day_clock_seconds / self.world.day_length_seconds) * 24.0 * 60.0
    }

    pub(super) fn set_clock_minutes(&mut self, minutes: f32) {
        self.world.day_clock_seconds = (minutes / (24.0 * 60.0)) * self.world.day_length_seconds;
    }

    pub(super) fn advance_to_next_day(&mut self, data: &GameData, with_feedback: bool) {
        self.world.day_index += 1;
        self.world.gathered_nodes.clear();
        self.advance_planters(data);
        self.refresh_available_nodes(data);
        if with_feedback {
            self.runtime.status_text =
                time_status_text::day_begin(self.current_weather(), self.current_season());
            self.trigger_day_begin_feedback();
        }
    }

    pub(super) fn sleep_until(&mut self, data: &GameData, wake_minutes: f32, forced_home: bool) {
        let advanced_day = self.current_clock_minutes() >= wake_minutes;
        if advanced_day {
            self.advance_to_next_day(data, false);
        }
        self.set_clock_minutes(wake_minutes);
        if forced_home {
            if let Some(bed) = data
                .stations
                .iter()
                .find(|station| station.id == "entry_rest_bed")
            {
                self.world.current_area_id = bed.area_id.clone();
                self.set_player_position([bed.position[0], bed.position[1] + 52.0]);
                self.set_player_facing([0.0, -1.0]);
                self.stop_player_motion();
            }
            self.runtime.sleep_flash_seconds = 1.2;
            self.runtime.status_text = time_status_text::fainted_home();
        } else {
            self.runtime.status_text = time_status_text::slept_until("07:00");
        }
    }

    pub(super) fn handle_sleep_pressure(&mut self, data: &GameData) {
        let minutes = self.current_clock_minutes();
        if (60.0..120.0).contains(&minutes) {
            self.sleep_until(data, 10.0 * 60.0, true);
        }
    }

}
