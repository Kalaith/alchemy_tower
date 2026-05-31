use macroquad::prelude::{vec2, Vec2};

use super::gameplay_world_types::PLAYER_RADIUS;
use crate::data::{AreaDefinition, RectDefinition};

pub(super) fn segment_is_clear(area: &AreaDefinition, start: Vec2, end: Vec2, margin: f32) -> bool {
    !area
        .blockers
        .iter()
        .any(|blocker| line_intersects_expanded_rect(start, end, blocker, margin))
}

pub(super) fn clamp_npc_point(area: &AreaDefinition, point: Vec2) -> Vec2 {
    vec2(
        point
            .x
            .clamp(PLAYER_RADIUS + 4.0, area.size[0] - PLAYER_RADIUS - 4.0),
        point
            .y
            .clamp(PLAYER_RADIUS + 4.0, area.size[1] - PLAYER_RADIUS - 4.0),
    )
}

fn point_inside_expanded_blocker(area: &AreaDefinition, point: Vec2, margin: f32) -> bool {
    area.blockers.iter().any(|rect| {
        point.x > rect.x - margin
            && point.x < rect.x + rect.w + margin
            && point.y > rect.y - margin
            && point.y < rect.y + rect.h + margin
    })
}

pub(super) fn point_outside_path_blockers(
    area: &AreaDefinition,
    point: Vec2,
    margin: f32,
) -> bool {
    !point_inside_expanded_blocker(area, point, margin)
}

fn line_intersects_expanded_rect(
    start: Vec2,
    end: Vec2,
    rect: &RectDefinition,
    margin: f32,
) -> bool {
    let min_x = rect.x - margin;
    let max_x = rect.x + rect.w + margin;
    let min_y = rect.y - margin;
    let max_y = rect.y + rect.h + margin;

    if start.x > min_x && start.x < max_x && start.y > min_y && start.y < max_y {
        return true;
    }
    if end.x > min_x && end.x < max_x && end.y > min_y && end.y < max_y {
        return true;
    }

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let mut t0 = 0.0;
    let mut t1 = 1.0;

    for (p, q) in [
        (-dx, start.x - min_x),
        (dx, max_x - start.x),
        (-dy, start.y - min_y),
        (dy, max_y - start.y),
    ] {
        if p.abs() <= f32::EPSILON {
            if q < 0.0 {
                return false;
            }
            continue;
        }
        let r = q / p;
        if p < 0.0 {
            if r > t1 {
                return false;
            }
            if r > t0 {
                t0 = r;
            }
        } else {
            if r < t0 {
                return false;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }

    t0 <= t1
}
