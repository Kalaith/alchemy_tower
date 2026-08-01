use super::GameplayState;
use crate::data::{GameData, ItemCategory, PlanterStateEntry, StationDefinition};

impl GameplayState {
    pub(super) fn interact_with_planter(&mut self, data: &GameData, station: &StationDefinition) {
        let existing_state = self.progression.planter_states.get(&station.id).cloned();
        let candidate = existing_state
            .as_ref()
            .filter(|state| state.planted_item_id.is_empty())
            .and_then(|_| self.planter_seed_choice(data, station));
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
