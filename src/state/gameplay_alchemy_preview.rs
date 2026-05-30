use super::GameplayState;
use crate::alchemy::BrewResolution;
use crate::data::{GameData, StationDefinition};

impl GameplayState {
    pub(super) fn recipe_mastery_brews(&self, recipe_id: &str) -> u32 {
        self.progression
            .recipe_mastery
            .get(recipe_id)
            .copied()
            .unwrap_or_default()
    }

    pub(super) fn preview_mastery_brews(
        &self,
        data: &GameData,
        station: &StationDefinition,
        selected: &[String],
    ) -> u32 {
        crate::alchemy::match_recipe(data, station, selected)
            .map(|recipe| self.recipe_mastery_brews(&recipe.id))
            .unwrap_or_default()
    }

    pub(super) fn preview_is_uncertain(&self, preview: &BrewResolution<'_>) -> bool {
        preview
            .recipe
            .map(|recipe| !recipe.morph_targets.is_empty() && self.selected_catalyst().is_some())
            .unwrap_or(false)
    }
}
