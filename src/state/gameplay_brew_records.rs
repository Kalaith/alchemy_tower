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
            stable: resolution.is_stable(),
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

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::alchemy::resolve_brew;

    /// An overcharge collapse is the one failure the log used to misfile: it
    /// spelled the stability rule out itself and left `destabilized` off, so a
    /// brew that visibly collapsed into its unstable output was archived as
    /// clean. Potion memory is rebuilt from this log whenever a save loads with
    /// no memories, so the lie survived the session it was told in.
    #[test]
    fn an_overcharge_collapse_is_logged_as_the_failure_it_was() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        // Any formula that brews clean on spec will do — what matters is that
        // the *only* thing wrong with the overcharged version is the collapse.
        // Some recipes want a catalyst or a reagent order this test does not
        // set up, so ask the brewer which one it can make cleanly.
        let (station, recipe, selected) = data
            .recipes
            .iter()
            .filter_map(|recipe| {
                let station = data
                    .stations
                    .iter()
                    .find(|station| station.id == recipe.station_id)?;
                let selected = recipe
                    .ingredients
                    .iter()
                    .map(|ingredient| ingredient.item_id.clone())
                    .collect::<Vec<_>>();
                let on_spec = resolve_brew(
                    &data,
                    station,
                    &selected,
                    None,
                    recipe.required_heat,
                    recipe.required_stirs,
                    &recipe.required_timing,
                    0,
                );
                on_spec.is_stable().then_some((station, recipe, selected))
            })
            .next()
            .expect("some recipe should brew clean when run to its own spec");

        // Stirred far past the requirement: the numbers are better than on spec
        // and the process still matches, which is exactly the shape the old
        // rule read as stable.
        let resolution = resolve_brew(
            &data,
            station,
            &selected,
            None,
            recipe.required_heat,
            recipe.required_stirs + 20,
            &recipe.required_timing,
            0,
        );
        assert!(resolution.destabilized, "the setup should collapse");
        assert!(resolution.process_match, "the process itself was not wrong");
        assert!(resolution.minimum_quality_met && resolution.minimum_elements_met);
        assert!(!resolution.is_stable());

        state.record_brew_inventory_result(&data, &resolution, resolution.is_stable());
        let logged = state
            .progression
            .experiment_log
            .last()
            .expect("the brew should be logged");
        assert!(
            !logged.stable,
            "a collapse filed as a stable brew in the archive"
        );

        // What a save/load does: memories are dropped and rebuilt from the log.
        let logged_output = logged.output_item_id.clone();
        state.progression.potion_memories.clear();
        state.rebuild_memory_state(&data);
        assert_eq!(
            state
                .progression
                .potion_memories
                .get(&logged_output)
                .map(|memory| memory.successful_brews),
            Some(0),
            "reloading turned a collapse into a successful brew"
        );
    }
}
