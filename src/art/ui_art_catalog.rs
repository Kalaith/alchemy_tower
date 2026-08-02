use std::sync::OnceLock;

use serde::Deserialize;

use super::embedded_json::load_embedded_json;

/// `deny_unknown_fields` because this file had two keys nothing read —
/// `toast_icons` and `default_toast_icon` — and serde dropped them in silence,
/// which is how six generated icons sat unused for the whole project. A key
/// that names an asset should fail to load rather than look configured.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UiArtCatalog {
    #[serde(default)]
    pub(super) title_screens: Vec<UiIconAssetDefinition>,
    pub(super) journal_tabs: Vec<JournalTabIconBinding>,
    pub(super) effects: Vec<UiIconAssetDefinition>,
    pub(super) toast_icons: Vec<UiIconAssetDefinition>,
    /// Shown for a toast whose icon key names nothing, and for the plain
    /// `push_event_toast` calls that pass no key at all.
    pub(super) default_toast_icon: String,
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

#[cfg(test)]
mod tests {
    use super::ui_art_catalog;

    /// The fallback icon has to name a real one, or a toast raised with no icon
    /// key — every `push_event_toast` call — draws nothing beside itself. This
    /// key sat in the file unread for the whole project, so it has never once
    /// been checked against the list directly underneath it.
    #[test]
    fn the_default_toast_icon_names_one_that_exists() {
        let catalog = ui_art_catalog();
        assert!(
            !catalog.toast_icons.is_empty(),
            "no toast icons are registered at all"
        );
        assert!(
            catalog
                .toast_icons
                .iter()
                .any(|icon| icon.key == catalog.default_toast_icon),
            "default_toast_icon is {:?}, which is not in toast_icons",
            catalog.default_toast_icon
        );
    }
}
