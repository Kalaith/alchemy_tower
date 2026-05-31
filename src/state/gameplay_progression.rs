use super::gameplay_overlay_types::{ArchiveExperimentFilter, ARCHIVE_TABS};
use super::GameplayState;
use crate::data::{ExperimentLogEntry, GameData, RecipeDefinition};

#[path = "gameplay_progression_text.rs"]
mod progression_text;

#[allow(dead_code)]
impl GameplayState {
    pub(super) fn archive_experiment_entries<'a>(
        &'a self,
        data: &'a GameData,
    ) -> Vec<&'a ExperimentLogEntry> {
        let _ = data;
        self.progression
            .experiment_log
            .iter()
            .rev()
            .filter(|entry| match self.ui.archive_experiment_filter {
                ArchiveExperimentFilter::All => true,
                ArchiveExperimentFilter::Stable => entry.stable,
                ArchiveExperimentFilter::Unstable => !entry.stable,
            })
            .collect()
    }

    pub(super) fn archive_experiment_filter_label(&self) -> &'static str {
        self.ui.archive_experiment_filter.label()
    }

    pub(super) fn cycle_archive_experiment_filter(&mut self) {
        self.ui.archive_experiment_filter = self.ui.archive_experiment_filter.next();
        self.ui.archive_index = 0;
        self.runtime.status_text =
            progression_text::archive_filter_status(self.archive_experiment_filter_label());
    }

    pub(super) fn mastery_recipes<'a>(
        &self,
        data: &'a GameData,
    ) -> Vec<&'a RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| self.progression.known_recipes.contains(&recipe.id))
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| {
            self.recipe_mastery_brews(&right.id)
                .cmp(&self.recipe_mastery_brews(&left.id))
                .then(left.name.cmp(&right.name))
        });
        recipes
    }

    pub(super) fn morph_recipes<'a>(
        &self,
        data: &'a GameData,
    ) -> Vec<&'a RecipeDefinition> {
        let mut recipes = data
            .recipes
            .iter()
            .filter(|recipe| !recipe.morph_targets.is_empty())
            .collect::<Vec<_>>();
        recipes.sort_by(|left, right| left.name.cmp(&right.name));
        recipes
    }

    pub(super) fn last_morph_output<'a>(
        &'a self,
        recipe_id: &str,
    ) -> Option<&'a ExperimentLogEntry> {
        self.progression
            .experiment_log
            .iter()
            .rev()
            .find(|entry| entry.recipe_id == recipe_id && !entry.morph_output_item_id.is_empty())
    }

    pub(super) fn morph_output_discovered(&self, item_id: &str) -> bool {
        self.progression.crafted_item_profiles.contains_key(item_id)
    }

    pub(super) fn archive_selection_len(&self, data: &GameData) -> usize {
        match ARCHIVE_TABS[self.ui.archive_tab] {
            "experiments" => self.archive_experiment_entries(data).len(),
            "mastery" => self.mastery_recipes(data).len(),
            "morphs" => self.morph_recipes(data).len(),
            "disassembly" => self.available_disassembly_recipes(data).len(),
            "duplication" => self.duplication_candidates(data).len(),
            _ => 0,
        }
    }
}
