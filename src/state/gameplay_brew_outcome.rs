use super::GameplayState;
use crate::alchemy::{mastery_stage, BrewResolution};
use crate::data::GameData;

#[path = "gameplay_brew_outcome_text.rs"]
mod outcome_text;

impl GameplayState {
    pub(super) fn update_brew_result_status(
        &mut self,
        data: &GameData,
        resolution: &BrewResolution<'_>,
        stable_brew: bool,
    ) {
        if let Some(recipe) = resolution.recipe {
            let previous_mastery = self.recipe_mastery_brews(&recipe.id);
            let mastery_improved = if stable_brew {
                let mastery = self
                    .progression
                    .recipe_mastery
                    .entry(recipe.id.clone())
                    .or_insert(0);
                *mastery += 1;
                *mastery > previous_mastery
            } else {
                false
            };
            let current_mastery_stage = mastery_stage(self.recipe_mastery_brews(&recipe.id));
            let recipe_discovered = self.progression.known_recipes.insert(recipe.id.clone());
            if recipe_discovered {
                self.trigger_recipe_logged_feedback(outcome_text::recipe_logged(&recipe.name));
                self.runtime.status_text =
                    outcome_text::recipe_discovered(&recipe.name, resolution, current_mastery_stage);
            } else {
                self.runtime.status_text =
                    outcome_text::brewed(data, resolution, stable_brew, current_mastery_stage);
            }
            if mastery_improved {
                self.trigger_mastery_improved_feedback(outcome_text::mastery_improved(
                    &recipe.name,
                    current_mastery_stage,
                ));
            }
        } else {
            self.runtime.status_text = outcome_text::collapsed(data, resolution);
        }
    }
}
