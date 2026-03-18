use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::schema::{
    AreaDefinition, GatheringRouteDefinition, GameConfig, ItemDefinition,
    MutationFormulaDefinition, NpcDefinition, QuestDefinition, RecipeDefinition,
    RuneRecipeDefinition, StationDefinition,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameData {
    pub config: GameConfig,
    pub areas: Vec<AreaDefinition>,
    #[serde(default)]
    pub gathering_routes: Vec<GatheringRouteDefinition>,
    #[serde(default)]
    pub npcs: Vec<NpcDefinition>,
    #[serde(default)]
    pub quests: Vec<QuestDefinition>,
    pub items: Vec<ItemDefinition>,
    pub recipes: Vec<RecipeDefinition>,
    #[serde(default)]
    pub rune_recipes: Vec<RuneRecipeDefinition>,
    #[serde(default)]
    pub mutation_formulas: Vec<MutationFormulaDefinition>,
    pub stations: Vec<StationDefinition>,
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

impl GameData {
    pub fn build_indexes(&mut self) {
        self.area_index = self
            .areas
            .iter()
            .enumerate()
            .map(|(index, area)| (area.id.clone(), index))
            .collect();
        self.item_index = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| (item.id.clone(), index))
            .collect();
        self.route_index = self
            .gathering_routes
            .iter()
            .enumerate()
            .map(|(index, route)| (route.id.clone(), index))
            .collect();
        self.npc_index = self
            .npcs
            .iter()
            .enumerate()
            .map(|(index, npc)| (npc.id.clone(), index))
            .collect();
        self.quest_index = self
            .quests
            .iter()
            .enumerate()
            .map(|(index, quest)| (quest.id.clone(), index))
            .collect();

        let mut mutation_formula_index = HashMap::<String, Vec<usize>>::new();
        for (index, formula) in self.mutation_formulas.iter().enumerate() {
            mutation_formula_index
                .entry(formula.seed_item_id.clone())
                .or_default()
                .push(index);
        }
        self.mutation_formula_index = mutation_formula_index;
    }

    pub fn area(&self, area_id: &str) -> Option<&AreaDefinition> {
        self.area_index
            .get(area_id)
            .and_then(|index| self.areas.get(*index))
    }

    pub fn fallback() -> Self {
        let mut data: Self =
            serde_json::from_str(include_str!("../../assets/data/game_data.json"))
                .expect("embedded fallback game_data.json must remain valid");
        data.build_indexes();
        data
    }

    pub fn item(&self, item_id: &str) -> Option<&ItemDefinition> {
        self.item_index
            .get(item_id)
            .and_then(|index| self.items.get(*index))
    }

    pub fn item_name<'a>(&'a self, item_id: &'a str) -> &'a str {
        self.item(item_id)
            .map(|item| item.name.as_str())
            .unwrap_or(item_id)
    }

    pub fn route(&self, route_id: &str) -> Option<&GatheringRouteDefinition> {
        self.route_index
            .get(route_id)
            .and_then(|index| self.gathering_routes.get(*index))
    }

    pub fn npc(&self, npc_id: &str) -> Option<&NpcDefinition> {
        self.npc_index
            .get(npc_id)
            .and_then(|index| self.npcs.get(*index))
    }

    pub fn quest(&self, quest_id: &str) -> Option<&QuestDefinition> {
        self.quest_index
            .get(quest_id)
            .and_then(|index| self.quests.get(*index))
    }

    #[cfg(test)]
    pub fn recipe_for_output(&self, item_id: &str) -> Option<&RecipeDefinition> {
        self.recipes
            .iter()
            .find(|recipe| recipe.output_item_id == item_id)
    }

    pub fn mutation_formulas_for_seed(
        &self,
        seed_item_id: &str,
    ) -> Vec<&MutationFormulaDefinition> {
        self.mutation_formula_index
            .get(seed_item_id)
            .into_iter()
            .flatten()
            .filter_map(|index| self.mutation_formulas.get(*index))
            .collect()
    }
}
