use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use macroquad::prelude::{vec2, Color};

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
            self.runtime.status_text = ui_format(
                "day_begin_status",
                &[
                    ("weather", self.current_weather()),
                    ("season", self.current_season()),
                ],
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(176, 226, 255, 255),
                true,
                1.5,
            );
            self.trigger_camera_shake(0.22, 6.0);
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
                self.world.player.position = vec2(bed.position[0], bed.position[1] + 52.0);
                self.world.player.facing = vec2(0.0, -1.0);
                self.world.player.moving = false;
            }
            self.runtime.sleep_flash_seconds = 1.2;
            self.runtime.status_text = ui_copy("gameplay_fainted_home").to_owned();
        } else {
            self.runtime.status_text = ui_format("gameplay_slept_until", &[("time", "07:00")]);
        }
    }

    pub(super) fn handle_sleep_pressure(&mut self, data: &GameData) {
        let minutes = self.current_clock_minutes();
        if (60.0..120.0).contains(&minutes) {
            self.sleep_until(data, 10.0 * 60.0, true);
        }
    }

}
