use super::gameplay_path_geometry::{
    clamp_npc_point, point_outside_path_blockers, segment_is_clear,
};
use super::gameplay_world_types::PLAYER_RADIUS;
use super::GameplayState;
use crate::data::AreaDefinition;
use macroquad::prelude::{vec2, Vec2};

impl GameplayState {
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
}
