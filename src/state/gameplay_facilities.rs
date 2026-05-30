use super::GameplayState;
use crate::data::{GameData, StationDefinition};
use macroquad::prelude::vec2;

impl GameplayState {
    pub(super) fn nearby_station<'a>(&self, data: &'a GameData) -> Option<&'a StationDefinition> {
        self.visible_stations(data).into_iter().find(|station| {
            station.area_id == self.world.current_area_id
                && self
                    .world
                    .player
                    .position
                    .distance(vec2(station.position[0], station.position[1]))
                    <= station.interaction_radius
        })
    }

    pub(super) fn visible_stations<'a>(&self, data: &'a GameData) -> Vec<&'a StationDefinition> {
        data.stations
            .iter()
            .filter(|station| {
                station.required_completed_quest.is_empty()
                    || self
                        .progression
                        .completed_quests
                        .contains(&station.required_completed_quest)
            })
            .filter(|station| self.progression.total_brews >= station.required_total_brews)
            .filter(|station| {
                station.required_journal_milestone.is_empty()
                    || self.has_journal_milestone(&station.required_journal_milestone)
            })
            .collect()
    }
}
