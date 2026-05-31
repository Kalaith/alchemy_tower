use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{JournalBrewMemoryView, JournalBrewsTabView};

impl GameplayState {
    pub(super) fn journal_brews_tab_view(&self, data: &GameData) -> JournalBrewsTabView {
        let potion_memories = self.potion_memories(data);
        if potion_memories.is_empty() {
            return JournalBrewsTabView {
                title: ui_copy("overlay_potion_memories"),
                empty_text: ui_copy("journal_memory_no_potions").to_owned(),
                entries: Vec::new(),
            };
        }

        JournalBrewsTabView {
            title: ui_copy("overlay_potion_memories"),
            empty_text: String::new(),
            entries: potion_memories
                .into_iter()
                .map(|entry| {
                    let profile = self.journal_potion_profile_summary(&entry.item_id);
                    let formula_text = if entry.last_recipe_id.is_empty() {
                        None
                    } else {
                        let recipe_name = data
                            .recipes
                            .iter()
                            .find(|recipe| recipe.id == entry.last_recipe_id)
                            .map(|recipe| recipe.name.as_str())
                            .unwrap_or(entry.last_recipe_id.as_str());
                        Some(ui_format("journal_memory_formula", &[("formula", recipe_name)]))
                    };

                    JournalBrewMemoryView {
                        title: data.item_name(&entry.item_id).to_owned(),
                        state_line: self.journal_potion_state_line(entry),
                        recap: self.journal_potion_recap(data, &entry.item_id),
                        effects_text: profile.as_ref().map(|profile| {
                            ui_format(
                                "journal_memory_effects",
                                &[("effects", &profile.effects_text)],
                            )
                        }),
                        traits_text: profile.and_then(|profile| {
                            profile.traits_text.map(|traits_text| {
                                ui_format("inventory_memory_traits", &[("traits", &traits_text)])
                            })
                        }),
                        best_brew_text: (entry.best_quality_score > 0).then(|| {
                            ui_format(
                                "journal_memory_best_brew",
                                &[
                                    ("quality", &entry.best_quality_score.to_string()),
                                    ("band", &entry.best_quality_band),
                                ],
                            )
                        }),
                        formula_text,
                        successful_brews_text: (entry.successful_brews > 0).then(|| {
                            ui_format(
                                "journal_memory_successful_brews",
                                &[("count", &entry.successful_brews.to_string())],
                            )
                        }),
                    }
                })
                .collect(),
        }
    }
}
