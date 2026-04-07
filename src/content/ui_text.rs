use std::sync::OnceLock;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UiText {
    pub statuses: StatusText,
    pub prompts: PromptText,
    pub overlays: OverlayText,
    #[serde(default)]
    pub copy: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusText {
    pub closed_alchemy: String,
    pub closed_shop: String,
    pub closed_rune: String,
    pub closed_archive: String,
    pub closed_journal: String,
    pub closed_quest_board: String,
    pub open_journal: String,
    pub open_alchemy: String,
    pub reading_quest_board: String,
}

#[derive(Debug, Deserialize)]
pub struct PromptText {
    pub open_alchemy: String,
    pub sleep_in_bed: String,
    pub browse_shop: String,
    pub open_rune_workshop: String,
    pub reconstruct_archives: String,
    pub focus_observatory: String,
    pub read_request_board: String,
}

#[derive(Debug, Deserialize)]
pub struct OverlayText {
    pub shop_subtitle: String,
    pub rune_subtitle: String,
    pub archive_subtitle: String,
    pub quest_board_subtitle: String,
    pub alchemy_subtitle: String,
}

pub fn ui_text() -> &'static UiText {
    static TEXT: OnceLock<UiText> = OnceLock::new();
    TEXT.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/data/ui_text.json"))
            .expect("embedded ui_text.json should be valid")
    })
}

pub fn ui_copy(key: &str) -> &'static str {
    ui_text()
        .copy
        .get(key)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("missing ui text copy key: {key}"))
}

pub fn ui_format(key: &str, replacements: &[(&str, &str)]) -> String {
    let mut text = ui_copy(key).to_owned();
    for (name, value) in replacements {
        text = text.replace(&format!("{{{name}}}"), value);
    }
    text
}
