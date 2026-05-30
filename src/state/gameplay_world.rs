use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, JournalMilestoneEntry};
use macroquad::prelude::Color;

impl GameplayState {
    pub(super) fn current_season(&self) -> &'static str {
        match (self.world.day_index / 5) % 4 {
            0 => "spring",
            1 => "summer",
            2 => "autumn",
            _ => "winter",
        }
    }

    pub(super) fn current_weather(&self) -> &'static str {
        match self.world.day_index % 4 {
            0 => "clear",
            1 => "mist",
            2 => "rain",
            _ => "windy",
        }
    }

    pub(super) fn node_daily_roll(&self, node_id: &str) -> u32 {
        let mut value = self.world.day_index.wrapping_mul(31);
        for byte in node_id.as_bytes() {
            value = value.wrapping_mul(33).wrapping_add(*byte as u32);
        }
        (value % 100) + 1
    }

    pub(super) fn refresh_available_nodes(&mut self, data: &GameData) {
        self.world.available_nodes.clear();
        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };

        for node in &area.gather_nodes {
            let season_ok = node.seasons.is_empty()
                || node
                    .seasons
                    .iter()
                    .any(|season| season == self.current_season());
            let weather_ok = node.weathers.is_empty()
                || node
                    .weathers
                    .iter()
                    .any(|weather| weather == self.current_weather());
            let time_ok = node.time_windows.is_empty()
                || node
                    .time_windows
                    .iter()
                    .any(|time| time == self.current_time_window());
            let daily_roll = self.node_daily_roll(&node.id);

            if season_ok && weather_ok && time_ok && daily_roll <= node.spawn_chance {
                self.world.available_nodes.insert(node.id.clone());
            }
        }
    }

    pub(super) fn advance_planters(&mut self, data: &GameData) {
        for state in self.progression.planter_states.values_mut() {
            if state.planted_item_id.is_empty() || state.ready {
                continue;
            }
            let days = data
                .stations
                .iter()
                .find(|station| station.id == state.station_id)
                .map(|station| {
                    station
                        .planter_harvest_days
                        .max(1)
                        .saturating_sub(state.mutation_growth_bonus_days)
                        .max(1)
                })
                .unwrap_or(2);
            state.growth_days = self.world.day_index.saturating_sub(state.planted_day);
            if state.growth_days >= days {
                state.ready = true;
            }
        }
    }

    pub(super) fn push_journal_milestone(&mut self, id: &str, title: &str, text: &str) {
        if self
            .progression
            .journal_milestones
            .iter()
            .any(|entry| entry.id == id)
        {
            return;
        }
        self.progression
            .journal_milestones
            .push(JournalMilestoneEntry {
                id: id.to_owned(),
                title: title.to_owned(),
                text: text.to_owned(),
            });
        self.push_event_toast_with_icon(
            ui_format("gameplay_new_journal_note", &[("title", title)]),
            Color::from_rgba(176, 226, 255, 255),
            "journal_note",
        );
        self.trigger_world_feedback(
            self.world.player.position,
            Color::from_rgba(176, 226, 255, 255),
            true,
            1.6,
        );
    }

    pub(super) fn has_journal_milestone(&self, id: &str) -> bool {
        self.progression
            .journal_milestones
            .iter()
            .any(|entry| entry.id == id)
    }

}
