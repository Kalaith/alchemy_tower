use std::collections::HashMap;

use macroquad::prelude::*;

use crate::data::GameData;

const PLAYER_ID: &str = "player_tower_alchemist";
const JOURNAL_TAB_KEYS: [&str; 5] = ["routes", "notes", "brews", "greenhouse", "rapport"];
const TOAST_ICON_KEYS: [&str; 6] = [
    "journal_note",
    "recipe_logged",
    "quest_accepted",
    "quest_complete",
    "route_restored",
    "best_quality",
];

pub struct ArtAssets {
    backgrounds: HashMap<String, Texture2D>,
    characters: HashMap<String, Texture2D>,
    stations: HashMap<String, Texture2D>,
    item_icons: HashMap<String, Texture2D>,
    world_nodes: HashMap<String, Texture2D>,
    journal_tabs: HashMap<String, Texture2D>,
    toast_icons: HashMap<String, Texture2D>,
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
            toast_icons: HashMap::new(),
            effects: HashMap::new(),
        };

        for area in &data.areas {
            assets.backgrounds.insert(
                area.id.clone(),
                load_game_texture(&format!("assets/generated/areas/{}.png", area.id)).await,
            );
        }

        let mut character_ids = vec![PLAYER_ID.to_owned()];
        character_ids.extend(data.npcs.iter().map(|npc| npc.id.clone()));
        character_ids.sort();
        character_ids.dedup();
        for id in character_ids {
            assets.characters.insert(
                id.clone(),
                load_game_texture(&format!("assets/generated/characters/{id}.png")).await,
            );
        }

        for station in &data.stations {
            assets.stations.insert(
                station.id.clone(),
                load_game_texture(&format!("assets/generated/stations/{}.png", station.id)).await,
            );
        }

        for item in &data.items {
            assets.item_icons.insert(
                item.id.clone(),
                load_game_texture(&format!("assets/generated/items/icons/{}.png", item.id)).await,
            );
            assets.world_nodes.insert(
                item.id.clone(),
                load_game_texture(&format!("assets/generated/items/world/{}.png", item.id)).await,
            );
        }

        for key in JOURNAL_TAB_KEYS {
            assets.journal_tabs.insert(
                key.to_owned(),
                load_game_texture(&format!("assets/generated/ui/journal_tabs/{key}.png")).await,
            );
        }

        for key in TOAST_ICON_KEYS {
            assets.toast_icons.insert(
                key.to_owned(),
                load_game_texture(&format!("assets/generated/ui/toasts/{key}.png")).await,
            );
        }

        for key in ["gather_feedback_sparkle", "brew_bubble_effect", "warp_glow_effect"] {
            assets.effects.insert(
                key.to_owned(),
                load_game_texture(&format!("assets/generated/effects/{key}.png")).await,
            );
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

    pub fn toast_icon(&self, key: &str) -> Option<&Texture2D> {
        self.toast_icons.get(key)
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

pub fn draw_character_frame(texture: &Texture2D, center: Vec2, facing: Vec2, moving: bool, alpha: f32) {
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

pub fn toast_icon_for_text(text: &str) -> &'static str {
    let lower = text.to_ascii_lowercase();
    if lower.contains("journal") || lower.contains("note") {
        "journal_note"
    } else if lower.contains("recipe") || lower.contains("formula") {
        "recipe_logged"
    } else if lower.contains("accepted") || lower.contains("started") {
        "quest_accepted"
    } else if lower.contains("completed") || lower.contains("delivered") {
        "quest_complete"
    } else if lower.contains("restored") || lower.contains("repaired") || lower.contains("unlock") {
        "route_restored"
    } else {
        "best_quality"
    }
}

pub fn journal_tab_icon_key(tab: &str) -> &'static str {
    match tab {
        "Routes" => "routes",
        "Notes" => "notes",
        "Brews" => "brews",
        "Greenhouse" => "greenhouse",
        _ => "rapport",
    }
}

async fn load_game_texture(path: &str) -> Texture2D {
    match load_texture(path).await {
        Ok(texture) => {
            texture.set_filter(FilterMode::Nearest);
            texture
        }
        Err(_) => fallback_texture(),
    }
}

fn fallback_texture() -> Texture2D {
    let image = Image::gen_image_color(8, 8, Color::from_rgba(255, 0, 255, 255));
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}
