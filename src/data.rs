//! Data definitions and embedded content loading.

mod game_data;
mod loader;
mod save_models;
mod schema;

pub use game_data::GameData;
pub use save_models::{
    CraftedItemProfileEntry, ExperimentLogEntry, FieldJournalEntry, InventoryEntry,
    RecipeMasteryEntry, RelationshipEntry, SaveData,
};
pub use schema::{
    AreaDefinition, BlockerVisualStyle, EffectDefinition, EffectKind, ElementProfile, GameConfig,
    GatherNodeDefinition, GatheringRouteDefinition, HabitatStateEntry, ItemCategory,
    ItemDefinition, JournalMilestoneEntry, MutationFormulaDefinition, NpcDefinition,
    PlanterStateEntry, QuestDefinition, RecipeDefinition, RectDefinition,
    RuneRecipeDefinition, StationDefinition, StationKind, WarpDefinition,
};
pub use loader::GameDataLoader;
