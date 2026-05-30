use crate::art::{draw_texture_centered, ArtAssets};
use macroquad::prelude::{get_time, vec2, Color};

pub(crate) fn draw_brew_bubble_effect(art: &ArtAssets, x: f32, y: f32, w: f32) {
    if let Some(texture) = art.effect("brew_bubble_effect") {
        let alpha = 0.55 + ((get_time() as f32 * 2.4).sin() * 0.5 + 0.5) * 0.25;
        draw_texture_centered(
            texture,
            vec2(x + w - 54.0, y + 44.0),
            vec2(42.0, 42.0),
            Color::new(1.0, 1.0, 1.0, alpha),
        );
    }
}
