use serde::{Deserialize, Serialize};

use super::schema::{HabitatStateEntry, JournalMilestoneEntry, PlanterStateEntry};

#[path = "save_memory_models.rs"]
mod save_memory_models;
pub(crate) use self::save_memory_models::{FieldJournalEntry, HerbMemoryEntry, PotionMemoryEntry};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct InventoryEntry {
    pub(crate) item_id: String,
    pub(crate) amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RecipeMasteryEntry {
    pub(crate) recipe_id: String,
    pub(crate) successful_brews: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CraftedItemProfileEntry {
    pub(crate) item_id: String,
    #[serde(default)]
    pub(crate) best_quality_score: u32,
    #[serde(default)]
    pub(crate) best_quality_band: String,
    #[serde(default)]
    pub(crate) inherited_traits: Vec<String>,
    #[serde(default)]
    pub(crate) effect_kinds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ExperimentLogEntry {
    #[serde(default)]
    pub(crate) recipe_id: String,
    pub(crate) output_item_id: String,
    #[serde(default)]
    pub(crate) quality_score: u32,
    #[serde(default)]
    pub(crate) quality_band: String,
    #[serde(default)]
    pub(crate) stable: bool,
    #[serde(default)]
    pub(crate) catalyst_item_id: String,
    #[serde(default)]
    pub(crate) morph_output_item_id: String,
    #[serde(default)]
    pub(crate) day_index: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct SaveData {
    pub(crate) version: u32,
    pub(crate) current_area: String,
    pub(crate) player_position: [f32; 2],
    pub(crate) day_clock_seconds: f32,
    #[serde(default = "default_vitality")]
    pub(crate) vitality: f32,
    #[serde(default)]
    pub(crate) coins: u32,
    pub(crate) inventory: Vec<InventoryEntry>,
    pub(crate) gathered_nodes: Vec<String>,
    #[serde(default)]
    pub(crate) known_recipes: Vec<String>,
    #[serde(default)]
    pub(crate) day_index: u32,
    #[serde(default)]
    pub(crate) field_journal: Vec<FieldJournalEntry>,
    #[serde(default)]
    pub(crate) herb_memories: Vec<HerbMemoryEntry>,
    #[serde(default)]
    pub(crate) started_quests: Vec<String>,
    #[serde(default)]
    pub(crate) completed_quests: Vec<String>,
    #[serde(default)]
    pub(crate) recipe_mastery: Vec<RecipeMasteryEntry>,
    #[serde(default)]
    pub(crate) crafted_item_profiles: Vec<CraftedItemProfileEntry>,
    #[serde(default)]
    pub(crate) experiment_log: Vec<ExperimentLogEntry>,
    #[serde(default)]
    pub(crate) potion_memories: Vec<PotionMemoryEntry>,
    #[serde(default)]
    pub(crate) total_brews: u32,
    #[serde(default)]
    pub(crate) unlocked_warps: Vec<String>,
    #[serde(default)]
    pub(crate) planter_states: Vec<PlanterStateEntry>,
    #[serde(default)]
    pub(crate) journal_milestones: Vec<JournalMilestoneEntry>,
    #[serde(default)]
    pub(crate) relationships: Vec<RelationshipEntry>,
    #[serde(default)]
    pub(crate) habitat_states: Vec<HabitatStateEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RelationshipEntry {
    pub(crate) npc_id: String,
    pub(crate) value: i32,
}

fn default_vitality() -> f32 {
    100.0
}
