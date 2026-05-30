use std::sync::OnceLock;

use serde::Deserialize;

use super::embedded_json::load_embedded_json;

#[derive(Debug, Deserialize)]
pub(super) struct UiArtCatalog {
    #[serde(default)]
    pub(super) title_screens: Vec<UiIconAssetDefinition>,
    pub(super) journal_tabs: Vec<JournalTabIconBinding>,
    pub(super) effects: Vec<UiIconAssetDefinition>,
}

#[derive(Debug, Deserialize)]
pub(super) struct JournalTabIconBinding {
    pub(super) label: String,
    pub(super) icon_key: String,
    pub(super) path: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct UiIconAssetDefinition {
    pub(super) key: String,
    pub(super) path: String,
}

pub(super) fn ui_art_catalog() -> &'static UiArtCatalog {
    static CATALOG: OnceLock<UiArtCatalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        load_embedded_json("ui_art.json", include_str!("../../assets/data/ui_art.json"))
    })
}
