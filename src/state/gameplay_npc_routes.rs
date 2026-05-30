use super::gameplay_npc_pathing::{
    clamp_npc_point, matching_arrival_position, point_outside_path_blockers, segment_is_clear,
    warp_center,
};
use super::{GameplayState, TravelSegment, PLAYER_RADIUS};
use crate::data::{AreaDefinition, GameData};
use macroquad::prelude::{vec2, Vec2};
use std::collections::BTreeMap;

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

    pub(super) fn local_path_points(
        &self,
        area: &AreaDefinition,
        start: Vec2,
        end: Vec2,
    ) -> Vec<Vec2> {
        let margin = PLAYER_RADIUS + 16.0;
        let corner_padding = 8.0;
        let start = clamp_npc_point(area, start);
        let end = clamp_npc_point(area, end);
        let mut nodes = vec![start, end];
        for blocker in &area.blockers {
            let left = (blocker.x - margin - corner_padding).max(PLAYER_RADIUS + corner_padding);
            let right = (blocker.x + blocker.w + margin + corner_padding)
                .min(area.size[0] - PLAYER_RADIUS - corner_padding);
            let top = (blocker.y - margin - corner_padding).max(PLAYER_RADIUS + corner_padding);
            let bottom = (blocker.y + blocker.h + margin + corner_padding)
                .min(area.size[1] - PLAYER_RADIUS - corner_padding);
            nodes.push(vec2(left, top));
            nodes.push(vec2(right, top));
            nodes.push(vec2(left, bottom));
            nodes.push(vec2(right, bottom));
        }
        nodes.retain(|point| point_outside_path_blockers(area, *point, margin * 0.85));

        let mut distances = vec![f32::INFINITY; nodes.len()];
        let mut previous = vec![None::<usize>; nodes.len()];
        let mut visited = vec![false; nodes.len()];
        distances[0] = 0.0;

        for _ in 0..nodes.len() {
            let Some(current) = (0..nodes.len())
                .filter(|index| !visited[*index])
                .min_by(|left, right| distances[*left].total_cmp(&distances[*right]))
            else {
                break;
            };
            if current == 1 {
                break;
            }
            visited[current] = true;
            for next in 0..nodes.len() {
                if current == next || visited[next] {
                    continue;
                }
                if !segment_is_clear(area, nodes[current], nodes[next], margin) {
                    continue;
                }
                let candidate = distances[current] + nodes[current].distance(nodes[next]);
                if candidate < distances[next] {
                    distances[next] = candidate;
                    previous[next] = Some(current);
                }
            }
        }

        if !distances[1].is_finite() {
            for corner in [vec2(start.x, end.y), vec2(end.x, start.y)] {
                let corner = clamp_npc_point(area, corner);
                if segment_is_clear(area, start, corner, margin)
                    && segment_is_clear(area, corner, end, margin)
                {
                    return vec![start, corner, end];
                }
            }
            return vec![start];
        }

        let mut order = vec![1usize];
        let mut current = 1usize;
        while let Some(prev) = previous[current] {
            order.push(prev);
            current = prev;
            if current == 0 {
                break;
            }
        }
        order.reverse();
        order.into_iter().map(|index| nodes[index]).collect()
    }

    pub(super) fn area_path(
        &self,
        data: &GameData,
        start_area_id: &str,
        target_area_id: &str,
    ) -> Option<Vec<String>> {
        let mut frontier = vec![start_area_id.to_owned()];
        let mut came_from = BTreeMap::<String, (String, String)>::new();
        let mut index = 0;

        while index < frontier.len() {
            let area_id = frontier[index].clone();
            index += 1;
            if area_id == target_area_id {
                break;
            }
            let area = data.area(&area_id)?;
            for warp in &area.warps {
                if !came_from.contains_key(&warp.target_area) && warp.target_area != start_area_id {
                    came_from.insert(warp.target_area.clone(), (area_id.clone(), warp.id.clone()));
                    frontier.push(warp.target_area.clone());
                }
            }
        }

        if start_area_id != target_area_id && !came_from.contains_key(target_area_id) {
            return None;
        }

        let mut path = Vec::<String>::new();
        let mut current = target_area_id.to_owned();
        while current != start_area_id {
            let (previous_area, warp_id) = came_from.get(&current)?.clone();
            path.push(warp_id);
            current = previous_area;
        }
        path.reverse();
        Some(path)
    }
}
