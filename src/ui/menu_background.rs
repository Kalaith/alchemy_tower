use crate::art::{draw_texture_cover, ArtAssets};
use crate::data::GameData;
use macroquad::prelude::*;

pub(super) fn draw_title_background(data: &GameData, art: &ArtAssets) -> bool {
    if let Some(texture) = art.title_screen("main") {
        draw_texture_cover(
            texture,
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            WHITE,
        );
        return true;
    }

    if let Some(texture) = art.background(&data.config.starting_area) {
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            Color::from_rgba(255, 255, 255, 215),
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
    false
}

pub(super) fn draw_title_vignette(has_title_screen: bool) {
    let base_alpha = if has_title_screen { 62 } else { 126 };
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(8, 10, 16, base_alpha),
    );

    let band_height = screen_height() * 0.42;
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        band_height,
        Color::from_rgba(8, 10, 16, 58),
    );
    draw_rectangle(
        0.0,
        screen_height() - band_height * 0.7,
        screen_width(),
        band_height * 0.7,
        Color::from_rgba(8, 10, 16, 86),
    );
}
