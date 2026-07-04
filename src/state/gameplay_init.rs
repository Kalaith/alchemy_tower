use super::gameplay_alchemy_types::AlchemySession;
use super::gameplay_journal_support::initial_journal_milestones;
use super::gameplay_overlay_types::OverlayState;
use super::gameplay_progression_types::ProgressionState;
use super::gameplay_runtime_types::RuntimeState;
use super::gameplay_support::starting_day_time;
use super::gameplay_world_types::WorldState;
use super::GameplayState;
use crate::data::GameData;
use std::collections::BTreeMap;

impl GameplayState {
    pub(crate) fn new(data: &GameData) -> Self {
        let mut state = Self {
            world: WorldState::new(data, starting_day_time(data)),
            progression: ProgressionState::new(initial_journal_milestones()),
            coins: 24,
            vitality: 100.0,
            inventory: BTreeMap::new(),
            runtime: RuntimeState::new(data),
            ui: OverlayState::new_gameplay(),
            alchemy: AlchemySession::default(),
        };
        state.seed_starter_recipes(data);
        state.initialize_npc_motion_states(data);
        state.refresh_available_nodes(data);
        state
    }

    /// Reveal the recipes for stations that are usable from the very start so a
    /// new player can see how to brew the town's first potions instead of
    /// facing an empty formulae panel. Gated stations (greenhouse, rune bench,
    /// containment, etc.) stay discovery-only.
    fn seed_starter_recipes(&mut self, data: &GameData) {
        let starter_stations: std::collections::HashSet<&str> = data
            .stations
            .iter()
            .filter(|station| {
                station.area_id == data.config.starting_area
                    && station.required_completed_quest.is_empty()
                    && station.required_total_brews == 0
                    && station.required_journal_milestone.is_empty()
            })
            .map(|station| station.id.as_str())
            .collect();

        for recipe in &data.recipes {
            if starter_stations.contains(recipe.station_id.as_str()) {
                self.progression.known_recipes.insert(recipe.id.clone());
            }
        }
    }
}
