//! Data definitions and embedded content loading.

mod embedded_json;
mod game_data;
#[cfg(test)]
mod game_data_apply_target_tests;
#[cfg(test)]
mod game_data_economy_tests;
mod game_data_fallback;
#[cfg(test)]
mod game_data_narrative_tests;
#[cfg(test)]
mod game_data_progression_tests;
#[cfg(test)]
mod game_data_rapport_tests;
#[cfg(test)]
mod game_data_reference_tests;
#[cfg(test)]
mod game_data_schema_tests;
#[cfg(test)]
mod game_data_world_tests;
mod loader;
mod save_models;
mod schema;

pub(crate) use game_data::GameData;
pub(crate) use game_data::GameDataParts;
#[cfg(test)]
pub(crate) use loader::load_embedded;
pub(crate) use loader::load_embedded_or_fallback;
pub(crate) use save_models::{
    BoardQuestCooldownEntry, BottleBatchEntry, CraftedItemProfileEntry, ExperimentLogEntry,
    FieldJournalEntry, HerbMemoryEntry, InventoryEntry, PotionMemoryEntry, RecipeMasteryEntry,
    RelationshipEntry, SalvageFamiliarityEntry, SaveData, VariantStockEntry,
};
pub(crate) use schema::{
    ApplyTargetDefinition, AreaDefinition, BlockerVisualStyle, EffectDefinition, EffectKind,
    ElementProfile, FlourishShape, GameConfig, GatherNodeDefinition, GatheringRouteDefinition,
    HabitatStateEntry, ItemCategory, ItemDefinition, JournalMilestoneEntry, MorphDefinition,
    MutationFormulaDefinition, NpcDefinition, PlanterStateEntry, QuestDefinition, RecipeDefinition,
    RectDefinition, RuneRecipeDefinition, SalvageTuning, StationDefinition, StationKind,
    WarpDefinition, WildVariantDefinition,
};
