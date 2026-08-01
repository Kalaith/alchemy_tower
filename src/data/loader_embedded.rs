use serde::Deserialize;

use super::super::embedded_json::load_labeled_json;
use crate::data::{
    AreaDefinition, GameConfig, GameDataParts, GatheringRouteDefinition, ItemDefinition,
    MutationFormulaDefinition, NpcDefinition, QuestDefinition, RecipeDefinition,
    RuneRecipeDefinition, StationDefinition,
};

/// One file per area, so a room's blockers, warps and gather nodes stay small
/// enough to read at once. This order is the order areas load in.
const AREA_SOURCES: &[(&str, &str)] = &[
    (
        "world/areas/tower_entry",
        include_str!("../../assets/data/world/areas/tower_entry.json"),
    ),
    (
        "world/areas/north_plains",
        include_str!("../../assets/data/world/areas/north_plains.json"),
    ),
    (
        "world/areas/town_square",
        include_str!("../../assets/data/world/areas/town_square.json"),
    ),
    (
        "world/areas/moonlit_forest",
        include_str!("../../assets/data/world/areas/moonlit_forest.json"),
    ),
    (
        "world/areas/rock_fields",
        include_str!("../../assets/data/world/areas/rock_fields.json"),
    ),
    (
        "world/areas/lake_shore",
        include_str!("../../assets/data/world/areas/lake_shore.json"),
    ),
    (
        "world/areas/sunscar_desert",
        include_str!("../../assets/data/world/areas/sunscar_desert.json"),
    ),
    (
        "world/areas/tropical_rainforest",
        include_str!("../../assets/data/world/areas/tropical_rainforest.json"),
    ),
    (
        "world/areas/greenhouse_floor",
        include_str!("../../assets/data/world/areas/greenhouse_floor.json"),
    ),
    (
        "world/areas/containment_floor",
        include_str!("../../assets/data/world/areas/containment_floor.json"),
    ),
    (
        "world/areas/rune_workshop_floor",
        include_str!("../../assets/data/world/areas/rune_workshop_floor.json"),
    ),
    (
        "world/areas/archive_floor",
        include_str!("../../assets/data/world/areas/archive_floor.json"),
    ),
    (
        "world/areas/observatory_floor",
        include_str!("../../assets/data/world/areas/observatory_floor.json"),
    ),
];

/// Ingredients are filed under the biome that anchors them. Herbs gathered in
/// three or more areas, or produced rather than gathered, live in `shared`.
const ITEM_SOURCES: &[(&str, &str)] = &[
    (
        "items/ingredients_shared",
        include_str!("../../assets/data/items/ingredients_shared.json"),
    ),
    (
        "items/ingredients_plains",
        include_str!("../../assets/data/items/ingredients_plains.json"),
    ),
    (
        "items/ingredients_moonlit_forest",
        include_str!("../../assets/data/items/ingredients_moonlit_forest.json"),
    ),
    (
        "items/ingredients_rock_fields",
        include_str!("../../assets/data/items/ingredients_rock_fields.json"),
    ),
    (
        "items/ingredients_lake_shore",
        include_str!("../../assets/data/items/ingredients_lake_shore.json"),
    ),
    (
        "items/ingredients_sunscar_desert",
        include_str!("../../assets/data/items/ingredients_sunscar_desert.json"),
    ),
    (
        "items/ingredients_rainforest",
        include_str!("../../assets/data/items/ingredients_rainforest.json"),
    ),
    (
        "items/materials",
        include_str!("../../assets/data/items/materials.json"),
    ),
    (
        "items/potions",
        include_str!("../../assets/data/items/potions.json"),
    ),
];

#[derive(Debug, Deserialize)]
struct EmbeddedConfigData {
    config: GameConfig,
}

#[derive(Debug, Deserialize)]
struct EmbeddedRouteData {
    #[serde(default)]
    gathering_routes: Vec<GatheringRouteDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedStationData {
    #[serde(default)]
    stations: Vec<StationDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedNpcData {
    #[serde(default)]
    npcs: Vec<NpcDefinition>,
    #[serde(default)]
    quests: Vec<QuestDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedItemData {
    items: Vec<ItemDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedCraftingData {
    recipes: Vec<RecipeDefinition>,
    #[serde(default)]
    rune_recipes: Vec<RuneRecipeDefinition>,
    #[serde(default)]
    mutation_formulas: Vec<MutationFormulaDefinition>,
}

fn load_areas() -> Result<Vec<AreaDefinition>, String> {
    AREA_SOURCES
        .iter()
        .map(|&(label, source)| load_labeled_json(label, source))
        .collect()
}

fn load_items() -> Result<Vec<ItemDefinition>, String> {
    let mut items = Vec::new();
    for &(label, source) in ITEM_SOURCES {
        let part: EmbeddedItemData = load_labeled_json(label, source)?;
        items.extend(part.items);
    }
    Ok(items)
}

pub(super) fn load_embedded_parts() -> Result<GameDataParts, String> {
    let config: EmbeddedConfigData = load_labeled_json(
        "game_data_config",
        include_str!("../../assets/data/game_data_config.json"),
    )?;
    let routes: EmbeddedRouteData = load_labeled_json(
        "world/gathering_routes",
        include_str!("../../assets/data/world/gathering_routes.json"),
    )?;
    let stations: EmbeddedStationData = load_labeled_json(
        "world/stations",
        include_str!("../../assets/data/world/stations.json"),
    )?;
    let npc: EmbeddedNpcData = load_labeled_json(
        "game_data_npcs",
        include_str!("../../assets/data/game_data_npcs.json"),
    )?;
    let crafting: EmbeddedCraftingData = load_labeled_json(
        "game_data_crafting",
        include_str!("../../assets/data/game_data_crafting.json"),
    )?;

    Ok(GameDataParts {
        config: config.config,
        areas: load_areas()?,
        gathering_routes: routes.gathering_routes,
        npcs: npc.npcs,
        quests: npc.quests,
        items: load_items()?,
        recipes: crafting.recipes,
        rune_recipes: crafting.rune_recipes,
        mutation_formulas: crafting.mutation_formulas,
        stations: stations.stations,
    })
}
