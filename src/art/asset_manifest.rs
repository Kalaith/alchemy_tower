use std::collections::HashMap;

use super::asset_keys::{
    asset_key, BACKGROUND_CATEGORY, CHARACTER_CATEGORY, EFFECT_CATEGORY, ITEM_ICON_CATEGORY,
    JOURNAL_TAB_CATEGORY, PLAYER_ID, STATION_CATEGORY, TITLE_SCREEN_CATEGORY, WORLD_NODE_CATEGORY,
};
use super::ui_art_catalog::{ui_art_catalog, UiArtCatalog};
use macroquad_toolkit::assets::{TextureConfig, TextureFilter};

use crate::data::GameData;

pub(super) struct TextureManifest {
    pub(super) texture_configs: Vec<TextureConfig>,
    pub(super) journal_tab_bindings: HashMap<String, String>,
}

pub(super) fn build_texture_manifest(data: &GameData) -> TextureManifest {
    let mut texture_configs = Vec::new();
    let mut journal_tab_bindings = HashMap::new();
    let catalog = ui_art_catalog();

    add_area_backgrounds(&mut texture_configs, data);
    add_characters(&mut texture_configs, data);
    add_stations(&mut texture_configs, data);
    add_item_icons(&mut texture_configs, data);
    add_world_nodes(&mut texture_configs, data);
    add_journal_tabs(&mut texture_configs, &mut journal_tab_bindings, catalog);
    add_effects(&mut texture_configs, catalog);
    add_title_screens(&mut texture_configs, catalog);

    TextureManifest {
        texture_configs,
        journal_tab_bindings,
    }
}

fn add_area_backgrounds(configs: &mut Vec<TextureConfig>, data: &GameData) {
    for area in &data.areas {
        push_texture_config(
            configs,
            BACKGROUND_CATEGORY,
            &area.id,
            format!("assets/generated/areas/{}.png", area.id),
        );
    }
}

fn add_characters(configs: &mut Vec<TextureConfig>, data: &GameData) {
    let mut character_ids = vec![PLAYER_ID.to_owned()];
    character_ids.extend(data.npcs.iter().map(|npc| npc.id.clone()));
    character_ids.sort();
    character_ids.dedup();
    for id in character_ids {
        push_texture_config(
            configs,
            CHARACTER_CATEGORY,
            &id,
            format!("assets/generated/characters/{id}.png"),
        );
    }
}

fn add_stations(configs: &mut Vec<TextureConfig>, data: &GameData) {
    for station in &data.stations {
        push_texture_config(
            configs,
            STATION_CATEGORY,
            &station.id,
            format!("assets/generated/stations/{}.png", station.id),
        );
    }
}

fn add_item_icons(configs: &mut Vec<TextureConfig>, data: &GameData) {
    for item in &data.items {
        push_texture_config(
            configs,
            ITEM_ICON_CATEGORY,
            &item.id,
            format!("assets/generated/items/icons/{}.png", item.id),
        );
    }
}

fn add_world_nodes(configs: &mut Vec<TextureConfig>, data: &GameData) {
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
            configs,
            WORLD_NODE_CATEGORY,
            &id,
            format!("assets/generated/items/world/{id}.png"),
        );
    }
}

fn add_journal_tabs(
    configs: &mut Vec<TextureConfig>,
    journal_tab_bindings: &mut HashMap<String, String>,
    catalog: &UiArtCatalog,
) {
    for binding in &catalog.journal_tabs {
        journal_tab_bindings.insert(binding.label.clone(), binding.icon_key.clone());
        push_texture_config(
            configs,
            JOURNAL_TAB_CATEGORY,
            &binding.icon_key,
            binding.path.clone(),
        );
    }
}

fn add_effects(configs: &mut Vec<TextureConfig>, catalog: &UiArtCatalog) {
    for effect in &catalog.effects {
        push_texture_config(configs, EFFECT_CATEGORY, &effect.key, effect.path.clone());
    }
}

fn add_title_screens(configs: &mut Vec<TextureConfig>, catalog: &UiArtCatalog) {
    for title_screen in &catalog.title_screens {
        push_texture_config_with_filter(
            configs,
            TITLE_SCREEN_CATEGORY,
            &title_screen.key,
            title_screen.path.clone(),
            TextureFilter::Linear,
        );
    }
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
