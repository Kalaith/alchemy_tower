use super::GameplayState;
use crate::content::{narrative_text, ui_format};
use crate::data::{GameData, HabitatStateEntry, StationDefinition};

impl GameplayState {
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
                self.runtime.status_text = ui_format(
                    "gameplay_habitat_accepts",
                    &[
                        ("station", &station.name),
                        (
                            "items",
                            &station
                                .habitat_creature_ids
                                .iter()
                                .map(|item_id| data.item_name(item_id))
                                .collect::<Vec<_>>()
                                .join(", "),
                        ),
                    ],
                );
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
            self.runtime.status_text = ui_format(
                "gameplay_habitat_settled",
                &[
                    ("item", data.item_name(&creature_id)),
                    ("station", &station.name),
                ],
            );
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
            self.runtime.status_text = ui_format(
                "gameplay_habitat_collected",
                &[
                    ("item", data.item_name(&output_item_id)),
                    ("amount", &amount.to_string()),
                    ("station", &station_name),
                ],
            );
            self.note_inventory_observation(data, &output_item_id);
        } else {
            let days_left = ready_day.saturating_sub(self.world.day_index);
            self.runtime.status_text = ui_format(
                "gameplay_habitat_waiting",
                &[
                    ("creature", data.item_name(&state.creature_item_id)),
                    ("days", &days_left.to_string()),
                    ("output", data.item_name(&station.habitat_output_item_id)),
                ],
            );
        }
    }
}
