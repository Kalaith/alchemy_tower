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
    (
        "world/areas/southern_pass",
        include_str!("../../assets/data/world/areas/southern_pass.json"),
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
        "items/ingredients_containment",
        include_str!("../../assets/data/items/ingredients_containment.json"),
    ),
    (
        "items/ingredients_southern_pass",
        include_str!("../../assets/data/items/ingredients_southern_pass.json"),
    ),
    (
        "items/ingredients_observatory",
        include_str!("../../assets/data/items/ingredients_observatory.json"),
    ),
    (
        "items/ingredients_archive",
        include_str!("../../assets/data/items/ingredients_archive.json"),
    ),
    (
        "items/materials",
        include_str!("../../assets/data/items/materials.json"),
    ),
    (
        "items/potions_restore",
        include_str!("../../assets/data/items/potions_restore.json"),
    ),
    (
        "items/potions_restore_tower",
        include_str!("../../assets/data/items/potions_restore_tower.json"),
    ),
    (
        "items/potions_glow",
        include_str!("../../assets/data/items/potions_glow.json"),
    ),
    // The archive's lights are a family of their own: they show a surface
    // rather than a distance, and they are brewed at the reading bench, which
    // already has its own recipe file.
    (
        "items/potions_glow_reading",
        include_str!("../../assets/data/items/potions_glow_reading.json"),
    ),
    (
        "items/potions_speed",
        include_str!("../../assets/data/items/potions_speed.json"),
    ),
    (
        "items/potions_unstable",
        include_str!("../../assets/data/items/potions_unstable.json"),
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

/// One file per room, like the areas themselves. A room's benches, beds,
/// habitats and counters are read together and belong together.
const STATION_SOURCES: &[(&str, &str)] = &[
    (
        "world/stations/tower_entry",
        include_str!("../../assets/data/world/stations/tower_entry.json"),
    ),
    (
        "world/stations/greenhouse_floor",
        include_str!("../../assets/data/world/stations/greenhouse_floor.json"),
    ),
    (
        "world/stations/town_square",
        include_str!("../../assets/data/world/stations/town_square.json"),
    ),
    (
        "world/stations/containment_floor",
        include_str!("../../assets/data/world/stations/containment_floor.json"),
    ),
    (
        "world/stations/rune_workshop_floor",
        include_str!("../../assets/data/world/stations/rune_workshop_floor.json"),
    ),
    (
        "world/stations/archive_floor",
        include_str!("../../assets/data/world/stations/archive_floor.json"),
    ),
    (
        "world/stations/observatory_floor",
        include_str!("../../assets/data/world/stations/observatory_floor.json"),
    ),
];

#[derive(Debug, Deserialize)]
struct EmbeddedNpcData {
    #[serde(default)]
    npcs: Vec<NpcDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedQuestData {
    #[serde(default)]
    quests: Vec<QuestDefinition>,
}

/// Requests split by who hands them out and what it takes to be offered one: a
/// townsperson working through their own arc, the open board in the square, the
/// standing work that only comes to somebody who has earned it, the commissions
/// the player pays into rather than being paid for, the unsigned notes, and the
/// work the valley places once it no longer needs rescuing.
///
/// The unsigned chain came out of the standing board because it is a story
/// rather than a supply arrangement — five notes in one hand with their own
/// beats — and because that file had reached 776 lines.
const QUEST_SOURCES: &[(&str, &str)] = &[
    (
        "town/quests_arcs",
        include_str!("../../assets/data/town/quests_arcs.json"),
    ),
    (
        "town/quests_board",
        include_str!("../../assets/data/town/quests_board.json"),
    ),
    (
        "town/quests_board_standing",
        include_str!("../../assets/data/town/quests_board_standing.json"),
    ),
    (
        "town/quests_board_commissions",
        include_str!("../../assets/data/town/quests_board_commissions.json"),
    ),
    (
        "town/quests_board_unsigned",
        include_str!("../../assets/data/town/quests_board_unsigned.json"),
    ),
    (
        "town/quests_board_afterward",
        include_str!("../../assets/data/town/quests_board_afterward.json"),
    ),
    // Orders that exist only because a particular piece of ground does. Filed
    // apart from the standing orders because the standing file was at 658 lines
    // and because these six share a cause rather than a tier: each is the
    // greenhouse's flat, two-reagent brew for somewhere an arc opened, and none
    // of them had a buyer until the bench was counted room by room.
    (
        "town/quests_board_ground",
        include_str!("../../assets/data/town/quests_board_ground.json"),
    ),
];

#[derive(Debug, Deserialize)]
struct EmbeddedItemData {
    items: Vec<ItemDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedRecipeData {
    #[serde(default)]
    recipes: Vec<RecipeDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedRuneRecipeData {
    #[serde(default)]
    rune_recipes: Vec<RuneRecipeDefinition>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedMutationData {
    #[serde(default)]
    mutation_formulas: Vec<MutationFormulaDefinition>,
}

/// Recipes are filed under the effect kind their output potion leads with, so
/// a formula's home is decided by what it does rather than when it was added.
const RECIPE_SOURCES: &[(&str, &str)] = &[
    (
        "crafting/recipes_restore_entry_cauldron",
        include_str!("../../assets/data/crafting/recipes_restore_entry_cauldron.json"),
    ),
    (
        "crafting/recipes_restore_greenhouse_still",
        include_str!("../../assets/data/crafting/recipes_restore_greenhouse_still.json"),
    ),
    (
        "crafting/recipes_restore_containment_cold_bench",
        include_str!("../../assets/data/crafting/recipes_restore_containment_cold_bench.json"),
    ),
    (
        "crafting/recipes_restore_rune_forge_bench",
        include_str!("../../assets/data/crafting/recipes_restore_rune_forge_bench.json"),
    ),
    (
        "crafting/recipes_restore_archive_reading_bench",
        include_str!("../../assets/data/crafting/recipes_restore_archive_reading_bench.json"),
    ),
    (
        "crafting/recipes_glow",
        include_str!("../../assets/data/crafting/recipes_glow.json"),
    ),
    (
        "crafting/recipes_glow_archive_reading_bench",
        include_str!("../../assets/data/crafting/recipes_glow_archive_reading_bench.json"),
    ),
    (
        "crafting/recipes_speed",
        include_str!("../../assets/data/crafting/recipes_speed.json"),
    ),
    (
        "crafting/recipes_speed_archive_reading_bench",
        include_str!("../../assets/data/crafting/recipes_speed_archive_reading_bench.json"),
    ),
];

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

fn load_stations() -> Result<Vec<StationDefinition>, String> {
    let mut stations = Vec::new();
    for &(label, source) in STATION_SOURCES {
        let part: EmbeddedStationData = load_labeled_json(label, source)?;
        stations.extend(part.stations);
    }
    Ok(stations)
}

fn load_quests() -> Result<Vec<QuestDefinition>, String> {
    let mut quests = Vec::new();
    for &(label, source) in QUEST_SOURCES {
        let part: EmbeddedQuestData = load_labeled_json(label, source)?;
        quests.extend(part.quests);
    }
    Ok(quests)
}

fn load_recipes() -> Result<Vec<RecipeDefinition>, String> {
    let mut recipes = Vec::new();
    for &(label, source) in RECIPE_SOURCES {
        let part: EmbeddedRecipeData = load_labeled_json(label, source)?;
        recipes.extend(part.recipes);
    }
    Ok(recipes)
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
    let npc: EmbeddedNpcData = load_labeled_json(
        "town/npcs",
        include_str!("../../assets/data/town/npcs.json"),
    )?;
    let runes: EmbeddedRuneRecipeData = load_labeled_json(
        "crafting/rune_recipes",
        include_str!("../../assets/data/crafting/rune_recipes.json"),
    )?;
    let mutations: EmbeddedMutationData = load_labeled_json(
        "crafting/mutation_formulas",
        include_str!("../../assets/data/crafting/mutation_formulas.json"),
    )?;

    Ok(GameDataParts {
        config: config.config,
        areas: load_areas()?,
        gathering_routes: routes.gathering_routes,
        npcs: npc.npcs,
        quests: load_quests()?,
        items: load_items()?,
        recipes: load_recipes()?,
        rune_recipes: runes.rune_recipes,
        mutation_formulas: mutations.mutation_formulas,
        stations: load_stations()?,
    })
}
