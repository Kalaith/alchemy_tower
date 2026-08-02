use std::collections::{BTreeMap, HashSet};

use crate::data::{
    BottleBatchEntry, CraftedItemProfileEntry, ExperimentLogEntry, HabitatStateEntry,
    HerbMemoryEntry, JournalMilestoneEntry, PlanterStateEntry, PotionMemoryEntry,
};

#[derive(Clone, Debug)]
pub(super) struct ProgressionState {
    pub(super) total_brews: u32,
    pub(super) known_recipes: HashSet<String>,
    pub(super) recipe_mastery: BTreeMap<String, u32>,
    pub(super) crafted_item_profiles: BTreeMap<String, CraftedItemProfileEntry>,
    pub(super) experiment_log: Vec<ExperimentLogEntry>,
    pub(super) unlocked_warps: HashSet<String>,
    pub(super) planter_states: BTreeMap<String, PlanterStateEntry>,
    pub(super) habitat_states: BTreeMap<String, HabitatStateEntry>,
    pub(super) journal_milestones: Vec<JournalMilestoneEntry>,
    pub(super) relationships: BTreeMap<String, i32>,
    pub(super) started_quests: HashSet<String>,
    pub(super) completed_quests: HashSet<String>,
    /// Repeatable board request id -> the day index on/after which it may be
    /// offered again. Absent means never delivered (or not repeatable).
    pub(super) board_quest_cooldowns: BTreeMap<String, u32>,
    pub(super) herb_memories: BTreeMap<String, HerbMemoryEntry>,
    pub(super) potion_memories: BTreeMap<String, PotionMemoryEntry>,
    /// item id -> variant id -> how many held units came up as that variant.
    /// The plain `inventory` count remains the total held; this records which
    /// part of it was gathered under the right sky. Without it a variant was a
    /// line in the journal and nothing else, because inventory counts units and
    /// has nowhere to say one unit is better than another.
    pub(super) variant_stock: BTreeMap<String, BTreeMap<String, u32>>,
    /// item id -> the batches of that item currently on the shelf, worst first.
    /// A request used to be checked against `crafted_item_profiles`, which
    /// records the best the player has *ever* brewed, so one masterwork
    /// satisfied every later gate forever. Bottles have their own quality now.
    pub(super) bottle_stock: BTreeMap<String, Vec<BottleBatchEntry>>,
    /// Reaction lines a townsperson has already said, keyed `npc_id#order`.
    /// Earning a reaction is monotonic and the selector took the highest-order
    /// earned line, so any line that became earned alongside a later one was
    /// shadowed forever. Remembering what has been said lets the earlier ones
    /// have their turn first.
    pub(super) spoken_reactions: HashSet<String>,
}

impl ProgressionState {
    pub(super) fn new(journal_milestones: Vec<JournalMilestoneEntry>) -> Self {
        Self {
            total_brews: 0,
            known_recipes: HashSet::new(),
            recipe_mastery: BTreeMap::new(),
            crafted_item_profiles: BTreeMap::new(),
            experiment_log: Vec::new(),
            unlocked_warps: HashSet::new(),
            planter_states: BTreeMap::new(),
            habitat_states: BTreeMap::new(),
            journal_milestones,
            relationships: BTreeMap::new(),
            started_quests: HashSet::new(),
            completed_quests: HashSet::new(),
            board_quest_cooldowns: BTreeMap::new(),
            herb_memories: BTreeMap::new(),
            potion_memories: BTreeMap::new(),
            variant_stock: BTreeMap::new(),
            bottle_stock: BTreeMap::new(),
            spoken_reactions: HashSet::new(),
        }
    }
}
