use super::GameplayState;
use crate::data::{GameData, RecipeDefinition};

#[path = "gameplay_recipe_memory_text.rs"]
mod recipe_memory_text;

impl GameplayState {
    pub(super) fn recipe_is_known(&self, recipe_id: &str) -> bool {
        self.progression.known_recipes.contains(recipe_id)
    }

    pub(super) fn known_recipes<'a>(&self, data: &'a GameData) -> Vec<&'a RecipeDefinition> {
        data.recipes
            .iter()
            .filter(|recipe| self.recipe_is_known(&recipe.id))
            .collect()
    }

    pub(super) fn recipe_memory_meta(&self, _data: &GameData, recipe: &RecipeDefinition) -> String {
        let mastery = self.recipe_mastery_brews(&recipe.id);
        let best = self
            .progression
            .crafted_item_profiles
            .get(&recipe.output_item_id)
            .map(|profile| profile.best_quality_band.as_str());
        recipe_memory_text::meta(mastery, best, &recipe.catalyst_tag)
    }

    pub(super) fn recipe_memory_detail(
        &self,
        data: &GameData,
        recipe: &RecipeDefinition,
    ) -> String {
        let inherited_traits = self
            .progression
            .crafted_item_profiles
            .get(&recipe.output_item_id)
            .map(|profile| profile.inherited_traits.as_slice())
            .unwrap_or(&[]);
        recipe_memory_text::detail(data, recipe, inherited_traits)
    }
}
