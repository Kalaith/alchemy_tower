use super::GameplayState;
use crate::alchemy::resolve_brew;
use crate::audio::AudioAssets;
use crate::content::{narrative_text, ui_copy, ui_format};
use crate::data::{GameData, ItemCategory, StationDefinition};
use macroquad::prelude::{vec2, Color};

impl GameplayState {
    pub(super) fn brew_selected(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
        audio: &AudioAssets,
    ) {
        let selected = self.selected_items();
        if selected.is_empty() {
            self.runtime.status_text = narrative_text().statuses.cauldron_empty.clone();
            return;
        }
        self.save_last_brew_setup();
        let resolution = resolve_brew(
            data,
            station,
            &selected,
            self.selected_catalyst(),
            self.alchemy.heat,
            self.alchemy.stirs,
            self.alchemy_timing(),
            self.preview_mastery_brews(data, station, &selected),
        );
        let stable_brew = self.brew_is_stable(&resolution);
        let brew_feedback_color = if resolution.recipe.is_none() {
            Color::from_rgba(196, 162, 255, 255)
        } else if stable_brew {
            Color::from_rgba(188, 255, 220, 255)
        } else {
            Color::from_rgba(255, 214, 132, 255)
        };
        self.trigger_world_feedback(
            vec2(station.position[0], station.position[1]),
            brew_feedback_color,
            stable_brew || resolution.recipe.is_none(),
            if stable_brew { 1.9 } else { 1.4 },
        );
        self.trigger_camera_shake(
            if stable_brew { 0.12 } else { 0.08 },
            if stable_brew { 3.6 } else { 2.0 },
        );
        for item_id in &selected {
            if let Some(amount) = self.inventory.get_mut(item_id) {
                *amount -= 1;
            }
        }
        if let Some(item_id) = self.selected_catalyst().map(str::to_owned) {
            if let Some(amount) = self.inventory.get_mut(&item_id) {
                *amount -= 1;
            }
        }
        self.inventory.retain(|_, amount| *amount > 0);
        *self
            .inventory
            .entry(resolution.output_item_id.clone())
            .or_insert(0) += resolution.output_amount;
        self.note_inventory_observation(data, &resolution.output_item_id);
        self.progression.total_brews += 1;
        self.record_experiment_log(&resolution);
        if self.progression.total_brews == 1 {
            let milestone = &narrative_text().milestones.first_true_brew;
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
        let previous_profile = self
            .progression
            .crafted_item_profiles
            .get(&resolution.output_item_id)
            .cloned();
        self.record_crafted_item_profile(data, &resolution.output_item_id, &resolution);
        if data
            .item(&resolution.output_item_id)
            .map(|item| item.category == ItemCategory::Potion)
            .unwrap_or(false)
        {
            self.record_potion_result_memory(
                &resolution.output_item_id,
                resolution.recipe.map(|recipe| recipe.id.as_str()),
                stable_brew,
                resolution.quality_score,
                resolution.quality_band,
            );
        }
        if let Some(recipe) = resolution.recipe {
            let previous_mastery = self.recipe_mastery_brews(&recipe.id);
            let stable_brew = self.brew_is_stable(&resolution);
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
            let current_mastery_stage =
                crate::alchemy::mastery_stage(self.recipe_mastery_brews(&recipe.id));
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
        audio.play_brew_result(self.brew_is_stable(&resolution));
        let current_profile = self
            .progression
            .crafted_item_profiles
            .get(&resolution.output_item_id);
        let improved_best = current_profile
            .zip(previous_profile.as_ref())
            .map(|(current, previous)| current.best_quality_score > previous.best_quality_score)
            .unwrap_or(current_profile.is_some());
        if improved_best {
            if let Some(profile) = current_profile {
                self.push_event_toast_with_icon(
                    ui_format(
                        "inventory_new_best",
                        &[
                            ("item", data.item_name(&resolution.output_item_id)),
                            ("band", &profile.best_quality_band),
                        ],
                    ),
                    Color::from_rgba(188, 255, 220, 255),
                    "best_quality",
                );
            }
        }
        if self.progression.total_brews == 10 {
            self.push_event_toast_with_icon(
                &ui_format("inventory_greenhouse_unlock", &[]),
                Color::from_rgba(200, 255, 200, 255),
                "route_restored",
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(200, 255, 200, 255),
                true,
                2.1,
            );
            self.trigger_camera_shake(0.2, 5.4);
            self.runtime.status_text = narrative_text().statuses.greenhouse_unlock.clone();
        }
        self.alchemy.stirs = 0;
        self.alchemy.timing_index = 0;
        self.alchemy.slots = [None, None, None];
        self.alchemy.catalyst = None;
    }
}
