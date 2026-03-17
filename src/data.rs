//! Data definitions and embedded content loading.

mod definitions;
mod loader;

pub use definitions::{
    AreaDefinition, CraftedItemProfileEntry, EffectDefinition, EffectKind, ElementProfile,
    ExperimentLogEntry, FieldJournalEntry, GameData, GatherNodeDefinition, HabitatStateEntry,
    InventoryEntry, ItemCategory, ItemDefinition, JournalMilestoneEntry, NpcDefinition,
    PlanterStateEntry, QuestDefinition, RecipeDefinition, RecipeMasteryEntry, RectDefinition,
    RelationshipEntry, RuneRecipeDefinition, SaveData, StationDefinition, StationKind,
    WarpDefinition,
};
pub use loader::GameDataLoader;
