use super::GameplayState;
use crate::content::narrative_text;
use crate::data::{GameData, HabitatStateEntry, StationDefinition};

#[path = "gameplay_habitat_text.rs"]
mod habitat_text;

impl GameplayState {
    pub(super) fn habitat_prompt_text(&self, station: &StationDefinition) -> String {
        self.progression
            .habitat_states
            .get(&station.id)
            .map(|habitat| {
                if habitat.creature_item_id.is_empty() {
                    self.interact_prompt_copy("world_prompt_habitat_place", &[])
                } else if self.world.day_index
                    >= habitat
                        .last_harvest_day
                        .saturating_add(station.habitat_harvest_days.max(1))
                {
                    self.interact_prompt_copy("world_prompt_habitat_harvest", &[])
                } else {
                    self.interact_prompt_copy("world_prompt_habitat_check", &[])
                }
            })
            .unwrap_or_else(|| self.interact_prompt_copy("world_prompt_habitat_place", &[]))
    }

    pub(super) fn interact_with_habitat(&mut self, data: &GameData, station: &StationDefinition) {
        let candidate = self
            .inventory
            .iter()
            .find(|(item_id, amount)| {
                **amount > 0
                    && station
                        .habitat_creature_ids
                        .iter()
                        .any(|creature_id| creature_id == *item_id)
            })
            .map(|(item_id, _)| item_id.clone());
        let state = self
            .progression
            .habitat_states
            .entry(station.id.clone())
            .or_insert(HabitatStateEntry {
                station_id: station.id.clone(),
                creature_item_id: String::new(),
                placed_day: self.world.day_index,
                last_harvest_day: self.world.day_index,
            });

        if state.creature_item_id.is_empty() {
            let Some(creature_id) = candidate else {
                self.runtime.status_text = habitat_text::accepts(data, station);
                return;
            };
            if let Some(amount) = self.inventory.get_mut(&creature_id) {
                *amount -= 1;
            }
            self.inventory.retain(|_, amount| *amount > 0);
            state.creature_item_id = creature_id.clone();
            state.placed_day = self.world.day_index;
            state.last_harvest_day = self.world.day_index;
            let milestone = &narrative_text().milestones.containment_started;
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            self.runtime.status_text = habitat_text::settled(data, station, &creature_id);
            return;
        }

        let ready_day = state
            .last_harvest_day
            .saturating_add(station.habitat_harvest_days.max(1));
        if self.world.day_index >= ready_day {
            let amount = 1 + u32::from(self.progression.total_brews >= 20);
            let output_item_id = station.habitat_output_item_id.clone();
            *self.inventory.entry(output_item_id.clone()).or_insert(0) += amount;
            state.last_harvest_day = self.world.day_index;
            let station_name = station.name.clone();
            self.runtime.status_text =
                habitat_text::collected(data, &station_name, &output_item_id, amount);
            self.note_inventory_observation(data, &output_item_id);
        } else {
            let days_left = ready_day.saturating_sub(self.world.day_index);
            self.runtime.status_text = habitat_text::waiting(
                data,
                &state.creature_item_id,
                &station.habitat_output_item_id,
                days_left,
            );
        }
    }
}
