use crate::content::ui_copy;
use crate::data::PotionMemoryEntry;

pub(super) fn new_seen_potion_memory(item_id: &str, day_index: u32) -> PotionMemoryEntry {
    PotionMemoryEntry {
        item_id: item_id.to_owned(),
        first_seen_day: day_index,
        seen: true,
        learned: false,
        learned_day: 0,
        successful_brews: 0,
        best_quality_score: 0,
        best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
        last_recipe_id: String::new(),
    }
}

pub(super) fn empty_effects_text() -> String {
    ui_copy("journal_memory_effects_none").to_owned()
}
