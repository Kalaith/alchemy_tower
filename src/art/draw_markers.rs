use macroquad::prelude::*;

use crate::data::{GatherNodeDefinition, ItemCategory, StationDefinition};

use super::assets::ArtAssets;
use super::draw::draw_texture_centered;

pub(crate) fn draw_station_marker(
    station: &StationDefinition,
    center: Vec2,
    emphasized: bool,
    art: &ArtAssets,
) {
    if let Some(texture) = art.station(&station.id) {
        let size = vec2(station.render.sprite_size[0], station.render.sprite_size[1]);
        draw_texture_centered(texture, center, size, WHITE);

        if !station.render.overlay_effect_id.is_empty() {
            if let Some(effect) = art.effect(&station.render.overlay_effect_id) {
                let overlay_size = vec2(
                    station.render.overlay_effect_size[0],
                    station.render.overlay_effect_size[1],
                );
                if overlay_size.length_squared() > 0.0 {
                    let alpha = 0.42 + ((get_time() as f32 * 2.2).sin() * 0.5 + 0.5) * 0.22;
                    draw_texture_centered(
                        effect,
                        center
                            + vec2(
                                station.render.overlay_effect_offset[0],
                                station.render.overlay_effect_offset[1],
                            ),
                        overlay_size,
                        Color::new(1.0, 1.0, 1.0, alpha),
                    );
                }
            }
        }

        if emphasized {
            let pulse = ((get_time() as f32 * 2.1) + center.x * 0.01).sin() * 0.5 + 0.5;
            let tint = Color::from_rgba(255, 248, 204, (150.0 + pulse * 70.0) as u8);
            draw_texture_centered(
                texture,
                center,
                size + vec2(
                    station.render.highlight_size_bonus[0],
                    station.render.highlight_size_bonus[1],
                ),
                tint,
            );
        }
    }
}

pub(crate) fn draw_gather_node_marker(
    node: &GatherNodeDefinition,
    item_category: Option<ItemCategory>,
    center: Vec2,
    color: Color,
    available: bool,
    art: &ArtAssets,
) {
    let pulse = ((get_time() as f32 * 3.2) + node.radius).sin() * 0.5 + 0.5;
    let aura_alpha = if available {
        (34.0 + pulse * 28.0) as u8
    } else {
        14
    };
    let aura = Color::new(color.r, color.g, color.b, aura_alpha as f32 / 255.0);
    draw_circle(center.x, center.y, node.radius + 2.0, aura);

    let sprite_id = if node.render.sprite_id.is_empty() {
        &node.item_id
    } else {
        &node.render.sprite_id
    };
    if let Some(texture) = art.world_node(sprite_id) {
        let pulse_scale = 1.0 + if available { pulse * 0.08 } else { 0.0 };
        let size = vec2(node.render.sprite_size[0], node.render.sprite_size[1]) * pulse_scale;
        draw_texture_centered(
            texture,
            center,
            size,
            Color::new(1.0, 1.0, 1.0, if available { 1.0 } else { 0.6 }),
        );
    } else if item_category == Some(ItemCategory::Catalyst) {
        draw_poly(center.x, center.y, 4, node.radius + 2.0, 45.0, color);
    } else {
        draw_circle(center.x, center.y, node.radius - 3.0, color);
    }
}

pub(crate) fn draw_priority_marker(center: Vec2, color: Color) {
    let pulse = ((get_time() as f32 * 3.2) + center.x * 0.02).sin() * 0.5 + 0.5;
    let marker_y = center.y - 42.0 - pulse * 4.0;
    let bg = Color::from_rgba(20, 22, 28, 210);
    draw_rectangle(center.x - 7.0, marker_y - 13.0, 14.0, 24.0, bg);
    draw_rectangle(center.x - 3.0, marker_y + 14.0, 6.0, 6.0, bg);
    draw_rectangle(center.x - 5.0, marker_y - 11.0, 10.0, 20.0, color);
    draw_rectangle(center.x - 2.0, marker_y + 12.0, 4.0, 4.0, color);
}
