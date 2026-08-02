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
    #[serde(default)]
    pub(crate) board_quest_cooldowns: Vec<BoardQuestCooldownEntry>,
    #[serde(default)]
    pub(crate) variant_stock: Vec<VariantStockEntry>,
    #[serde(default)]
    pub(crate) bottle_stock: Vec<BottleBatchEntry>,
    #[serde(default)]
    pub(crate) spoken_reactions: Vec<String>,
    #[serde(default)]
    pub(crate) salvage_familiarity: Vec<SalvageFamiliarityEntry>,
    #[serde(default)]
    pub(crate) treated_targets: Vec<String>,
    #[serde(default)]
    pub(crate) shown_tutorial_hints: Vec<String>,
}

/// How many times the player has run one particular off-book mixture. Brewing
/// something no recipe describes used to be a dead end that always handed back
/// the same four salvage bottles; doing it repeatedly is now how a formula gets
/// found without anybody writing it down first.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct SalvageFamiliarityEntry {
    pub(crate) signature: String,
    pub(crate) attempts: u32,
}

/// How many of the player's units of `item_id` were gathered as a particular
/// wild variant. The plain inventory count stays the total; this says how much
/// of that total is the good stuff.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct VariantStockEntry {
    pub(crate) item_id: String,
    pub(crate) variant_id: String,
    pub(crate) count: u32,
}

/// Bottles of one item that came off the bench together, and how good they
/// were. The inventory count is still the total held; this says what the
/// individual bottles making up that total are actually worth, so a request can
/// be checked against the shelf rather than against the player's best-ever
/// record. Bottles from anywhere but the bench — bought, gifted, granted — have
/// no batch and count as a plain example of the item.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct BottleBatchEntry {
    pub(crate) item_id: String,
    pub(crate) quality_score: u32,
    pub(crate) quality_band: String,
    pub(crate) traits: Vec<String>,
    pub(crate) count: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RelationshipEntry {
    pub(crate) npc_id: String,
    pub(crate) value: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct BoardQuestCooldownEntry {
    pub(crate) quest_id: String,
    pub(crate) available_day: u32,
}

fn default_vitality() -> f32 {
    100.0
}
