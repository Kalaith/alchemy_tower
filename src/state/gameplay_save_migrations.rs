use std::collections::BTreeMap;

use super::gameplay_journal_support::initial_journal_milestones;
use crate::data::{FieldJournalEntry, HerbMemoryEntry, JournalMilestoneEntry};

pub(super) fn restored_herb_memories(
    herb_memories: Vec<HerbMemoryEntry>,
    field_journal: Vec<FieldJournalEntry>,
    day_index: u32,
) -> BTreeMap<String, HerbMemoryEntry> {
    let memories: BTreeMap<_, _> = herb_memories
        .into_iter()
        .map(|entry| (entry.item_id.clone(), entry))
        .collect();
    if !memories.is_empty() {
        return memories;
    }

    field_journal
        .into_iter()
        .map(|entry| {
            (
                entry.item_id.clone(),
                HerbMemoryEntry {
                    item_id: entry.item_id,
                    first_seen_day: day_index,
                    first_seen_route_id: entry.route_id.clone(),
                    seen: true,
                    learned: true,
                    learned_day: day_index,
                    learned_route_id: entry.route_id,
                    note: entry.note,
                    best_quality: entry.best_quality,
                    best_quality_band: entry.best_quality_band,
                    variant_name: entry.variant_name,
                },
            )
        })
        .collect()
}

pub(super) fn restored_journal_milestones(
    journal_milestones: Vec<JournalMilestoneEntry>,
) -> Vec<JournalMilestoneEntry> {
    if journal_milestones.is_empty() {
        initial_journal_milestones()
    } else {
        journal_milestones
    }
}
