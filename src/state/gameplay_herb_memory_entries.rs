use crate::content::ui_copy;
use crate::data::HerbMemoryEntry;

pub(super) fn new_seen_herb_memory(
    item_id: &str,
    day_index: u32,
    route_id: &str,
) -> HerbMemoryEntry {
    HerbMemoryEntry {
        item_id: item_id.to_owned(),
        first_seen_day: day_index,
        first_seen_route_id: route_id.to_owned(),
        seen: true,
        learned: false,
        learned_day: 0,
        learned_route_id: String::new(),
        note: String::new(),
        best_quality: 0,
        best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
        variant_name: String::new(),
    }
}
