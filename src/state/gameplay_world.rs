use super::gameplay_planter_actions::{planter_growth_days, planter_growth_target};
use super::GameplayState;
use crate::data::{GameData, JournalMilestoneEntry};

#[path = "gameplay_world_text.rs"]
mod world_text;

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
            // Ground that a finished story beat brought back to life stays bare
            // until that beat is actually finished.
            if !node.required_completed_quest.is_empty()
                && !self
                    .progression
                    .completed_quests
                    .contains(&node.required_completed_quest)
            {
                continue;
            }
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
                .map(|station| planter_growth_target(station, state))
                .unwrap_or(2);
            state.growth_days = planter_growth_days(state, self.world.day_index);
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
        self.trigger_journal_note_feedback(world_text::new_journal_note(title));
    }

    pub(super) fn has_journal_milestone(&self, id: &str) -> bool {
        self.progression
            .journal_milestones
            .iter()
            .any(|entry| entry.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    /// Season, weather and hour are three independent gates and a daily roll is
    /// a fourth. It is easy to author a node whose conditions can never all be
    /// true at once, and such a node is invisible content: it costs art, data
    /// and a route entry, and never appears. Walk a full cycle of every gate and
    /// insist each node turns up at least once.
    #[test]
    fn every_gather_node_can_actually_spawn() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        for quest in &data.quests {
            state.progression.completed_quests.insert(quest.id.clone());
        }

        // Season advances every 5 days and weather every 4, so a 20-day sweep
        // covers every pairing; the extra days vary the per-node daily roll.
        let day_length = state.world.day_length_seconds;
        let never_spawns = data
            .areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter().map(move |node| (area, node)))
            .filter(|(area, node)| {
                state.world.current_area_id = area.id.clone();
                for day in 0..60u32 {
                    for fraction in [0.3, 0.5, 0.75, 0.95] {
                        state.world.day_index = day;
                        state.world.day_clock_seconds = day_length * fraction;
                        state.refresh_available_nodes(&data);
                        if state.world.available_nodes.contains(&node.id) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|(area, node)| format!("{} in {}", node.id, area.id))
            .collect::<Vec<_>>();

        assert!(
            never_spawns.is_empty(),
            "gather nodes that can never appear:\n{never_spawns:#?}"
        );
    }

    /// `every_gather_node_can_actually_spawn` only asks whether a node appears
    /// at all. That is a low bar: a node can clear it and still make the player
    /// wait weeks. This measures the two things they actually feel — how long
    /// until it first turns up, and how often it turns up after that.
    ///
    /// The per-node daily roll is `day * 31` mixed with the node id, which is a
    /// linear sequence rather than noise, and season and weather cycle every 20
    /// days. Those periods interact: the lake's stillwater pearl qualifies on
    /// five days of the first cycle and its roll is too high on every one of
    /// them, so the only source of a catalyst three morphs need is absent until
    /// day 21. That is the behaviour this pins — the floors are precedent, set
    /// just outside the worst node that already ships, not an ambition.
    #[test]
    fn every_gather_node_turns_up_soon_enough_and_often_enough() {
        const SWEEP_DAYS: u32 = 100;
        const LATEST_FIRST_APPEARANCE: u32 = 28;
        const MINIMUM_DAYS_IN_SWEEP: usize = 6;

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        for quest in &data.quests {
            state.progression.completed_quests.insert(quest.id.clone());
        }

        let day_length = state.world.day_length_seconds;
        let mut complaints = Vec::new();
        for area in &data.areas {
            for node in &area.gather_nodes {
                state.world.current_area_id = area.id.clone();
                let mut first = None;
                let mut seen = 0usize;
                for day in 0..SWEEP_DAYS {
                    // Any hour of that day counts: the player can wait for dusk.
                    let available = [0.15, 0.35, 0.6, 0.85, 0.97].iter().any(|fraction| {
                        state.world.day_index = day;
                        state.world.day_clock_seconds = day_length * fraction;
                        state.refresh_available_nodes(&data);
                        state.world.available_nodes.contains(&node.id)
                    });
                    if available {
                        seen += 1;
                        first.get_or_insert(day);
                    }
                }

                match first {
                    None => complaints.push(format!("{} in {}: never", node.id, area.id)),
                    Some(day) if day > LATEST_FIRST_APPEARANCE => complaints.push(format!(
                        "{} in {}: first appears on day {day}",
                        node.id, area.id
                    )),
                    _ => {}
                }
                if seen < MINIMUM_DAYS_IN_SWEEP {
                    complaints.push(format!(
                        "{} in {}: only {seen} of {SWEEP_DAYS} days",
                        node.id, area.id
                    ));
                }
            }
        }
        complaints.sort();

        assert!(
            complaints.is_empty(),
            "gather nodes the player cannot reasonably find:
{complaints:#?}"
        );
    }

    /// Seasons are deliberately unequal — winter should be leaner than spring,
    /// and the charred hollow exists to give that leanness a destination rather
    /// than erase it. This is a floor, not a balance target: it catches a pass
    /// that starves a quarter of the year without noticing, which is exactly how
    /// winter got down to 62% of spring before anyone counted.
    #[test]
    fn no_season_is_starved_of_gatherable_ground() {
        const SEASONS: [&str; 4] = ["spring", "summer", "autumn", "winter"];
        const FLOOR: f32 = 0.5;

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let counts = SEASONS.map(|season| {
            data.areas
                .iter()
                .flat_map(|area| area.gather_nodes.iter())
                .filter(|node| node.seasons.is_empty() || node.seasons.iter().any(|s| s == season))
                .count()
        });

        let best = *counts.iter().max().expect("four seasons");
        let worst = *counts.iter().min().expect("four seasons");
        let ratio = worst as f32 / best as f32;

        assert!(
            ratio >= FLOOR,
            "the leanest season has only {worst} nodes against {best} in the richest \
             ({ratio:.2} of it); per season: {:?}",
            SEASONS.iter().zip(counts.iter()).collect::<Vec<_>>()
        );
    }

    /// The same floor for weather. Wind sat at 23 nodes against clear's 40
    /// before the plains were given a reason to be walked in it — a whole
    /// condition a player could ignore for a whole game. Leanness is fine; an
    /// axis nothing uses is not.
    #[test]
    fn no_weather_is_starved_of_gatherable_ground() {
        const WEATHERS: [&str; 4] = ["clear", "mist", "rain", "windy"];
        const FLOOR: f32 = 0.5;

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let counts = WEATHERS.map(|weather| {
            data.areas
                .iter()
                .flat_map(|area| area.gather_nodes.iter())
                .filter(|node| {
                    node.weathers.is_empty() || node.weathers.iter().any(|w| w == weather)
                })
                .count()
        });

        let best = *counts.iter().max().expect("four weathers");
        let worst = *counts.iter().min().expect("four weathers");
        let ratio = worst as f32 / best as f32;

        assert!(
            ratio >= FLOOR,
            "the leanest weather has only {worst} nodes against {best} in the richest \
             ({ratio:.2} of it); per weather: {:?}",
            WEATHERS.iter().zip(counts.iter()).collect::<Vec<_>>()
        );
    }
}
