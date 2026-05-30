use super::GameplayState;
use crate::content::{ui_copy, ui_copy_optional, ui_format};
use crate::data::{GameData, PotionMemoryEntry};
use crate::ui::draw_wrapped_text;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_journal_brews_tab(&self, data: &GameData, x: f32, y: f32, _w: f32, h: f32) {
        draw_text(
            ui_copy("overlay_potion_memories"),
            x + 20.0,
            y + 136.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut brew_y = y + 168.0;
        let potion_memories = self.potion_memories(data);
        if potion_memories.is_empty() {
            draw_text(
                ui_copy("journal_memory_no_potions"),
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_DIM,
            );
            return;
        }
        for entry in potion_memories {
            draw_text(
                data.item_name(&entry.item_id),
                x + 20.0,
                brew_y,
                20.0,
                dark::TEXT_BRIGHT,
            );
            brew_y += 20.0;
            draw_text(
                &self.journal_potion_state_line(entry),
                x + 20.0,
                brew_y,
                17.0,
                dark::TEXT_DIM,
            );
            brew_y += 18.0;
            draw_wrapped_text(
                &self.journal_potion_recap(data, &entry.item_id),
                x + 20.0,
                brew_y,
                520.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            if let Some(profile) = self.progression.crafted_item_profiles.get(&entry.item_id) {
                let effects = if profile.effect_kinds.is_empty() {
                    ui_copy("journal_memory_effects_none").to_owned()
                } else {
                    profile.effect_kinds.join(", ")
                };
                draw_text(
                    &ui_format("journal_memory_effects", &[("effects", &effects)]),
                    x + 580.0,
                    brew_y - 2.0,
                    18.0,
                    dark::TEXT_DIM,
                );
                if !profile.inherited_traits.is_empty() {
                    draw_text(
                        &ui_format(
                            "inventory_memory_traits",
                            &[("traits", &profile.inherited_traits.join(", "))],
                        ),
                        x + 580.0,
                        brew_y + 20.0,
                        18.0,
                        dark::TEXT_DIM,
                    );
                }
            }
            brew_y += 40.0;
            if entry.best_quality_score > 0 {
                draw_text(
                    &ui_format(
                        "journal_memory_best_brew",
                        &[
                            ("quality", &entry.best_quality_score.to_string()),
                            ("band", &entry.best_quality_band),
                        ],
                    ),
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if !entry.last_recipe_id.is_empty() {
                let recipe_name = data
                    .recipes
                    .iter()
                    .find(|recipe| recipe.id == entry.last_recipe_id)
                    .map(|recipe| recipe.name.as_str())
                    .unwrap_or(entry.last_recipe_id.as_str());
                draw_text(
                    &ui_format("journal_memory_formula", &[("formula", recipe_name)]),
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if entry.successful_brews > 0 {
                draw_text(
                    &ui_format(
                        "journal_memory_successful_brews",
                        &[("count", &entry.successful_brews.to_string())],
                    ),
                    x + 20.0,
                    brew_y,
                    18.0,
                    dark::TEXT_DIM,
                );
                brew_y += 22.0;
            }
            if brew_y > y + h - 40.0 {
                break;
            }
        }
    }

    pub(super) fn journal_herb_summary(&self, data: &GameData, item_id: &str) -> String {
        let key = format!("journal_herb_summary_{item_id}");
        ui_copy_optional(&key)
            .map(str::to_owned)
            .or_else(|| data.item(item_id).map(|item| item.description.clone()))
            .unwrap_or_else(|| data.item_name(item_id).to_owned())
    }

    pub(super) fn journal_potion_recap(&self, data: &GameData, item_id: &str) -> String {
        let key = format!("journal_potion_recap_{item_id}");
        ui_copy_optional(&key)
            .map(str::to_owned)
            .or_else(|| data.item(item_id).map(|item| item.description.clone()))
            .unwrap_or_else(|| data.item_name(item_id).to_owned())
    }

    pub(super) fn journal_potion_state_line(&self, entry: &PotionMemoryEntry) -> String {
        let mut parts = Vec::new();
        if entry.seen {
            parts.push(ui_copy("journal_memory_state_seen").to_owned());
        }
        if entry.learned {
            parts.push(ui_copy("journal_memory_state_learned").to_owned());
        }
        if entry.successful_brews > 0 {
            parts.push(ui_copy("journal_memory_state_brewed").to_owned());
        }
        ui_format(
            "journal_memory_state_line",
            &[("state", &parts.join("  |  "))],
        )
    }

}
