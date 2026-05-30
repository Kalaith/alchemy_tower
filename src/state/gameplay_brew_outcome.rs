use super::GameplayState;
use crate::alchemy::{mastery_stage, BrewResolution};
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use macroquad::prelude::Color;

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
                self.push_event_toast_with_icon(
                    ui_format("inventory_recipe_logged", &[("name", &recipe.name)]),
                    Color::from_rgba(176, 226, 255, 255),
                    "recipe_logged",
                );
                self.runtime.status_text = ui_format(
                    "inventory_discovered_status",
                    &[
                        ("recipe", &recipe.name),
                        ("quality", resolution.quality_band),
                        ("mastery", current_mastery_stage),
                        ("traits", &resolution.inherited_traits.join(", ")),
                    ],
                );
            } else {
                self.runtime.status_text = ui_format(
                    "inventory_brewed_status",
                    &[
                        ("item", data.item_name(&resolution.output_item_id)),
                        ("amount", &resolution.output_amount.to_string()),
                        ("quality", resolution.quality_band),
                        (
                            "result",
                            if stable_brew {
                                ui_copy("inventory_brew_result_stable")
                            } else {
                                ui_copy("inventory_brew_result_imperfect")
                            },
                        ),
                        ("mastery", current_mastery_stage),
                    ],
                );
            }
            if mastery_improved {
                self.push_event_toast_with_icon(
                    ui_format(
                        "inventory_mastery_improved",
                        &[("name", &recipe.name), ("stage", current_mastery_stage)],
                    ),
                    Color::from_rgba(255, 230, 170, 255),
                    "best_quality",
                );
            }
        } else {
            self.runtime.status_text = ui_format(
                "inventory_collapse_status",
                &[
                    ("quality", resolution.quality_band),
                    ("item", data.item_name(&resolution.output_item_id)),
                    ("reasons", &resolution.failure_reasons.join(" ")),
                ],
            );
        }
    }
}
