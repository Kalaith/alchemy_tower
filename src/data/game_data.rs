use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::schema::{
    AreaDefinition, GameConfig, GatheringRouteDefinition, ItemDefinition,
    MutationFormulaDefinition, NpcDefinition, QuestDefinition, RecipeDefinition,
    RuneRecipeDefinition, StationDefinition,
};

#[path = "game_data_access.rs"]
mod game_data_access;
#[path = "game_data_indexes.rs"]
mod game_data_indexes;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GameData {
    pub(crate) config: GameConfig,
    pub(crate) areas: Vec<AreaDefinition>,
    #[serde(default)]
    pub(crate) gathering_routes: Vec<GatheringRouteDefinition>,
    #[serde(default)]
    pub(crate) npcs: Vec<NpcDefinition>,
    #[serde(default)]
    pub(crate) quests: Vec<QuestDefinition>,
    pub(crate) items: Vec<ItemDefinition>,
    pub(crate) recipes: Vec<RecipeDefinition>,
    #[serde(default)]
    pub(crate) rune_recipes: Vec<RuneRecipeDefinition>,
    #[serde(default)]
    pub(crate) mutation_formulas: Vec<MutationFormulaDefinition>,
    pub(crate) stations: Vec<StationDefinition>,
    #[serde(skip)]
    area_index: HashMap<String, usize>,
    #[serde(skip)]
    item_index: HashMap<String, usize>,
    #[serde(skip)]
    route_index: HashMap<String, usize>,
    #[serde(skip)]
    npc_index: HashMap<String, usize>,
    #[serde(skip)]
    quest_index: HashMap<String, usize>,
    #[serde(skip)]
    mutation_formula_index: HashMap<String, Vec<usize>>,
}

pub(crate) struct GameDataParts {
    pub(crate) config: GameConfig,
    pub(crate) areas: Vec<AreaDefinition>,
    pub(crate) gathering_routes: Vec<GatheringRouteDefinition>,
    pub(crate) npcs: Vec<NpcDefinition>,
    pub(crate) quests: Vec<QuestDefinition>,
    pub(crate) items: Vec<ItemDefinition>,
    pub(crate) recipes: Vec<RecipeDefinition>,
    pub(crate) rune_recipes: Vec<RuneRecipeDefinition>,
    pub(crate) mutation_formulas: Vec<MutationFormulaDefinition>,
    pub(crate) stations: Vec<StationDefinition>,
}

impl GameData {
    pub(crate) fn from_parts(parts: GameDataParts) -> Result<Self, String> {
        let mut data = Self {
            config: parts.config,
            areas: parts.areas,
            gathering_routes: parts.gathering_routes,
            npcs: parts.npcs,
            quests: parts.quests,
            items: parts.items,
            recipes: parts.recipes,
            rune_recipes: parts.rune_recipes,
            mutation_formulas: parts.mutation_formulas,
            stations: parts.stations,
            area_index: HashMap::new(),
            item_index: HashMap::new(),
            route_index: HashMap::new(),
            npc_index: HashMap::new(),
            quest_index: HashMap::new(),
            mutation_formula_index: HashMap::new(),
        };
        data.build_indexes()?;
        Ok(data)
    }
}
