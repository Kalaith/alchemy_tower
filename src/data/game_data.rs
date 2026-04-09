use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::schema::{
    AreaDefinition, GameConfig, GatheringRouteDefinition, ItemDefinition,
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
    pub(crate) fn from_parts(
        config: GameConfig,
        areas: Vec<AreaDefinition>,
        gathering_routes: Vec<GatheringRouteDefinition>,
        npcs: Vec<NpcDefinition>,
        quests: Vec<QuestDefinition>,
        items: Vec<ItemDefinition>,
        recipes: Vec<RecipeDefinition>,
        rune_recipes: Vec<RuneRecipeDefinition>,
        mutation_formulas: Vec<MutationFormulaDefinition>,
        stations: Vec<StationDefinition>,
    ) -> Result<Self, String> {
        let mut data = Self {
            config,
            areas,
            gathering_routes,
            npcs,
            quests,
            items,
            recipes,
            rune_recipes,
            mutation_formulas,
            stations,
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

    pub fn build_indexes(&mut self) -> Result<(), String> {
        self.area_index = build_unique_index(
            self.areas
                .iter()
                .enumerate()
                .map(|(index, area)| (&area.id, index)),
            "area",
        )?;
        self.item_index = build_unique_index(
            self.items
                .iter()
                .enumerate()
                .map(|(index, item)| (&item.id, index)),
            "item",
        )?;
        self.route_index = build_unique_index(
            self.gathering_routes
                .iter()
                .enumerate()
                .map(|(index, route)| (&route.id, index)),
            "route",
        )?;
        self.npc_index = build_unique_index(
            self.npcs
                .iter()
                .enumerate()
                .map(|(index, npc)| (&npc.id, index)),
            "npc",
        )?;
        self.quest_index = build_unique_index(
            self.quests
                .iter()
                .enumerate()
                .map(|(index, quest)| (&quest.id, index)),
            "quest",
        )?;

        let mut mutation_formula_index = HashMap::<String, Vec<usize>>::new();
        for (index, formula) in self.mutation_formulas.iter().enumerate() {
            mutation_formula_index
                .entry(formula.seed_item_id.clone())
                .or_default()
                .push(index);
        }
        self.mutation_formula_index = mutation_formula_index;
        Ok(())
    }

    pub fn area(&self, area_id: &str) -> Option<&AreaDefinition> {
        self.area_index
            .get(area_id)
            .and_then(|index| self.areas.get(*index))
    }

    pub fn fallback() -> Self {
        crate::data::GameDataLoader::load_embedded()
            .expect("embedded fallback game data must remain valid")
    }

    pub fn runtime_fallback() -> Self {
        Self::from_parts(
            GameConfig {
                starting_area: "fallback_room".to_owned(),
                starting_position: [320.0, 180.0],
                move_speed: 180.0,
                interaction_range: 42.0,
                day_length_seconds: 600.0,
                save_version: 1,
            },
            vec![AreaDefinition {
                id: "fallback_room".to_owned(),
                name: "Fallback Room".to_owned(),
                size: [640.0, 360.0],
                background: [22, 24, 30, 255],
                accent: [160, 170, 190, 255],
                blockers: Vec::new(),
                warps: Vec::new(),
                gather_nodes: Vec::new(),
                render: Default::default(),
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("static fallback game data must remain valid")
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

fn build_unique_index<'a, I>(entries: I, kind: &str) -> Result<HashMap<String, usize>, String>
where
    I: IntoIterator<Item = (&'a String, usize)>,
{
    let mut index = HashMap::new();
    let mut duplicates = Vec::new();
    for (id, value) in entries {
        if index.insert(id.clone(), value).is_some() {
            duplicates.push(id.clone());
        }
    }
    if duplicates.is_empty() {
        Ok(index)
    } else {
        duplicates.sort();
        duplicates.dedup();
        Err(format!("Duplicate {kind} id(s): {}", duplicates.join(", ")))
    }
}
