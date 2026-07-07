use std::collections::{BTreeMap, HashSet};

use crate::data::{
    CraftedItemProfileEntry, ExperimentLogEntry, HabitatStateEntry, HerbMemoryEntry,
    JournalMilestoneEntry, PlanterStateEntry, PotionMemoryEntry,
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
        }
    }
}
