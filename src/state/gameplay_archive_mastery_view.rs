use super::GameplayState;
use crate::alchemy::mastery_stage;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::archive::{
    ArchiveMasteryDetailView, ArchiveMasteryRecipeEntry, ArchiveMasterySectionView,
};

impl GameplayState {
    pub(super) fn archive_mastery_section_view(
        &self,
        data: &GameData,
    ) -> ArchiveMasterySectionView {
        let recipes = self.mastery_recipes(data);
        if recipes.is_empty() {
            return ArchiveMasterySectionView {
                empty_text: self.unavailable_state_text(ui_copy("overlay_archive_empty_mastery")),
                entries: Vec::new(),
                detail: None,
            };
        }

        let selected_index = self.archive_selected_index(recipes.len());
        let entries = recipes
            .iter()
            .take(6)
            .enumerate()
            .map(|(index, recipe)| {
                let mastery = self.recipe_mastery_brews(&recipe.id);
                ArchiveMasteryRecipeEntry {
                    title: recipe.name.clone(),
                    detail: recipe.description.clone(),
                    meta: format!(
                        "{}  {}",
                        mastery_stage(mastery),
                        self.recipe_memory_meta(data, recipe)
                    ),
                    selected: index == selected_index,
                }
            })
            .collect();

        let recipe = recipes[selected_index];
        let mastery = self.recipe_mastery_brews(&recipe.id);
        let profile = self.progression.crafted_item_profiles.get(&recipe.output_item_id);
        let last_attempt_text = self
            .progression
            .experiment_log
            .iter()
            .rev()
            .find(|entry| entry.recipe_id == recipe.id)
            .map(|entry| {
                ui_format(
                    "overlay_archive_last_attempt",
                    &[
                        ("day", &(entry.day_index + 1).to_string()),
                        ("band", &entry.quality_band),
                    ],
                )
            });

        ArchiveMasterySectionView {
            empty_text: String::new(),
            entries,
            detail: Some(ArchiveMasteryDetailView {
                title: recipe.name.clone(),
                stage_text: ui_format(
                    "overlay_archive_mastery_stage",
                    &[
                        ("stage", mastery_stage(mastery)),
                        ("count", &mastery.to_string()),
                    ],
                ),
                best_result_text: profile.map(|profile| {
                    ui_format(
                        "overlay_archive_best_result",
                        &[
                            ("quality", &profile.best_quality_score.to_string()),
                            ("band", &profile.best_quality_band),
                        ],
                    )
                }),
                traits_text: profile.map(|profile| {
                    ui_format(
                        "overlay_archive_traits_carried",
                        &[(
                            "traits",
                            &if profile.inherited_traits.is_empty() {
                                ui_copy("overlay_archive_none").to_owned()
                            } else {
                                profile.inherited_traits.join(", ")
                            },
                        )],
                    )
                }),
                last_attempt_text,
                lore_note: recipe.lore_note.clone(),
            }),
        }
    }
}
