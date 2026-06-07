use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad_toolkit::assets::{AssetManager, TextureConfig};

use crate::data::GameData;

use super::asset_keys::{
    asset_key, BACKGROUND_CATEGORY, CHARACTER_CATEGORY, EFFECT_CATEGORY, GENERATED_ASSET_PACK,
    ITEM_ICON_CATEGORY, JOURNAL_TAB_CATEGORY, PLAYER_ID, STATION_CATEGORY, TITLE_SCREEN_CATEGORY,
    WORLD_NODE_CATEGORY,
};
use super::asset_manifest::build_texture_manifest;

pub(crate) struct ArtAssets {
    manager: AssetManager,
    journal_tab_bindings: HashMap<String, String>,
}

impl ArtAssets {
    pub(crate) async fn load(data: &GameData) -> Result<Self, String> {
        let mut manager = AssetManager::new();
        manager.set_placeholder_texture_direct(transparent_placeholder_texture());
        let asset_pack_error = manager.load_asset_pack(GENERATED_ASSET_PACK).await.err();

        let manifest = build_texture_manifest(data);
        load_required_textures(&mut manager, &manifest.texture_configs)
            .await
            .map_err(|error| match asset_pack_error.as_ref() {
                Some(pack_error) => {
                    format!("{error}; asset pack fallback also failed: {pack_error}")
                }
                None => error,
            })?;
        if let Some(pack_error) = asset_pack_error {
            eprintln!(
                "Generated asset pack was not loaded; using loose asset files instead: {pack_error}"
            );
        }

        Ok(Self {
            manager,
            journal_tab_bindings: manifest.journal_tab_bindings,
        })
    }

    pub(crate) fn background(&self, id: &str) -> Option<&Texture2D> {
        self.texture(BACKGROUND_CATEGORY, id)
    }

    pub(crate) fn character(&self, id: &str) -> Option<&Texture2D> {
        self.texture(CHARACTER_CATEGORY, id)
    }

    pub(crate) fn player(&self) -> Option<&Texture2D> {
        self.character(PLAYER_ID)
    }

    pub(crate) fn station(&self, id: &str) -> Option<&Texture2D> {
        self.texture(STATION_CATEGORY, id)
    }

    pub(crate) fn item_icon(&self, id: &str) -> Option<&Texture2D> {
        self.texture(ITEM_ICON_CATEGORY, id)
    }

    pub(crate) fn world_node(&self, id: &str) -> Option<&Texture2D> {
        self.texture(WORLD_NODE_CATEGORY, id)
    }

    pub(crate) fn journal_tab(&self, key: &str) -> Option<&Texture2D> {
        self.manager
            .get_texture_or_placeholder(&asset_key(JOURNAL_TAB_CATEGORY, key))
    }

    pub(crate) fn journal_tab_by_label(&self, label: &str) -> Option<&Texture2D> {
        let key = self.journal_tab_bindings.get(label)?;
        self.journal_tab(key)
    }

    pub(crate) fn effect(&self, id: &str) -> Option<&Texture2D> {
        self.texture(EFFECT_CATEGORY, id)
    }

    pub(crate) fn title_screen(&self, id: &str) -> Option<&Texture2D> {
        self.texture(TITLE_SCREEN_CATEGORY, id)
    }

    fn texture(&self, category: &str, id: &str) -> Option<&Texture2D> {
        self.manager.get_texture(&asset_key(category, id))
    }
}

async fn load_required_textures(
    manager: &mut AssetManager,
    texture_configs: &[TextureConfig],
) -> Result<(), String> {
    let mut failures = Vec::new();
    for config in texture_configs {
        let filter = config
            .filter
            .map(FilterMode::from)
            .unwrap_or(FilterMode::Nearest);
        if let Err(error) = manager
            .load_texture_with_filter(&config.key, &config.path, filter)
            .await
        {
            failures.push(format!("{} from {} ({})", config.key, config.path, error));
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "failed to load {} required texture(s): {}",
            failures.len(),
            failures.join("; ")
        ))
    }
}

fn transparent_placeholder_texture() -> Texture2D {
    let image = Image::gen_image_color(8, 8, Color::from_rgba(255, 255, 255, 0));
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}
