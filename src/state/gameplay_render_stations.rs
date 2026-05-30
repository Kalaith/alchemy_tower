use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};
use crate::ui::draw_station_world_marker;
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_area_stations(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
        art: &ArtAssets,
    ) {
        for station in self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.area_id == area.id)
        {
            let center = vec2(
                offset.x + station.position[0],
                offset.y + station.position[1],
            );
            let player_distance = self
                .world
                .player
                .position
                .distance(vec2(station.position[0], station.position[1]));
            let nearby = player_distance <= station.interaction_radius + 60.0;
            let priority = self.station_world_label(data, station);
            draw_station_world_marker(
                station,
                center,
                nearby,
                priority
                    .as_ref()
                    .map(|(label, color)| (label.as_str(), *color)),
                art,
            );
        }
    }
}
