//! Embedded data loading.

use serde::Deserialize;

use crate::data::{
    AreaDefinition, GameConfig, GameData, GatheringRouteDefinition, ItemDefinition,
    MutationFormulaDefinition, NpcDefinition, QuestDefinition, RecipeDefinition,
    RuneRecipeDefinition, StationDefinition,
};
use macroquad_toolkit::data_loader::load_embedded_json_labeled;

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

pub struct GameDataLoader;

impl GameDataLoader {
    pub fn load_embedded() -> Result<GameData, String> {
        let config: EmbeddedConfigData = load_embedded_json_labeled(
            "game_data_config",
            include_str!("../../assets/data/game_data_config.json"),
        )?;
        let world: EmbeddedWorldData = load_embedded_json_labeled(
            "game_data_world",
            include_str!("../../assets/data/game_data_world.json"),
        )?;
        let npc: EmbeddedNpcData = load_embedded_json_labeled(
            "game_data_npcs",
            include_str!("../../assets/data/game_data_npcs.json"),
        )?;
        let items: EmbeddedItemData = load_embedded_json_labeled(
            "game_data_items",
            include_str!("../../assets/data/game_data_items.json"),
        )?;
        let crafting: EmbeddedCraftingData = load_embedded_json_labeled(
            "game_data_crafting",
            include_str!("../../assets/data/game_data_crafting.json"),
        )?;

        GameData::from_parts(
            config.config,
            world.areas,
            world.gathering_routes,
            npc.npcs,
            npc.quests,
            items.items,
            crafting.recipes,
            crafting.rune_recipes,
            crafting.mutation_formulas,
            world.stations,
        )
    }
}
