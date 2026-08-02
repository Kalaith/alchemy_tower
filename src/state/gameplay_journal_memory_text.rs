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

    /// The herb entry's flavour line, cut to its first sentence.
    ///
    /// Two thirds of the descriptions wrap to three lines in the detail box,
    /// which holds four — so carrying the whole paragraph here cost the
    /// gathering conditions and the "brews into" line, the two things the entry
    /// is actually consulted for. The opening sentence is the part that says
    /// what the thing is; the rest is colour the player has already read once
    /// on the pickup.
    pub(super) fn journal_herb_lead(&self, data: &GameData, item_id: &str) -> String {
        let summary = self.journal_herb_summary(data, item_id);
        let Some(end) = summary.find(". ") else {
            return summary;
        };
        summary[..=end].to_owned()
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
