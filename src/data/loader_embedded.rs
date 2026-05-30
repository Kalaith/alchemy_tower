use serde::Deserialize;

use super::super::embedded_json::load_labeled_json;
use crate::data::{
    AreaDefinition, GameConfig, GameDataParts, GatheringRouteDefinition, ItemDefinition,
    MutationFormulaDefinition, NpcDefinition, QuestDefinition, RecipeDefinition,
    RuneRecipeDefinition, StationDefinition,
};

#[derive(Debug, Deserialize)]
struct EmbeddedConfigData {
    config: GameConfig,
}

#[derive(Debug, Deserialize)]
struct EmbeddedWorldData {
    areas: Vec<AreaDefinition>,
    #[serde(default)]
    gathering_routes: Vec<GatheringRouteDefinition>,
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

pub(super) fn load_embedded_parts() -> Result<GameDataParts, String> {
    let config: EmbeddedConfigData = load_labeled_json(
        "game_data_config",
        include_str!("../../assets/data/game_data_config.json"),
    )?;
    let world: EmbeddedWorldData = load_labeled_json(
        "game_data_world",
        include_str!("../../assets/data/game_data_world.json"),
    )?;
    let npc: EmbeddedNpcData = load_labeled_json(
        "game_data_npcs",
        include_str!("../../assets/data/game_data_npcs.json"),
    )?;
    let items: EmbeddedItemData = load_labeled_json(
        "game_data_items",
        include_str!("../../assets/data/game_data_items.json"),
    )?;
    let crafting: EmbeddedCraftingData = load_labeled_json(
        "game_data_crafting",
        include_str!("../../assets/data/game_data_crafting.json"),
    )?;

    Ok(GameDataParts {
        config: config.config,
        areas: world.areas,
        gathering_routes: world.gathering_routes,
        npcs: npc.npcs,
        quests: npc.quests,
        items: items.items,
        recipes: crafting.recipes,
        rune_recipes: crafting.rune_recipes,
        mutation_formulas: crafting.mutation_formulas,
        stations: world.stations,
    })
}
