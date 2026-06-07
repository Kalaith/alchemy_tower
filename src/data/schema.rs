//! Serializable game definitions shared across systems.

#[path = "schema_alchemy.rs"]
mod schema_alchemy;
#[path = "schema_core.rs"]
mod schema_core;
#[path = "schema_items.rs"]
mod schema_items;
#[path = "schema_npcs.rs"]
mod schema_npcs;
#[path = "schema_progression.rs"]
mod schema_progression;
#[path = "schema_render.rs"]
mod schema_render;
#[path = "schema_stations.rs"]
mod schema_stations;
#[path = "schema_world.rs"]
mod schema_world;

pub(crate) use self::schema_alchemy::{
    MorphDefinition, MutationFormulaDefinition, RecipeDefinition, RoomBonusDefinition,
    RuneRecipeDefinition,
};
pub(crate) use self::schema_core::{EffectKind, ElementProfile, ItemCategory, StationKind};
pub(crate) use self::schema_items::{EffectDefinition, ItemDefinition};
pub(crate) use self::schema_npcs::NpcDefinition;
pub(crate) use self::schema_progression::{HabitatStateEntry, PlanterStateEntry};
pub(crate) use self::schema_render::{
    AreaRenderDefinition, BlockerVisualStyle, GatherNodeRenderDefinition, StationRenderDefinition,
};
pub(crate) use self::schema_stations::StationDefinition;
pub(crate) use self::schema_world::{
    AreaDefinition, GameConfig, GatherNodeDefinition, GatheringRouteDefinition,
    JournalMilestoneEntry, QuestDefinition, RectDefinition, WarpDefinition,
};
