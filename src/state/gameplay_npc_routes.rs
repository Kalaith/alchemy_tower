use super::gameplay_npc_pathing::{matching_arrival_position, warp_center};
use super::gameplay_npc_types::TravelSegment;
use super::GameplayState;
use crate::data::{AreaDefinition, GameData};
use macroquad::prelude::{vec2, Vec2};

impl GameplayState {
    pub(super) fn travel_segments(
        &self,
        data: &GameData,
        start_area_id: &str,
        start_position: Vec2,
        target_area_id: &str,
        target_position: Vec2,
    ) -> Option<Vec<TravelSegment>> {
        if start_area_id == target_area_id {
            let area = data.area(start_area_id)?;
            return Some(self.local_travel_segments(
                area,
                start_area_id,
                start_position,
                target_position,
            ));
        }

        let path = self.area_path(data, start_area_id, target_area_id)?;
        let mut segments = Vec::new();
        let mut current_area_id = start_area_id.to_owned();
        let mut current_position = start_position;

        for warp_id in path {
            let area = data.area(&current_area_id)?;
            let warp = area.warps.iter().find(|warp| warp.id == warp_id)?;
            let exit_position = warp_center(warp);
            segments.extend(self.local_travel_segments(
                area,
                &current_area_id,
                current_position,
                exit_position,
            ));
            let arrival_position = matching_arrival_position(data, &current_area_id, warp)
                .unwrap_or_else(|| vec2(warp.target_position[0], warp.target_position[1]));
            current_area_id = warp.target_area.clone();
            current_position = arrival_position;
        }

        let final_area = data.area(&current_area_id)?;
        segments.extend(self.local_travel_segments(
            final_area,
            &current_area_id,
            current_position,
            target_position,
        ));
        Some(segments)
    }

    pub(super) fn local_travel_segments(
        &self,
        area: &AreaDefinition,
        area_id: &str,
        start: Vec2,
        end: Vec2,
    ) -> Vec<TravelSegment> {
        let points = self.local_path_points(area, start, end);
        points
            .windows(2)
            .map(|pair| TravelSegment {
                area_id: area_id.to_owned(),
                start: pair[0],
                end: pair[1],
            })
            .collect()
    }
}
