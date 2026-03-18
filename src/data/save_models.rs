use serde::{Deserialize, Serialize};

use super::schema::{HabitatStateEntry, JournalMilestoneEntry, PlanterStateEntry};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InventoryEntry {
    pub item_id: String,
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecipeMasteryEntry {
    pub recipe_id: String,
    pub successful_brews: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CraftedItemProfileEntry {
    pub item_id: String,
    #[serde(default)]
    pub best_quality_score: u32,
    #[serde(default)]
    pub best_quality_band: String,
    #[serde(default)]
    pub inherited_traits: Vec<String>,
    #[serde(default)]
    pub effect_kinds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExperimentLogEntry {
    #[serde(default)]
    pub recipe_id: String,
    pub output_item_id: String,
    #[serde(default)]
    pub quality_score: u32,
    #[serde(default)]
    pub quality_band: String,
    #[serde(default)]
    pub stable: bool,
    #[serde(default)]
    pub catalyst_item_id: String,
    #[serde(default)]
    pub morph_output_item_id: String,
    #[serde(default)]
    pub day_index: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaveData {
    pub version: u32,
    pub current_area: String,
    pub player_position: [f32; 2],
    pub day_clock_seconds: f32,
    #[serde(default = "default_vitality")]
    pub vitality: f32,
    #[serde(default)]
    pub coins: u32,
    pub inventory: Vec<InventoryEntry>,
    pub gathered_nodes: Vec<String>,
    #[serde(default)]
    pub known_recipes: Vec<String>,
    #[serde(default)]
    pub day_index: u32,
    #[serde(default)]
    pub field_journal: Vec<FieldJournalEntry>,
    #[serde(default)]
    pub started_quests: Vec<String>,
    #[serde(default)]
    pub completed_quests: Vec<String>,
    #[serde(default)]
    pub recipe_mastery: Vec<RecipeMasteryEntry>,
    #[serde(default)]
    pub crafted_item_profiles: Vec<CraftedItemProfileEntry>,
    #[serde(default)]
    pub experiment_log: Vec<ExperimentLogEntry>,
    #[serde(default)]
    pub total_brews: u32,
    #[serde(default)]
    pub unlocked_warps: Vec<String>,
    #[serde(default)]
    pub planter_states: Vec<PlanterStateEntry>,
    #[serde(default)]
    pub journal_milestones: Vec<JournalMilestoneEntry>,
    #[serde(default)]
    pub relationships: Vec<RelationshipEntry>,
    #[serde(default)]
    pub habitat_states: Vec<HabitatStateEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationshipEntry {
    pub npc_id: String,
    pub value: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldJournalEntry {
    pub item_id: String,
    pub route_id: String,
    pub season: String,
    pub weather: String,
    pub time_window: String,
    pub note: String,
    #[serde(default)]
    pub best_quality: u32,
    #[serde(default)]
    pub best_quality_band: String,
    #[serde(default)]
    pub variant_name: String,
}

fn default_vitality() -> f32 {
    100.0
}
