use super::GameplayState;
use crate::data::{GameData, ItemCategory, PlanterStateEntry, StationDefinition};

impl GameplayState {
    pub(super) fn interact_with_planter(&mut self, data: &GameData, station: &StationDefinition) {
        let existing_state = self.progression.planter_states.get(&station.id).cloned();
        // A bed nobody has touched yet has no entry at all, and that counts as
        // empty. Reading the candidate off the *existing* entry meant the first
        // approach to a fresh bed always fell through to "you have no seed for
        // this" while the player was holding one, and only the second worked.
        let bed_is_empty = existing_state
            .as_ref()
            .map(|state| state.planted_item_id.is_empty())
            .unwrap_or(true);
        let candidate = bed_is_empty
            .then(|| self.planter_seed_choice(data, station))
            .flatten();
        let mutation_candidate = existing_state.as_ref().and_then(|state| {
            (!state.planted_item_id.is_empty()
                && !state.ready
                && state.mutation_formula_id.is_empty())
            .then(|| self.planter_mutation_candidate(data, &state.planted_item_id))
            .flatten()
        });
        let mut state = self
            .progression
            .planter_states
            .remove(&station.id)
            .unwrap_or(PlanterStateEntry {
                station_id: station.id.clone(),
                planted_item_id: String::new(),
                planted_day: self.world.day_index,
                ready: false,
                tended_day: 0,
                tended_days: 0,
                growth_days: 0,
                mutation_formula_id: String::new(),
                mutation_yield_bonus: 0,
                mutation_growth_bonus_days: 0,
                mutation_note: String::new(),
            });
        if state.ready && !state.planted_item_id.is_empty() {
            self.harvest_planter(data, station, &mut state);
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }
        if !state.planted_item_id.is_empty() {
            self.tend_or_report_planter(data, station, &mut state, mutation_candidate.as_ref());
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        }

        let Some(item_id) = candidate else {
            self.report_missing_planter_seed(data, station);
            self.progression
                .planter_states
                .insert(station.id.clone(), state);
            return;
        };
        self.plant_seed_in_planter(data, station, &mut state, item_id);
        self.progression
            .planter_states
            .insert(station.id.clone(), state);
    }

    pub(super) fn planter_seed_choice(
        &self,
        data: &GameData,
        station: &StationDefinition,
    ) -> Option<String> {
        self.inventory
            .iter()
            .find(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| planter_accepts(station, item, item_id))
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
    }
}

/// Whether this bed will take that seed. A bed that names its seeds means what
/// it says: the list is the rule, not a filter applied on top of a rarity floor.
/// Previously the floor won, so a common herb named in `planter_seed_ids` was
/// listed to the player as accepted and then silently refused.
pub(crate) fn planter_accepts(
    station: &StationDefinition,
    item: &crate::data::ItemDefinition,
    item_id: &str,
) -> bool {
    if item.category != ItemCategory::Ingredient {
        return false;
    }
    if station.planter_seed_ids.is_empty() {
        // An unspecialised bed still holds the old line: rare stock only.
        return item.rarity >= 2;
    }
    station.planter_seed_ids.iter().any(|seed| seed == item_id)
}

#[cfg(test)]
mod tests {
    use super::planter_accepts;
    use super::GameplayState;

    /// Tending used to be spent by the clock. `tend_or_report_planter` added a
    /// day of growth, and the midnight rollover then recomputed the same field
    /// from elapsed time alone and threw the visit away — so the beds ripened
    /// on a pure timer and turning up changed nothing past the current day.
    /// The two models are composed now: elapsed time is the floor, and each
    /// day tended is worth a day on top of it. This walks a real bed across a
    /// rollover to prove the visit survives.
    #[test]
    fn a_tended_bed_stays_ahead_of_one_left_alone() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let station = data
            .stations
            .iter()
            .find(|station| station.planter_harvest_days >= 3)
            .expect("some bed should take more than a couple of days");

        let mut state = GameplayState::new(&data);
        let seed = station
            .planter_seed_ids
            .first()
            .cloned()
            .expect("a specialised bed names its seeds");
        state.inventory.insert(seed.clone(), 1);
        state.interact_with_planter(&data, station);
        assert_eq!(
            state.progression.planter_states[&station.id].planted_item_id, seed,
            "the seed should have gone in"
        );

        // Tend on the day it was planted, then let midnight pass.
        state.interact_with_planter(&data, station);
        assert_eq!(state.progression.planter_states[&station.id].tended_days, 1);
        state.world.day_index += 1;
        state.advance_planters(&data);

        let tended = state.progression.planter_states[&station.id].growth_days;
        assert_eq!(
            tended, 2,
            "one day elapsed plus one day tended should survive the rollover"
        );

        // The same bed, same elapsed time, never visited.
        let mut untended = GameplayState::new(&data);
        untended.inventory.insert(seed, 1);
        untended.interact_with_planter(&data, station);
        untended.world.day_index += 1;
        untended.advance_planters(&data);
        assert_eq!(
            untended.progression.planter_states[&station.id].growth_days, 1,
            "an untended bed should still grow, just slower"
        );
        assert!(tended > untended.progression.planter_states[&station.id].growth_days);
    }

    /// A bed lists what it accepts, and that list is shown to the player. Any id
    /// on it that the bed would in fact refuse is a promise the game breaks the
    /// moment someone tries it.
    #[test]
    fn every_advertised_seed_is_one_the_bed_will_take() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut refused = Vec::new();

        for station in &data.stations {
            for seed_id in &station.planter_seed_ids {
                match data.item(seed_id) {
                    None => refused.push(format!("{} -> no item {seed_id}", station.id)),
                    Some(item) if !planter_accepts(station, item, seed_id) => refused.push(
                        format!("{} advertises {seed_id} and refuses it", station.id),
                    ),
                    Some(_) => {}
                }
            }
        }

        assert!(
            refused.is_empty(),
            "beds that lie about their seeds:\n{refused:#?}"
        );
    }

    /// Planting is a two-step trade: a seed in the bed, then a potion to steer
    /// what it becomes. A seed with no mutation formula can be grown but never
    /// steered, which is fine; a formula for a seed no bed accepts is content
    /// that can never be reached.
    #[test]
    fn every_mutation_formula_has_a_bed_that_grows_its_seed() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let unreachable = data
            .mutation_formulas
            .iter()
            .filter(|formula| {
                !data.stations.iter().any(|station| {
                    data.item(&formula.seed_item_id)
                        .map(|item| planter_accepts(station, item, &formula.seed_item_id))
                        .unwrap_or(false)
                })
            })
            .map(|formula| format!("{} seeds {}", formula.id, formula.seed_item_id))
            .collect::<Vec<_>>();

        assert!(
            unreachable.is_empty(),
            "mutation formulas whose seed no bed will take:\n{unreachable:#?}"
        );
    }
}
