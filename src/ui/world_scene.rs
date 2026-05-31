use crate::art::{draw_blocker_prop, ArtAssets};
use crate::data::AreaDefinition;
use macroquad::prelude::*;

#[path = "world_story_flourishes.rs"]
mod world_story_flourishes;
#[path = "world_weather.rs"]
mod world_weather;

pub(crate) use self::world_story_flourishes::draw_phase1_story_flourishes_view;
pub(crate) use self::world_weather::draw_environment_overlay_view;

pub(crate) fn draw_area_background(area: &AreaDefinition, offset: Vec2, art: &ArtAssets) {
    if let Some(texture) = art.background(&area.id) {
        draw_texture_ex(
            texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(area.size[0], area.size[1])),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(
            offset.x,
            offset.y,
            area.size[0],
            area.size[1],
            rgba(area.background),
        );
    }
}

pub(crate) fn draw_area_blockers(area: &AreaDefinition, offset: Vec2) {
    for (index, blocker) in area.blockers.iter().enumerate() {
        draw_blocker_prop(area, blocker, index, offset);
    }
}

fn rgba(color: [u8; 4]) -> Color {
    Color::from_rgba(color[0], color[1], color[2], color[3])
}
