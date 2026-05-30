use macroquad::prelude::*;

use crate::data::{GatherNodeDefinition, ItemCategory, StationDefinition};

use super::assets::ArtAssets;

pub fn draw_texture_cover(texture: &Texture2D, rect: Rect, tint: Color) {
    let texture_width = texture.width();
    let texture_height = texture.height();
    if texture_width <= 0.0 || texture_height <= 0.0 || rect.w <= 0.0 || rect.h <= 0.0 {
        return;
    }

    let texture_aspect = texture_width / texture_height;
    let rect_aspect = rect.w / rect.h;
    let source = if texture_aspect > rect_aspect {
        let width = texture_height * rect_aspect;
        Rect::new((texture_width - width) * 0.5, 0.0, width, texture_height)
    } else {
        let height = texture_width / rect_aspect;
        Rect::new(0.0, (texture_height - height) * 0.5, texture_width, height)
    };

    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        tint,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(vec2(rect.w, rect.h)),
            ..Default::default()
        },
    );
}

pub fn draw_texture_centered(texture: &Texture2D, center: Vec2, size: Vec2, tint: Color) {
    draw_texture_ex(
        texture,
        center.x - size.x * 0.5,
        center.y - size.y * 0.5,
        tint,
        DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        },
    );
}

pub fn draw_character_frame(
    texture: &Texture2D,
    center: Vec2,
    facing: Vec2,
    moving: bool,
    alpha: f32,
) {
    let row = if facing.y > 0.5 {
        0.0
    } else if facing.x < -0.5 {
        1.0
    } else if facing.x > 0.5 {
        2.0
    } else {
        3.0
    };
    let column = if moving {
        1.0 + ((get_time() * 7.0) as i32).rem_euclid(4) as f32
    } else {
        0.0
    };
    draw_texture_ex(
        texture,
        center.x - 32.0,
        center.y - 32.0,
        Color::new(1.0, 1.0, 1.0, alpha),
        DrawTextureParams {
            source: Some(Rect::new(column * 64.0, row * 64.0, 64.0, 64.0)),
            dest_size: Some(vec2(64.0, 64.0)),
            ..Default::default()
        },
    );
}

pub fn draw_station_marker(
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

pub fn draw_gather_node_marker(
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

pub fn draw_priority_marker(center: Vec2, color: Color) {
    let pulse = ((get_time() as f32 * 3.2) + center.x * 0.02).sin() * 0.5 + 0.5;
    let marker_y = center.y - 42.0 - pulse * 4.0;
    let bg = Color::from_rgba(20, 22, 28, 210);
    draw_rectangle(center.x - 7.0, marker_y - 13.0, 14.0, 24.0, bg);
    draw_rectangle(center.x - 3.0, marker_y + 14.0, 6.0, 6.0, bg);
    draw_rectangle(center.x - 5.0, marker_y - 11.0, 10.0, 20.0, color);
    draw_rectangle(center.x - 2.0, marker_y + 12.0, 4.0, 4.0, color);
}