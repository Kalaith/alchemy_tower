use std::collections::HashMap;
use std::sync::OnceLock;

use macroquad::prelude::*;
use macroquad_toolkit::assets::{AssetManager, TextureConfig, TextureFilter};
use serde::Deserialize;

use crate::data::GameData;

const PLAYER_ID: &str = "player_tower_alchemist";
const GENERATED_ASSET_PACK: &str = "assets/generated.zip";

#[derive(Debug, Deserialize)]
struct UiArtCatalog {
    #[serde(default)]
    title_screens: Vec<UiIconAssetDefinition>,
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
    manager: AssetManager,
    journal_tab_bindings: HashMap<String, String>,
}

impl ArtAssets {
    pub async fn load(data: &GameData) -> Self {
        let mut manager = AssetManager::new();
        manager.set_placeholder_texture_direct(transparent_placeholder_texture());
        manager.load_asset_pack(GENERATED_ASSET_PACK).await.ok();

        let mut texture_configs = Vec::new();
        let mut journal_tab_bindings = HashMap::new();
        let catalog = ui_art_catalog();

        for area in &data.areas {
            push_texture_config(
                &mut texture_configs,
                "background",
                &area.id,
                format!("assets/generated/areas/{}.png", area.id),
            );
        }

        let mut character_ids = vec![PLAYER_ID.to_owned()];
        character_ids.extend(data.npcs.iter().map(|npc| npc.id.clone()));
        character_ids.sort();
        character_ids.dedup();
        for id in character_ids {
            push_texture_config(
                &mut texture_configs,
                "character",
                &id,
                format!("assets/generated/characters/{id}.png"),
            );
        }

        for station in &data.stations {
            push_texture_config(
                &mut texture_configs,
                "station",
                &station.id,
                format!("assets/generated/stations/{}.png", station.id),
            );
        }

        for item in &data.items {
            push_texture_config(
                &mut texture_configs,
                "item_icon",
                &item.id,
                format!("assets/generated/items/icons/{}.png", item.id),
            );
        }

        let mut world_node_ids = data
            .areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter())
            .map(|node| {
                if node.render.sprite_id.is_empty() {
                    node.item_id.clone()
                } else {
                    node.render.sprite_id.clone()
                }
            })
            .collect::<Vec<_>>();
        world_node_ids.sort();
        world_node_ids.dedup();
        for id in world_node_ids {
            push_texture_config(
                &mut texture_configs,
                "world_node",
                &id,
                format!("assets/generated/items/world/{id}.png"),
            );
        }

        for binding in &catalog.journal_tabs {
            journal_tab_bindings.insert(binding.label.clone(), binding.icon_key.clone());
            push_texture_config(
                &mut texture_configs,
                "journal_tab",
                &binding.icon_key,
                binding.path.clone(),
            );
        }

        for effect in &catalog.effects {
            push_texture_config(
                &mut texture_configs,
                "effect",
                &effect.key,
                effect.path.clone(),
            );
        }

        for title_screen in &catalog.title_screens {
            push_texture_config_with_filter(
                &mut texture_configs,
                "title_screen",
                &title_screen.key,
                title_screen.path.clone(),
                TextureFilter::Linear,
            );
        }

        manager.load_texture_configs(&texture_configs).await;

        Self {
            manager,
            journal_tab_bindings,
        }
    }

    pub fn background(&self, id: &str) -> Option<&Texture2D> {
        self.texture("background", id)
    }

    pub fn character(&self, id: &str) -> Option<&Texture2D> {
        self.texture("character", id)
    }

    pub fn player(&self) -> Option<&Texture2D> {
        self.character(PLAYER_ID)
    }

    pub fn station(&self, id: &str) -> Option<&Texture2D> {
        self.texture("station", id)
    }

    pub fn item_icon(&self, id: &str) -> Option<&Texture2D> {
        self.texture("item_icon", id)
    }

    pub fn world_node(&self, id: &str) -> Option<&Texture2D> {
        self.texture("world_node", id)
    }

    pub fn journal_tab(&self, key: &str) -> Option<&Texture2D> {
        self.manager
            .get_texture_or_placeholder(&asset_key("journal_tab", key))
    }

    pub fn journal_tab_by_label(&self, label: &str) -> Option<&Texture2D> {
        let key = self.journal_tab_bindings.get(label)?;
        self.journal_tab(key)
    }

    pub fn effect(&self, id: &str) -> Option<&Texture2D> {
        self.texture("effect", id)
    }

    pub fn title_screen(&self, id: &str) -> Option<&Texture2D> {
        self.texture("title_screen", id)
    }

    fn texture(&self, category: &str, id: &str) -> Option<&Texture2D> {
        self.manager.get_texture(&asset_key(category, id))
    }
}

fn transparent_placeholder_texture() -> Texture2D {
    let image = Image::gen_image_color(8, 8, Color::from_rgba(255, 255, 255, 0));
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn push_texture_config(configs: &mut Vec<TextureConfig>, category: &str, id: &str, path: String) {
    configs.push(TextureConfig {
        key: asset_key(category, id),
        path,
        filter: None,
    });
}

fn push_texture_config_with_filter(
    configs: &mut Vec<TextureConfig>,
    category: &str,
    id: &str,
    path: String,
    filter: TextureFilter,
) {
    configs.push(TextureConfig {
        key: asset_key(category, id),
        path,
        filter: Some(filter),
    });
}

fn asset_key(category: &str, id: &str) -> String {
    format!("{category}:{id}")
}

fn ui_art_catalog() -> &'static UiArtCatalog {
    static CATALOG: OnceLock<UiArtCatalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/data/ui_art.json"))
            .expect("embedded ui_art.json should be valid")
    })
}