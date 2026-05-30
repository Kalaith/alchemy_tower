use macroquad::prelude::*;

use super::PLAYER_RADIUS;
#[cfg(test)]
use super::GameplayState;
use crate::data::{AreaDefinition, GameData, RectDefinition, WarpDefinition};

pub(super) fn schedule_start_minutes(time_window: &str) -> f32 {
    match time_window {
        "morning" => 6.0 * 60.0,
        "day" => 11.0 * 60.0,
        "evening" => 17.0 * 60.0,
        _ => 21.0 * 60.0,
    }
}

pub(super) fn warp_center(warp: &WarpDefinition) -> Vec2 {
    vec2(
        warp.rect.x + warp.rect.w * 0.5,
        warp.rect.y + warp.rect.h * 0.5,
    )
}

pub(super) fn matching_arrival_position(
    data: &GameData,
    source_area_id: &str,
    warp: &WarpDefinition,
) -> Option<Vec2> {
    let target_area = data.area(&warp.target_area)?;
    target_area
        .warps
        .iter()
        .find(|candidate| candidate.target_area == source_area_id)
        .map(warp_center)
}

pub(super) fn npc_motion_seed(id: &str) -> f32 {
    let mut value = 0u32;
    for byte in id.as_bytes() {
        value = value.wrapping_mul(33).wrapping_add(*byte as u32);
    }
    (value % 360) as f32 * 0.017453292
}

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

pub(super) fn line_intersects_expanded_rect(
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

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::GameData;

    #[test]
    fn town_to_tower_path_routes_through_plains() {
        let data = GameData::fallback();
        let state = GameplayState::new(&data);

        let path = state
            .area_path(&data, "town_square", "tower_entry")
            .expect("town should connect to tower");

        assert_eq!(
            path,
            vec!["town_to_plains".to_owned(), "plains_to_entry".to_owned()]
        );
    }

    #[test]
    fn town_to_forest_path_routes_through_plains() {
        let data = GameData::fallback();
        let state = GameplayState::new(&data);

        let path = state
            .area_path(&data, "town_square", "moonlit_forest")
            .expect("town should connect to east forest");

        assert_eq!(
            path,
            vec!["town_to_plains".to_owned(), "plains_to_forest".to_owned()]
        );
    }
}
