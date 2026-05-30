use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, RecipeDefinition};

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
            .map(|profile| profile.best_quality_band.clone())
            .unwrap_or_else(|| ui_format("inventory_best_unlogged", &[]));
        let catalyst = if recipe.catalyst_tag.is_empty() {
            ui_format("inventory_catalyst_any", &[])
        } else {
            ui_format(
                "inventory_catalyst_specific",
                &[("tag", &recipe.catalyst_tag)],
            )
        };
        ui_format(
            "inventory_memory_meta",
            &[
                ("mastery", &mastery.to_string()),
                ("best", &best),
                ("catalyst", &catalyst),
            ],
        )
    }

    pub(super) fn recipe_memory_detail(
        &self,
        data: &GameData,
        recipe: &RecipeDefinition,
    ) -> String {
        let mut parts = vec![ui_format(
            "inventory_memory_output",
            &[("item", data.item_name(&recipe.output_item_id))],
        )];
        if !recipe.required_sequence.is_empty() {
            let sequence = recipe
                .required_sequence
                .iter()
                .map(|item_id| data.item_name(item_id))
                .collect::<Vec<_>>()
                .join(" -> ");
            parts.push(ui_format(
                "inventory_memory_order",
                &[("sequence", &sequence)],
            ));
        }
        if let Some(profile) = self
            .progression
            .crafted_item_profiles
            .get(&recipe.output_item_id)
        {
            if !profile.inherited_traits.is_empty() {
                parts.push(ui_format(
                    "inventory_memory_traits",
                    &[("traits", &profile.inherited_traits.join(", "))],
                ));
            }
        }
        if !recipe.morph_targets.is_empty() {
            parts.push(ui_format("inventory_memory_morph", &[]));
        }
        parts.join("  ")
    }
}
