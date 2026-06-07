use macroquad::prelude::*;

use crate::data::{AreaDefinition, BlockerVisualStyle, RectDefinition};

#[path = "props_architecture.rs"]
mod props_architecture;
#[path = "props_terrain.rs"]
mod props_terrain;
#[path = "props_vegetation.rs"]
mod props_vegetation;

use self::props_architecture::{draw_house_blocker, draw_panel_blocker, draw_shelf_blocker};
use self::props_terrain::{draw_dunes_blocker, draw_quarry_blocker, draw_reeds_blocker};
use self::props_vegetation::{draw_forest_blocker, draw_grass_blocker, draw_rainforest_blocker};

pub(crate) fn draw_blocker_prop(
    area: &AreaDefinition,
    blocker: &RectDefinition,
    index: usize,
    offset: Vec2,
) {
    let x = offset.x + blocker.x;
    let y = offset.y + blocker.y;
    let w = blocker.w;
    let h = blocker.h;
    draw_rectangle(x + 6.0, y + 8.0, w, h, Color::from_rgba(10, 12, 18, 72));

    match area.render.blocker_style {
        BlockerVisualStyle::Shelf => draw_shelf_blocker(area, x, y, w, h),
        BlockerVisualStyle::House => draw_house_blocker(area, index, x, y, w, h),
        BlockerVisualStyle::Panel => draw_panel_blocker(area, x, y, w, h),
        BlockerVisualStyle::Grass => draw_grass_blocker(area, x, y, w, h),
        BlockerVisualStyle::Quarry => draw_quarry_blocker(area, x, y, w, h),
        BlockerVisualStyle::Forest => draw_forest_blocker(area, x, y, w, h),
        BlockerVisualStyle::Reeds => draw_reeds_blocker(area, x, y, w, h),
        BlockerVisualStyle::Dunes => draw_dunes_blocker(area, x, y, w, h),
        BlockerVisualStyle::Rainforest => draw_rainforest_blocker(area, x, y, w, h),
    }
}

fn color_from_option(source: Option<[u8; 4]>, fallback: Color) -> Color {
    source.map(rgba).unwrap_or(fallback)
}

fn rgba(values: [u8; 4]) -> Color {
    Color::from_rgba(values[0], values[1], values[2], values[3])
}
