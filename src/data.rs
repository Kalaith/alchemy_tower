//! Data definitions and embedded content loading.

mod game_data;
mod embedded_json;
mod game_data_fallback;
mod loader;
mod save_models;
mod schema;

pub(crate) use game_data::GameDataParts;
pub(crate) use game_data::GameData;
pub(crate) use loader::{load_embedded, load_embedded_or_fallback};
pub(crate) use save_models::{
    CraftedItemProfileEntry, ExperimentLogEntry, FieldJournalEntry, HerbMemoryEntry,
    InventoryEntry, PotionMemoryEntry, RecipeMasteryEntry, RelationshipEntry, SaveData,
};
pub(crate) use schema::{
    AreaDefinition, BlockerVisualStyle, EffectDefinition, EffectKind, ElementProfile, GameConfig,
    GatherNodeDefinition, GatheringRouteDefinition, HabitatStateEntry, ItemCategory, MorphDefinition,
    ItemDefinition, JournalMilestoneEntry, MutationFormulaDefinition, NpcDefinition,
    PlanterStateEntry, QuestDefinition, RecipeDefinition, RectDefinition, RuneRecipeDefinition,
    StationDefinition, StationKind, WarpDefinition,
};
