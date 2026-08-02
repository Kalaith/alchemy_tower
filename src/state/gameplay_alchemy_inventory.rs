use super::GameplayState;
use crate::alchemy::resolve_brew;
use crate::audio::AudioAssets;
use crate::content::narrative_text;
use crate::data::{GameData, StationDefinition};

#[path = "gameplay_alchemy_inventory_text.rs"]
mod alchemy_inventory_text;

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
        let ingredients = self.brew_ingredients(data, &selected);
        let resolution = resolve_brew(
            data,
            station,
            &selected,
            &ingredients,
            self.selected_catalyst(),
            self.alchemy.heat,
            self.alchemy.stirs,
            self.alchemy_timing(),
            self.preview_mastery_brews(data, station, &selected),
        );
        let stable_brew = resolution.is_stable();
        self.trigger_brew_result_feedback(
            station.position,
            stable_brew,
            resolution.recipe.is_none(),
        );
        // The variant units go into the pot with everything else.
        self.spend_brew_variants(data, &selected);
        self.consume_brew_inputs(&selected);
        let previous_profile = self.record_brew_inventory_result(data, &resolution, stable_brew);
        // A mixture nobody wrote down, made enough times to stop being an
        // accident, is a formula the player found on their own.
        let found_formula = resolution.recipe.is_none()
            && self.record_salvage_attempt(data, station, &selected, &resolution);
        self.update_brew_result_status(data, &resolution, stable_brew);
        if found_formula {
            self.trigger_quest_complete_feedback(narrative_text().statuses.found_formula.clone());
        }
        audio.play_brew_result(stable_brew);
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
                self.trigger_new_best_brew_feedback(alchemy_inventory_text::new_best_brew(
                    data,
                    &resolution.output_item_id,
                    &profile.best_quality_band,
                ));
            }
        }
        if self.progression.total_brews == 10 {
            self.trigger_greenhouse_unlock_feedback(
                alchemy_inventory_text::greenhouse_unlock_toast(),
            );
            self.runtime.status_text = alchemy_inventory_text::greenhouse_unlock_status();
        }
        self.alchemy.stirs = 0;
        self.alchemy.timing_index = 0;
        self.alchemy.slots = [None, None, None];
        self.alchemy.catalyst = None;
    }
}
