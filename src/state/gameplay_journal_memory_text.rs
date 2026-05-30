use super::GameplayState;
use crate::content::{ui_copy, ui_copy_optional, ui_format};
use crate::data::{GameData, PotionMemoryEntry};

impl GameplayState {
    pub(super) fn journal_herb_summary(&self, data: &GameData, item_id: &str) -> String {
        let key = format!("journal_herb_summary_{item_id}");
        ui_copy_optional(&key)
            .map(str::to_owned)
            .or_else(|| data.item(item_id).map(|item| item.description.clone()))
            .unwrap_or_else(|| data.item_name(item_id).to_owned())
    }

    pub(super) fn journal_potion_recap(&self, data: &GameData, item_id: &str) -> String {
        let key = format!("journal_potion_recap_{item_id}");
        ui_copy_optional(&key)
            .map(str::to_owned)
            .or_else(|| data.item(item_id).map(|item| item.description.clone()))
            .unwrap_or_else(|| data.item_name(item_id).to_owned())
    }

    pub(super) fn journal_potion_state_line(&self, entry: &PotionMemoryEntry) -> String {
        let mut parts = Vec::new();
        if entry.seen {
            parts.push(ui_copy("journal_memory_state_seen").to_owned());
        }
        if entry.learned {
            parts.push(ui_copy("journal_memory_state_learned").to_owned());
        }
        if entry.successful_brews > 0 {
            parts.push(ui_copy("journal_memory_state_brewed").to_owned());
        }
        ui_format(
            "journal_memory_state_line",
            &[("state", &parts.join("  |  "))],
        )
    }
}
