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

    /// Move the live clock and refresh time-gated ground at the instant its
    /// window changes. Area entry and midnight are not substitutes: a player
    /// waiting beside a night flower should see it arrive without zoning out.
    pub(super) fn advance_clock(&mut self, data: &GameData, seconds: f32) {
        let previous_window = self.current_time_window();
        let mut advanced_day = false;
        self.world.day_clock_seconds += seconds;
        while self.world.day_clock_seconds >= data.config.day_length_seconds {
            self.world.day_clock_seconds -= data.config.day_length_seconds;
            self.advance_to_next_day(data, true);
            advanced_day = true;
        }
        if !advanced_day && self.current_time_window() != previous_window {
            self.refresh_available_nodes(data);
        }
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
        // Day rollover refreshed at the hour the player went to bed. The final
        // wake hour is the condition the ground must actually answer to.
        self.refresh_available_nodes(data);
        let tuning = &data.config.balance.vitality;
        let recovered = if forced_home {
            tuning.collapse_restores
        } else {
            tuning.sleep_restores
        };
        self.restore_vitality_to(recovered);
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
            self.queue_sound(super::gameplay_runtime_types::GameSound::CollapseHome);
            self.runtime.status_text = time_status_text::fainted_home();
        } else {
            self.runtime.status_text = time_status_text::slept_until("07:00");
        }
    }

    /// The two ways a working day ends without the player choosing it: the
    /// small hours, and running out of anything left to give.
    pub(super) fn handle_sleep_pressure(&mut self, data: &GameData) {
        let minutes = self.current_clock_minutes();
        if (60.0..120.0).contains(&minutes) || self.is_exhausted() {
            self.sleep_until(data, 10.0 * 60.0, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    #[test]
    fn waiting_across_a_window_refreshes_the_ground_without_changing_area() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let node_id = "archive_inkgall_01";
        state.world.current_area_id = "archive_floor".to_owned();
        state.set_clock_minutes(1261.0);

        let day = (0..40)
            .find(|day| {
                state.world.day_index = *day;
                state.refresh_available_nodes(&data);
                state.world.available_nodes.contains(node_id)
            })
            .expect("the night ink should have a valid day in the campaign cycle");
        state.world.day_index = day;
        let two_minutes = state.world.day_length_seconds * 2.0 / (24.0 * 60.0);

        state.set_clock_minutes(1259.0);
        state.refresh_available_nodes(&data);
        assert_eq!(state.current_time_window(), "evening");
        assert!(!state.world.available_nodes.contains(node_id));
        state.advance_clock(&data, two_minutes);
        assert_eq!(state.current_time_window(), "night");
        assert!(
            state.world.available_nodes.contains(node_id),
            "night ground did not arrive when night did"
        );

        state.set_clock_minutes(359.0);
        state.refresh_available_nodes(&data);
        assert!(state.world.available_nodes.contains(node_id));
        state.advance_clock(&data, two_minutes);
        assert_eq!(state.current_time_window(), "morning");
        assert!(
            !state.world.available_nodes.contains(node_id),
            "night ground remained after morning arrived"
        );
    }
}
