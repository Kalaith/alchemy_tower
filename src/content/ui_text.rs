use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use serde::Deserialize;

use super::embedded_json::parse_json_or_else;

#[derive(Debug, Deserialize)]
pub(crate) struct UiText {
    pub(crate) statuses: StatusText,
    pub(crate) prompts: PromptText,
    pub(crate) overlays: OverlayText,
    #[serde(default)]
    pub(crate) copy: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StatusText {
    pub(crate) closed_alchemy: String,
    pub(crate) closed_shop: String,
    pub(crate) closed_rune: String,
    pub(crate) closed_archive: String,
    pub(crate) closed_journal: String,
    pub(crate) closed_quest_board: String,
    pub(crate) open_journal: String,
    pub(crate) open_alchemy: String,
    pub(crate) reading_quest_board: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PromptText {
    pub(crate) open_alchemy: String,
    pub(crate) sleep_in_bed: String,
    pub(crate) browse_shop: String,
    pub(crate) open_rune_workshop: String,
    pub(crate) reconstruct_archives: String,
    pub(crate) focus_observatory: String,
    pub(crate) read_request_board: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OverlayText {
    pub(crate) shop_subtitle: String,
    pub(crate) rune_subtitle: String,
    pub(crate) archive_subtitle: String,
    pub(crate) quest_board_subtitle: String,
    pub(crate) alchemy_subtitle: String,
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

pub(crate) fn ui_text() -> &'static UiText {
    static TEXT: OnceLock<UiText> = OnceLock::new();
    TEXT.get_or_init(|| {
        parse_json_or_else(
            include_str!("../../assets/data/ui_text.json"),
            "ui_text.json",
            UiText::fallback,
        )
    })
}

pub(crate) fn ui_copy(key: &str) -> &'static str {
    static MISSING_COPY: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    ui_text()
        .copy
        .get(key)
        .map(String::as_str)
        .unwrap_or_else(|| {
            let cache = MISSING_COPY.get_or_init(|| Mutex::new(HashMap::new()));
            let Ok(mut cache) = cache.lock() else {
                return missing_ui_copy(key);
            };
            *cache.entry(key.to_owned()).or_insert_with(|| missing_ui_copy(key))
        })
}

pub(crate) fn ui_copy_optional(key: &str) -> Option<&'static str> {
    ui_text().copy.get(key).map(String::as_str)
}

pub(crate) fn ui_format(key: &str, replacements: &[(&str, &str)]) -> String {
    let mut text = ui_copy(key).to_owned();
    for (name, value) in replacements {
        text = text.replace(&format!("{{{name}}}"), value);
    }
    text
}

fn missing_ui_copy(key: &str) -> &'static str {
    Box::leak(format!("[missing ui copy: {key}]").into_boxed_str())
}
