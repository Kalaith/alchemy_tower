use super::GameplayState;
use crate::data::{
    FieldJournalEntry, GameData, InventoryEntry, RecipeMasteryEntry, RelationshipEntry, SaveData,
};

pub(super) fn build_save_snapshot(state: &GameplayState, data: &GameData) -> SaveData {
    // Runtime-only UI/session fields intentionally stay transient across saves.
    SaveData {
        version: data.config.save_version,
        current_area: state.world.current_area_id.clone(),
        player_position: [state.world.player.position.x, state.world.player.position.y],
        day_clock_seconds: state.world.day_clock_seconds,
        vitality: state.vitality,
        total_brews: state.progression.total_brews,
        coins: state.coins,
        inventory: state
            .inventory
            .iter()
            .map(|(item_id, amount)| InventoryEntry {
                item_id: item_id.clone(),
                amount: *amount,
            })
            .collect(),
        gathered_nodes: state.world.gathered_nodes.iter().cloned().collect(),
        known_recipes: state.progression.known_recipes.iter().cloned().collect(),
        day_index: state.world.day_index,
        field_journal: state
            .progression
            .herb_memories
            .values()
            .filter(|entry| entry.learned)
            .map(|entry| FieldJournalEntry {
                item_id: entry.item_id.clone(),
                route_id: entry.learned_route_id.clone(),
                season: String::new(),
                weather: String::new(),
                time_window: String::new(),
                note: entry.note.clone(),
                best_quality: entry.best_quality,
                best_quality_band: entry.best_quality_band.clone(),
                variant_name: entry.variant_name.clone(),
            })
            .collect(),
        herb_memories: state.progression.herb_memories.values().cloned().collect(),
        started_quests: state.progression.started_quests.iter().cloned().collect(),
        completed_quests: state.progression.completed_quests.iter().cloned().collect(),
        recipe_mastery: state
            .progression
            .recipe_mastery
            .iter()
            .map(|(recipe_id, successful_brews)| RecipeMasteryEntry {
                recipe_id: recipe_id.clone(),
                successful_brews: *successful_brews,
            })
            .collect(),
        crafted_item_profiles: state
            .progression
            .crafted_item_profiles
            .values()
            .cloned()
            .collect(),
        experiment_log: state.progression.experiment_log.clone(),
        potion_memories: state
            .progression
            .potion_memories
            .values()
            .cloned()
            .collect(),
        unlocked_warps: state.progression.unlocked_warps.iter().cloned().collect(),
        planter_states: state.progression.planter_states.values().cloned().collect(),
        habitat_states: state.progression.habitat_states.values().cloned().collect(),
        journal_milestones: state.progression.journal_milestones.clone(),
        relationships: state
            .progression
            .relationships
            .iter()
            .map(|(npc_id, value)| RelationshipEntry {
                npc_id: npc_id.clone(),
                value: *value,
            })
            .collect(),
    }
}
