use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

use macroquad::prelude::*;
use serde::Deserialize;

use crate::data::{
    AreaDefinition, BlockerVisualStyle, GameData, GatherNodeDefinition, ItemCategory,
    RectDefinition, StationDefinition,
};

const PLAYER_ID: &str = "player_tower_alchemist";

#[derive(Debug, Deserialize)]
struct UiArtCatalog {
    journal_tabs: Vec<JournalTabIconBinding>,
    effects: Vec<UiIconAssetDefinition>,
}

#[derive(Debug, Deserialize)]
struct JournalTabIconBinding {
    label: String,
    icon_key: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct UiIconAssetDefinition {
    key: String,
    path: String,
}

pub struct ArtAssets {
    backgrounds: HashMap<String, Texture2D>,
    characters: HashMap<String, Texture2D>,
    stations: HashMap<String, Texture2D>,
    item_icons: HashMap<String, Texture2D>,
    world_nodes: HashMap<String, Texture2D>,
    journal_tabs: HashMap<String, Texture2D>,
    journal_tab_bindings: HashMap<String, String>,
    effects: HashMap<String, Texture2D>,
}

impl ArtAssets {
    pub async fn load(data: &GameData) -> Self {
        let mut assets = Self {
            backgrounds: HashMap::new(),
            characters: HashMap::new(),
            stations: HashMap::new(),
            item_icons: HashMap::new(),
            world_nodes: HashMap::new(),
            journal_tabs: HashMap::new(),
            journal_tab_bindings: HashMap::new(),
            effects: HashMap::new(),
        };
        let catalog = ui_art_catalog();

        for area in &data.areas {
            if let Some(texture) =
                load_game_texture(&format!("assets/generated/areas/{}.png", area.id)).await
            {
                assets.backgrounds.insert(area.id.clone(), texture);
            }
        }

        let mut character_ids = vec![PLAYER_ID.to_owned()];
        character_ids.extend(data.npcs.iter().map(|npc| npc.id.clone()));
        character_ids.sort();
        character_ids.dedup();
        for id in character_ids {
            if let Some(texture) =
                load_game_texture(&format!("assets/generated/characters/{id}.png")).await
            {
                assets.characters.insert(id.clone(), texture);
            }
        }

        for station in &data.stations {
            if let Some(texture) =
                load_game_texture(&format!("assets/generated/stations/{}.png", station.id)).await
            {
                assets.stations.insert(station.id.clone(), texture);
            }
        }

        for item in &data.items {
            if let Some(texture) =
                load_game_texture(&format!("assets/generated/items/icons/{}.png", item.id)).await
            {
                assets.item_icons.insert(item.id.clone(), texture);
            }
        }

        if let Ok(entries) = fs::read_dir("assets/generated/items/world") {
            let mut paths = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("png"))
                .collect::<Vec<_>>();
            paths.sort();
            for path in paths {
                let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                    continue;
                };
                if let Some(texture) = load_game_texture(&path.to_string_lossy()).await {
                    assets.world_nodes.insert(stem.to_owned(), texture);
                }
            }
        }

        for binding in &catalog.journal_tabs {
            assets
                .journal_tab_bindings
                .insert(binding.label.clone(), binding.icon_key.clone());
            assets.journal_tabs.insert(
                binding.icon_key.clone(),
                load_game_texture(&binding.path)
                    .await
                    .unwrap_or_else(transparent_placeholder_texture),
            );
        }

        for effect in &catalog.effects {
            if let Some(texture) = load_game_texture(&effect.path).await {
                assets.effects.insert(effect.key.clone(), texture);
            }
        }

        assets
    }

    pub fn background(&self, id: &str) -> Option<&Texture2D> {
        self.backgrounds.get(id)
    }

    pub fn character(&self, id: &str) -> Option<&Texture2D> {
        self.characters.get(id)
    }

    pub fn player(&self) -> Option<&Texture2D> {
        self.character(PLAYER_ID)
    }

    pub fn station(&self, id: &str) -> Option<&Texture2D> {
        self.stations.get(id)
    }

    pub fn item_icon(&self, id: &str) -> Option<&Texture2D> {
        self.item_icons.get(id)
    }

    pub fn world_node(&self, id: &str) -> Option<&Texture2D> {
        self.world_nodes.get(id)
    }

    pub fn journal_tab(&self, key: &str) -> Option<&Texture2D> {
        self.journal_tabs.get(key)
    }

    pub fn journal_tab_by_label(&self, label: &str) -> Option<&Texture2D> {
        let key = self.journal_tab_bindings.get(label)?;
        self.journal_tab(key)
    }

    pub fn effect(&self, id: &str) -> Option<&Texture2D> {
        self.effects.get(id)
    }
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

pub fn draw_blocker_prop(
    area: &AreaDefinition,
    blocker: &RectDefinition,
    index: usize,
    offset: Vec2,
) {
    let x = offset.x + blocker.x;
    let y = offset.y + blocker.y;
    let w = blocker.w;
    let h = blocker.h;
    let shadow = Color::from_rgba(10, 12, 18, 72);
    draw_rectangle(x + 6.0, y + 8.0, w, h, shadow);

    match area.render.blocker_style {
        BlockerVisualStyle::Shelf => {
            let wood = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(124, 92, 70, 255),
            );
            let top = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(158, 122, 94, 255),
            );
            let detail = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(92, 62, 46, 255),
            );
            let bottle_a = color_from_option(
                area.render.blocker_alt,
                Color::from_rgba(170, 222, 210, 255),
            );
            draw_rectangle(x, y, w, h, wood);
            draw_rectangle(x + 6.0, y + 6.0, w - 12.0, h - 12.0, top);
            for shelf in 0..(h / 26.0).max(1.0) as i32 {
                let sy = y + 14.0 + shelf as f32 * 26.0;
                if sy < y + h - 10.0 {
                    draw_line(x + 10.0, sy, x + w - 10.0, sy, 2.0, detail);
                }
            }
            for bottle in 0..3 {
                let bx = x + 18.0 + bottle as f32 * ((w - 36.0) / 3.0);
                draw_rectangle(bx, y + 14.0, 10.0, 18.0, bottle_a);
                draw_rectangle(
                    bx + 2.0,
                    y + 34.0,
                    6.0,
                    12.0,
                    Color::from_rgba(255, 214, 132, 255),
                );
            }
        }
        BlockerVisualStyle::House => {
            let wall = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(204, 184, 150, 255),
            );
            let roof = if index % 2 == 0 {
                color_from_option(
                    area.render.blocker_secondary,
                    Color::from_rgba(160, 104, 78, 255),
                )
            } else {
                color_from_option(area.render.blocker_alt, Color::from_rgba(142, 118, 82, 255))
            };
            let doorway = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(120, 94, 72, 255),
            );
            draw_rectangle(x, y, w, h, wall);
            draw_rectangle(x - 4.0, y - 8.0, w + 8.0, 18.0, roof);
            draw_rectangle(x + 12.0, y + 18.0, w - 24.0, h - 28.0, doorway);
        }
        BlockerVisualStyle::Panel => {
            let outer = color_from_option(area.render.blocker_primary, rgba(area.accent));
            let inner = color_from_option(
                area.render.blocker_secondary,
                Color::new(
                    (outer.r + 0.12).min(1.0),
                    (outer.g + 0.12).min(1.0),
                    (outer.b + 0.12).min(1.0),
                    1.0,
                ),
            );
            let detail = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(240, 238, 220, 100),
            );
            draw_rectangle(x, y, w, h, outer);
            draw_rectangle(x + 6.0, y + 6.0, w - 12.0, h - 12.0, inner);
            draw_rectangle_lines(x, y, w, h, 2.0, detail);
        }
        BlockerVisualStyle::Grass => {
            let base = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(104, 146, 82, 255),
            );
            let light = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(146, 186, 104, 255),
            );
            let flower = color_from_option(
                area.render.blocker_alt,
                Color::from_rgba(236, 218, 132, 255),
            );
            draw_rectangle(x, y, w, h, base);
            for tuft in 0..6 {
                let tx = x + 18.0 + tuft as f32 * ((w - 36.0) / 6.0);
                let sway = if tuft % 2 == 0 { -6.0 } else { 6.0 };
                draw_triangle(
                    vec2(tx - 10.0, y + h),
                    vec2(tx, y + 18.0 + (tuft % 3) as f32 * 8.0),
                    vec2(tx + 10.0 + sway, y + h - 10.0),
                    light,
                );
                if tuft % 2 == 0 {
                    draw_circle(tx + 6.0, y + 26.0 + tuft as f32 * 3.0, 4.0, flower);
                }
            }
        }
        BlockerVisualStyle::Quarry => {
            let stone = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(116, 112, 102, 255),
            );
            let face = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(156, 148, 132, 255),
            );
            let crack = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(82, 78, 74, 255),
            );
            let mineral = color_from_option(
                area.render.blocker_alt,
                Color::from_rgba(174, 144, 238, 255),
            );
            draw_rectangle(x, y, w, h, stone);
            draw_rectangle(x + 10.0, y + 10.0, w - 20.0, h * 0.28, face);
            draw_rectangle(x + 18.0, y + h * 0.42, w - 36.0, h * 0.22, face);
            draw_rectangle(x + 12.0, y + h * 0.72, w - 24.0, h * 0.14, face);
            draw_line(x + 18.0, y + 16.0, x + w - 24.0, y + h - 22.0, 3.0, crack);
            draw_line(
                x + w * 0.48,
                y + 14.0,
                x + w * 0.36,
                y + h - 20.0,
                2.0,
                crack,
            );
            draw_triangle(
                vec2(x + w - 34.0, y + 22.0),
                vec2(x + w - 18.0, y + 48.0),
                vec2(x + w - 50.0, y + 54.0),
                mineral,
            );
        }
        BlockerVisualStyle::Forest => {
            let trunk = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(84, 58, 42, 255),
            );
            let canopy = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(58, 102, 74, 255),
            );
            let dark = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(34, 56, 42, 255),
            );
            let glow = color_from_option(
                area.render.blocker_alt,
                Color::from_rgba(168, 224, 178, 180),
            );
            draw_rectangle(x, y, w, h, dark);
            for tree in 0..3 {
                let tx = x + w * (0.22 + tree as f32 * 0.28);
                draw_rectangle(tx - 8.0, y + h * 0.45, 16.0, h * 0.38, trunk);
                draw_circle(tx, y + h * 0.34, h.min(w) * 0.16, canopy);
                draw_circle(tx - 18.0, y + h * 0.38, h.min(w) * 0.12, canopy);
                draw_circle(tx + 18.0, y + h * 0.38, h.min(w) * 0.12, canopy);
            }
            draw_circle(x + w * 0.72, y + h * 0.28, 10.0, glow);
        }
        BlockerVisualStyle::Reeds => {
            let bank = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(116, 146, 132, 255),
            );
            let water = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(88, 142, 170, 255),
            );
            let reed = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(188, 204, 124, 255),
            );
            let stone = color_from_option(
                area.render.blocker_alt,
                Color::from_rgba(154, 164, 168, 255),
            );
            draw_rectangle(x, y, w, h, bank);
            draw_rectangle(x + 6.0, y + h * 0.38, w - 12.0, h * 0.48, water);
            for tuft in 0..8 {
                let tx = x + 14.0 + tuft as f32 * ((w - 28.0) / 8.0);
                draw_line(tx, y + h * 0.44, tx - 3.0, y + h * 0.1, 3.0, reed);
                draw_line(tx + 4.0, y + h * 0.48, tx + 7.0, y + h * 0.16, 2.0, reed);
            }
            for rock in 0..3 {
                draw_circle(
                    x + 18.0 + rock as f32 * (w - 36.0) / 2.0,
                    y + h * 0.78,
                    10.0,
                    stone,
                );
            }
        }
        BlockerVisualStyle::Dunes => {
            let sand = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(198, 166, 102, 255),
            );
            let ridge = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(222, 196, 130, 255),
            );
            let stone = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(138, 102, 64, 255),
            );
            let shrub =
                color_from_option(area.render.blocker_alt, Color::from_rgba(138, 148, 92, 255));
            draw_rectangle(x, y, w, h, sand);
            draw_circle(x + w * 0.22, y + h * 0.72, h * 0.28, ridge);
            draw_circle(x + w * 0.56, y + h * 0.62, h * 0.24, ridge);
            draw_circle(x + w * 0.84, y + h * 0.78, h * 0.22, ridge);
            draw_triangle(
                vec2(x + w * 0.52, y + h * 0.26),
                vec2(x + w * 0.64, y + h * 0.52),
                vec2(x + w * 0.42, y + h * 0.58),
                stone,
            );
            draw_circle(x + 22.0, y + h - 22.0, 8.0, shrub);
            draw_circle(x + 34.0, y + h - 26.0, 6.0, shrub);
        }
        BlockerVisualStyle::Rainforest => {
            let root = color_from_option(
                area.render.blocker_primary,
                Color::from_rgba(92, 70, 54, 255),
            );
            let leaf = color_from_option(
                area.render.blocker_secondary,
                Color::from_rgba(46, 118, 82, 255),
            );
            let deep = color_from_option(
                area.render.blocker_detail,
                Color::from_rgba(26, 74, 52, 255),
            );
            let mist = color_from_option(
                area.render.blocker_alt,
                Color::from_rgba(182, 232, 214, 120),
            );
            draw_rectangle(x, y, w, h, deep);
            for cluster in 0..3 {
                let cx = x + w * (0.24 + cluster as f32 * 0.28);
                draw_rectangle(cx - 10.0, y + h * 0.46, 20.0, h * 0.36, root);
                draw_circle(cx, y + h * 0.28, h.min(w) * 0.18, leaf);
                draw_triangle(
                    vec2(cx - 26.0, y + h * 0.56),
                    vec2(cx - 4.0, y + h * 0.44),
                    vec2(cx - 12.0, y + h * 0.76),
                    root,
                );
                draw_triangle(
                    vec2(cx + 26.0, y + h * 0.56),
                    vec2(cx + 4.0, y + h * 0.44),
                    vec2(cx + 12.0, y + h * 0.76),
                    root,
                );
            }
            draw_circle(x + w * 0.82, y + h * 0.26, 12.0, mist);
        }
    }
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

async fn load_game_texture(path: &str) -> Option<Texture2D> {
    match load_texture(path).await {
        Ok(texture) => {
            texture.set_filter(FilterMode::Nearest);
            Some(texture)
        }
        Err(_) => None,
    }
}

fn transparent_placeholder_texture() -> Texture2D {
    let image = Image::gen_image_color(8, 8, Color::from_rgba(255, 255, 255, 0));
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn ui_art_catalog() -> &'static UiArtCatalog {
    static CATALOG: OnceLock<UiArtCatalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/data/ui_art.json"))
            .expect("embedded ui_art.json should be valid")
    })
}

fn color_from_option(source: Option<[u8; 4]>, fallback: Color) -> Color {
    source.map(rgba).unwrap_or(fallback)
}

fn rgba(values: [u8; 4]) -> Color {
    Color::from_rgba(values[0], values[1], values[2], values[3])
}
