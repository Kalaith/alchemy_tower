use super::GameData;
#[cfg(test)]
use crate::data::RecipeDefinition;
use crate::data::{
    AreaDefinition, GatheringRouteDefinition, ItemDefinition, MutationFormulaDefinition,
    NpcDefinition, QuestDefinition,
};

impl GameData {
    pub(crate) fn area(&self, area_id: &str) -> Option<&AreaDefinition> {
        self.area_index
            .get(area_id)
            .and_then(|index| self.areas.get(*index))
    }

    pub(crate) fn item(&self, item_id: &str) -> Option<&ItemDefinition> {
        self.item_index
            .get(item_id)
            .and_then(|index| self.items.get(*index))
    }

    pub(crate) fn item_name<'a>(&'a self, item_id: &'a str) -> &'a str {
        self.item(item_id)
            .map(|item| item.name.as_str())
            .unwrap_or(item_id)
    }

    pub(crate) fn route(&self, route_id: &str) -> Option<&GatheringRouteDefinition> {
        self.route_index
            .get(route_id)
            .and_then(|index| self.gathering_routes.get(*index))
    }

    pub(crate) fn npc(&self, npc_id: &str) -> Option<&NpcDefinition> {
        self.npc_index
            .get(npc_id)
            .and_then(|index| self.npcs.get(*index))
    }

    pub(crate) fn quest(&self, quest_id: &str) -> Option<&QuestDefinition> {
        self.quest_index
            .get(quest_id)
            .and_then(|index| self.quests.get(*index))
    }

    #[cfg(test)]
    pub(crate) fn recipe_for_output(&self, item_id: &str) -> Option<&RecipeDefinition> {
        self.recipes
            .iter()
            .find(|recipe| recipe.output_item_id == item_id)
    }

    pub(crate) fn mutation_formulas_for_seed(
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
