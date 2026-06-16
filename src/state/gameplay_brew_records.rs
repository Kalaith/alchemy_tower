use super::GameplayState;
use crate::alchemy::BrewResolution;
use crate::data::{CraftedItemProfileEntry, ExperimentLogEntry, GameData};

impl GameplayState {
    pub(super) fn record_crafted_item_profile(
        &mut self,
        data: &GameData,
        item_id: &str,
        resolution: &BrewResolution<'_>,
    ) {
        let effect_kinds: Vec<_> = data
            .item(item_id)
            .map(|item| {
                item.effects
                    .iter()
                    .map(|effect| effect.kind.to_string())
                    .collect()
            })
            .unwrap_or_default();
        let entry = self
            .progression
            .crafted_item_profiles
            .entry(item_id.to_owned())
            .or_insert_with(|| CraftedItemProfileEntry {
                item_id: item_id.to_owned(),
                best_quality_score: 0,
                best_quality_band: "Crude".to_owned(),
                inherited_traits: Vec::new(),
                effect_kinds: effect_kinds.clone(),
            });
        if resolution.quality_score >= entry.best_quality_score {
            entry.best_quality_score = resolution.quality_score;
            entry.best_quality_band = resolution.quality_band.to_owned();
            entry.inherited_traits = resolution.inherited_traits.clone();
        }
        if entry.effect_kinds.is_empty() {
            entry.effect_kinds = effect_kinds;
        }
    }

    pub(super) fn record_experiment_log(&mut self, resolution: &BrewResolution<'_>) {
        self.progression.experiment_log.push(ExperimentLogEntry {
            recipe_id: resolution
                .recipe
                .map(|recipe| recipe.id.clone())
                .unwrap_or_default(),
            output_item_id: resolution.output_item_id.clone(),
            quality_score: resolution.quality_score,
            quality_band: resolution.quality_band.to_owned(),
            stable: resolution.process_match
                && resolution.minimum_quality_met
                && resolution.minimum_elements_met,
            catalyst_item_id: self.selected_catalyst().unwrap_or_default().to_owned(),
            morph_output_item_id: resolution.morph_output_item_id.clone().unwrap_or_default(),
            day_index: self.world.day_index,
        });
        if self.progression.experiment_log.len() > 60 {
            let excess = self.progression.experiment_log.len() - 60;
            self.progression.experiment_log.drain(0..excess);
        }
    }
}
