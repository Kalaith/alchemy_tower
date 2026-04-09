use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

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

impl UiText {
    fn fallback() -> Self {
        Self {
            statuses: StatusText {
                closed_alchemy: "[missing ui_text.statuses.closed_alchemy]".to_owned(),
                closed_shop: "[missing ui_text.statuses.closed_shop]".to_owned(),
                closed_rune: "[missing ui_text.statuses.closed_rune]".to_owned(),
                closed_archive: "[missing ui_text.statuses.closed_archive]".to_owned(),
                closed_journal: "[missing ui_text.statuses.closed_journal]".to_owned(),
                closed_quest_board: "[missing ui_text.statuses.closed_quest_board]".to_owned(),
                open_journal: "[missing ui_text.statuses.open_journal]".to_owned(),
                open_alchemy: "[missing ui_text.statuses.open_alchemy]".to_owned(),
                reading_quest_board: "[missing ui_text.statuses.reading_quest_board]".to_owned(),
            },
            prompts: PromptText {
                open_alchemy: "[missing ui_text.prompts.open_alchemy]".to_owned(),
                sleep_in_bed: "[missing ui_text.prompts.sleep_in_bed]".to_owned(),
                browse_shop: "[missing ui_text.prompts.browse_shop]".to_owned(),
                open_rune_workshop: "[missing ui_text.prompts.open_rune_workshop]".to_owned(),
                reconstruct_archives: "[missing ui_text.prompts.reconstruct_archives]".to_owned(),
                focus_observatory: "[missing ui_text.prompts.focus_observatory]".to_owned(),
                read_request_board: "[missing ui_text.prompts.read_request_board]".to_owned(),
            },
            overlays: OverlayText {
                shop_subtitle: "[missing ui_text.overlays.shop_subtitle]".to_owned(),
                rune_subtitle: "[missing ui_text.overlays.rune_subtitle]".to_owned(),
                archive_subtitle: "[missing ui_text.overlays.archive_subtitle]".to_owned(),
                quest_board_subtitle: "[missing ui_text.overlays.quest_board_subtitle]".to_owned(),
                alchemy_subtitle: "[missing ui_text.overlays.alchemy_subtitle]".to_owned(),
            },
            copy: HashMap::new(),
        }
    }
}

pub fn ui_text() -> &'static UiText {
    static TEXT: OnceLock<UiText> = OnceLock::new();
    TEXT.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/data/ui_text.json")).unwrap_or_else(
            |error| {
                eprintln!("Failed to load embedded ui_text.json: {error}");
                UiText::fallback()
            },
        )
    })
}

pub fn ui_copy(key: &str) -> &'static str {
    static MISSING_COPY: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    ui_text()
        .copy
        .get(key)
        .map(String::as_str)
        .unwrap_or_else(|| {
            let cache = MISSING_COPY.get_or_init(|| Mutex::new(HashMap::new()));
            let mut cache = cache
                .lock()
                .expect("missing ui text cache lock should succeed");
            *cache
                .entry(key.to_owned())
                .or_insert_with(|| Box::leak(format!("[missing ui copy: {key}]").into_boxed_str()))
        })
}

pub fn ui_copy_optional(key: &str) -> Option<&'static str> {
    ui_text().copy.get(key).map(String::as_str)
}

pub fn ui_format(key: &str, replacements: &[(&str, &str)]) -> String {
    let mut text = ui_copy(key).to_owned();
    for (name, value) in replacements {
        text = text.replace(&format!("{{{name}}}"), value);
    }
    text
}
