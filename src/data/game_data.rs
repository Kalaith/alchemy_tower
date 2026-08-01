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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::data::load_embedded;

    /// The brew matcher keys on the exact ingredient multiset at a station, so
    /// two recipes sharing one would make the second unreachable.
    #[test]
    fn recipe_ingredient_sets_are_unique_per_station() {
        let data = load_embedded().expect("embedded game data should load");
        let mut seen: HashMap<(String, Vec<(String, u32)>), String> = HashMap::new();

        for recipe in &data.recipes {
            let mut ingredients = recipe
                .ingredients
                .iter()
                .map(|ingredient| (ingredient.item_id.clone(), ingredient.amount))
                .collect::<Vec<_>>();
            ingredients.sort();
            let key = (recipe.station_id.clone(), ingredients);
            if let Some(existing) = seen.insert(key, recipe.id.clone()) {
                panic!(
                    "{} and {} share an ingredient set at station {}",
                    existing, recipe.id, recipe.station_id
                );
            }
        }
    }

    #[test]
    fn recipe_items_and_stations_resolve() {
        let data = load_embedded().expect("embedded game data should load");
        let mut missing = Vec::new();

        for recipe in &data.recipes {
            if !data.stations.iter().any(|s| s.id == recipe.station_id) {
                missing.push(format!("{} -> station {}", recipe.id, recipe.station_id));
            }
            let mut referenced = vec![
                recipe.output_item_id.clone(),
                recipe.unstable_output_item_id.clone(),
            ];
            referenced.extend(recipe.ingredients.iter().map(|i| i.item_id.clone()));
            referenced.extend(
                recipe
                    .morph_targets
                    .iter()
                    .map(|m| m.output_item_id.clone()),
            );
            for item_id in referenced.into_iter().filter(|id| !id.is_empty()) {
                if data.item(&item_id).is_none() {
                    missing.push(format!("{} -> item {}", recipe.id, item_id));
                }
            }
        }

        assert!(
            missing.is_empty(),
            "unresolved recipe references:\n{missing:#?}"
        );
    }

    #[test]
    fn gather_nodes_resolve_their_items_and_routes() {
        let data = load_embedded().expect("embedded game data should load");
        let mut missing = Vec::new();

        for area in &data.areas {
            for node in &area.gather_nodes {
                if data.item(&node.item_id).is_none() {
                    missing.push(format!("{} -> item {}", node.id, node.item_id));
                }
                if node.route_id.is_empty() {
                    continue;
                }
                let route = data.gathering_routes.iter().find(|r| r.id == node.route_id);
                match route {
                    None => missing.push(format!("{} -> route {}", node.id, node.route_id)),
                    Some(route) if route.area_id != area.id => missing.push(format!(
                        "{} uses route {} from area {}",
                        node.id, route.id, route.area_id
                    )),
                    Some(_) => {}
                }
            }
        }

        assert!(
            missing.is_empty(),
            "unresolved gather node references:\n{missing:#?}"
        );
    }

    #[test]
    fn quests_reward_items_that_exist() {
        let data = load_embedded().expect("embedded game data should load");
        let missing = data
            .quests
            .iter()
            .filter(|quest| !quest.required_item_id.is_empty())
            .filter(|quest| data.item(&quest.required_item_id).is_none())
            .map(|quest| format!("{} -> item {}", quest.id, quest.required_item_id))
            .collect::<Vec<_>>();

        assert!(
            missing.is_empty(),
            "quests want unknown items:\n{missing:#?}"
        );
    }
}
