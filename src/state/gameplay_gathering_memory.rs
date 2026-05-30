use super::gameplay_feedback_types::FieldDiscoveryFeedback;
use super::GameplayState;
use crate::alchemy::quality_band;
use crate::content::ui_copy;
use crate::data::{GameData, GatherNodeDefinition, HerbMemoryEntry, ItemCategory};

impl GameplayState {
    pub(super) fn item_has_field_notes(&self, item_id: &str) -> bool {
        self.progression
            .herb_memories
            .get(item_id)
            .map(|entry| entry.learned)
            .unwrap_or(false)
    }

    pub(super) fn herb_memory(&self, item_id: &str) -> Option<&HerbMemoryEntry> {
        self.progression.herb_memories.get(item_id)
    }

    pub(super) fn herb_memories<'a>(&'a self, data: &'a GameData) -> Vec<&'a HerbMemoryEntry> {
        let mut entries = self
            .progression
            .herb_memories
            .values()
            .filter(|entry| {
                data.item(&entry.item_id)
                    .map(|item| item.category == ItemCategory::Ingredient)
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| {
            right
                .learned
                .cmp(&left.learned)
                .then(right.best_quality.cmp(&left.best_quality))
                .then(
                    data.item_name(&left.item_id)
                        .cmp(data.item_name(&right.item_id)),
                )
        });
        entries
    }

    pub(super) fn herb_memory_state_key(&self, item_id: &str) -> &'static str {
        match self.herb_memory(item_id) {
            Some(entry) if entry.learned => "journal_memory_state_learned",
            Some(entry) if entry.seen => "journal_memory_state_seen",
            _ => "journal_memory_state_unseen",
        }
    }

    pub(super) fn record_field_discovery(
        &mut self,
        data: &GameData,
        node: &GatherNodeDefinition,
    ) -> FieldDiscoveryFeedback {
        let (best_quality, variant_name) = self
            .current_item_quality_snapshot(data, node.item_id.as_str())
            .unwrap_or((0, String::new()));
        let previous = self.progression.herb_memories.get(&node.item_id).cloned();
        let variant_discovered = !variant_name.is_empty()
            && previous
                .as_ref()
                .map(|entry| entry.variant_name != variant_name)
                .unwrap_or(true);
        let improved_quality = previous
            .as_ref()
            .map(|entry| best_quality > entry.best_quality)
            .unwrap_or(true);
        let entry = self
            .progression
            .herb_memories
            .entry(node.item_id.clone())
            .or_insert_with(|| HerbMemoryEntry {
                item_id: node.item_id.clone(),
                first_seen_day: self.world.day_index,
                first_seen_route_id: node.route_id.clone(),
                seen: true,
                learned: false,
                learned_day: 0,
                learned_route_id: String::new(),
                note: String::new(),
                best_quality: 0,
                best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
                variant_name: String::new(),
            });
        if !entry.seen {
            entry.seen = true;
            entry.first_seen_day = self.world.day_index;
            entry.first_seen_route_id = node.route_id.clone();
        }
        entry.learned = true;
        entry.learned_day = self.world.day_index;
        entry.learned_route_id = node.route_id.clone();
        entry.note = node.note.clone();
        if best_quality >= entry.best_quality {
            entry.best_quality = best_quality;
            entry.best_quality_band = quality_band(best_quality).to_owned();
            entry.variant_name = variant_name;
        }
        FieldDiscoveryFeedback {
            new_note: previous.map(|entry| !entry.learned).unwrap_or(true),
            improved_quality,
            variant_discovered,
        }
    }

    pub(super) fn ensure_herb_memory_seen(&mut self, item_id: &str, route_id: Option<&str>) {
        let entry = self
            .progression
            .herb_memories
            .entry(item_id.to_owned())
            .or_insert_with(|| HerbMemoryEntry {
                item_id: item_id.to_owned(),
                first_seen_day: self.world.day_index,
                first_seen_route_id: route_id.unwrap_or_default().to_owned(),
                seen: true,
                learned: false,
                learned_day: 0,
                learned_route_id: String::new(),
                note: String::new(),
                best_quality: 0,
                best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
                variant_name: String::new(),
            });
        if !entry.seen {
            entry.seen = true;
            entry.first_seen_day = self.world.day_index;
        }
        if entry.first_seen_route_id.is_empty() {
            entry.first_seen_route_id = route_id.unwrap_or_default().to_owned();
        }
    }
}
