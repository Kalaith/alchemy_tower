use super::world_marker_plates::draw_world_marker_plate;
#[path = "world_warp_markers.rs"]
mod world_warp_markers;

pub(crate) use self::world_warp_markers::draw_warp_marker;

use crate::art::{
    draw_character_frame, draw_gather_node_marker, draw_priority_marker, draw_station_marker,
    ArtAssets,
};
use crate::data::{GatherNodeDefinition, ItemCategory, NpcDefinition, StationDefinition};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_station_world_marker(
    station: &StationDefinition,
    center: Vec2,
    nearby: bool,
    priority: Option<(&str, Color)>,
    art: &ArtAssets,
) {
    draw_station_marker(station, center, priority.is_some(), art);
    if nearby || priority.is_some() {
        draw_world_marker_plate(
            &station.name,
            vec2(center.x, center.y + 46.0),
            dark::TEXT_BRIGHT,
            false,
        );
    }
    if let Some((label, color)) = priority {
        draw_world_marker_plate(label, vec2(center.x, center.y - 34.0), color, true);
    }
}

pub(crate) fn draw_gather_node_world_marker(
    node: &GatherNodeDefinition,
    item_category: Option<ItemCategory>,
    center: Vec2,
    color: Color,
    available: bool,
    art: &ArtAssets,
) {
    draw_gather_node_marker(node, item_category, center, color, available, art);
}

pub(crate) fn draw_npc_world_marker(
    npc: &NpcDefinition,
    center: Vec2,
    facing: Vec2,
    moving: bool,
    fallback_color: Color,
    show_name: bool,
    priority: Option<(&str, Color)>,
    art: &ArtAssets,
) {
    if let Some(texture) = art.character(&npc.id) {
        draw_character_frame(texture, center, facing, moving, 1.0);
    } else {
        draw_circle(center.x, center.y, 18.0, fallback_color);
    }
    if show_name {
        draw_text(
            &npc.name,
            center.x - 34.0,
            center.y - 28.0,
            18.0,
            dark::TEXT_BRIGHT,
        );
    }
    if let Some((label, color)) = priority {
        draw_priority_marker(center, color);
        draw_text(label, center.x - 34.0, center.y - 50.0, 18.0, color);
    }
}

pub(crate) fn draw_player_world_marker(
    center: Vec2,
    radius: f32,
    facing: Vec2,
    moving: bool,
    glow_active: bool,
    art: &ArtAssets,
) {
    if glow_active {
        draw_circle(
            center.x,
            center.y,
            radius + 18.0,
            Color::from_rgba(215, 202, 255, 70),
        );
    }
    if let Some(texture) = art.player() {
        draw_character_frame(texture, center, facing, moving, 1.0);
    } else {
        draw_circle(
            center.x,
            center.y,
            radius,
            Color::from_rgba(133, 204, 255, 255),
        );
        draw_circle_lines(center.x, center.y, radius, 2.0, WHITE);
        draw_circle(center.x + 5.0, center.y - 4.0, 2.5, WHITE);
    }
}
