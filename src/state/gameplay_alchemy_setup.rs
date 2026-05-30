use super::{GameplayState, SavedAlchemySetup, ALCHEMY_TIMINGS};
use crate::content::ui_format;
use crate::data::{GameData, ItemCategory};
use std::collections::BTreeMap;

impl GameplayState {
    pub(super) fn alchemy_materials(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| {
                            item.category == ItemCategory::Ingredient
                                || item.category == ItemCategory::Catalyst
                        })
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, false);
        items
    }

    pub(super) fn reserved_count(&self, item_id: &str) -> u32 {
        self.alchemy
            .slots
            .iter()
            .filter(|slot| slot.as_deref() == Some(item_id))
            .count() as u32
            + u32::from(self.alchemy.catalyst.as_deref() == Some(item_id))
    }

    pub(super) fn fill_slot(&mut self, data: &GameData, items: &[String], slot: usize) {
        let Some(item_id) = items.get(self.alchemy.index) else {
            return;
        };
        let Some(item) = data.item(item_id) else {
            return;
        };
        if item.category != ItemCategory::Ingredient {
            self.runtime.status_text = self.unavailable_state_text(&format!(
                "{}",
                ui_format("inventory_fill_slot_catalyst", &[("name", &item.name)])
            ));
            return;
        }
        let total = self.inventory.get(item_id).copied().unwrap_or_default();
        let reserved = self.reserved_count(item_id)
            - u32::from(self.alchemy.slots[slot].as_deref() == Some(item_id));
        if total <= reserved {
            self.runtime.status_text = self.unavailable_state_text(&format!(
                "{}",
                ui_format(
                    "inventory_no_more_ready",
                    &[("name", data.item_name(item_id))]
                )
            ));
            return;
        }
        self.alchemy.slots[slot] = Some(item_id.clone());
        self.runtime.status_text = ui_format(
            "inventory_added_slot",
            &[
                ("item", data.item_name(item_id)),
                ("slot", &(slot + 1).to_string()),
            ],
        );
    }

    pub(super) fn fill_catalyst(&mut self, data: &GameData, items: &[String]) {
        let Some(item_id) = items.get(self.alchemy.index) else {
            return;
        };
        let Some(item) = data.item(item_id) else {
            return;
        };
        if item.category != ItemCategory::Catalyst {
            self.runtime.status_text = self.unavailable_state_text(&format!(
                "{}",
                ui_format("inventory_fill_catalyst_invalid", &[("name", &item.name)])
            ));
            return;
        }
        let total = self.inventory.get(item_id).copied().unwrap_or_default();
        let reserved = self.reserved_count(item_id)
            - u32::from(self.alchemy.catalyst.as_deref() == Some(item_id));
        if total <= reserved {
            self.runtime.status_text = self.unavailable_state_text(&ui_format(
                "inventory_no_more_ready",
                &[("name", &item.name)],
            ));
            return;
        }
        self.alchemy.catalyst = Some(item_id.clone());
        self.runtime.status_text =
            ui_format("inventory_prepared_catalyst", &[("name", &item.name)]);
    }

    pub(super) fn selected_items(&self) -> Vec<String> {
        self.alchemy
            .slots
            .iter()
            .filter_map(|item_id| item_id.clone())
            .collect()
    }

    pub(super) fn selected_catalyst(&self) -> Option<&str> {
        self.alchemy.catalyst.as_deref()
    }

    pub(super) fn alchemy_timing(&self) -> &'static str {
        ALCHEMY_TIMINGS[self.alchemy.timing_index]
    }

    pub(super) fn save_last_brew_setup(&mut self) {
        self.runtime.last_brew_setup = Some(SavedAlchemySetup {
            heat: self.alchemy.heat,
            stirs: self.alchemy.stirs,
            timing_index: self.alchemy.timing_index,
            slots: self.alchemy.slots.clone(),
            catalyst: self.alchemy.catalyst.clone(),
        });
    }

    pub(super) fn repeat_last_brew_setup(&mut self, data: &GameData) {
        let Some(setup) = self.runtime.last_brew_setup.clone() else {
            self.runtime.status_text = ui_format("alchemy_repeat_none", &[]);
            return;
        };

        let mut needed = BTreeMap::<String, u32>::new();
        for item_id in setup.slots.iter().flatten() {
            *needed.entry(item_id.clone()).or_insert(0) += 1;
        }
        if let Some(item_id) = &setup.catalyst {
            *needed.entry(item_id.clone()).or_insert(0) += 1;
        }

        for (item_id, required) in &needed {
            let available = self.inventory.get(item_id).copied().unwrap_or_default();
            if available < *required {
                self.runtime.status_text = self.unavailable_state_text(&ui_format(
                    "alchemy_repeat_missing",
                    &[
                        ("name", data.item_name(item_id)),
                        ("count", &required.to_string()),
                    ],
                ));
                return;
            }
        }

        self.alchemy.heat = setup.heat;
        self.alchemy.stirs = setup.stirs;
        self.alchemy.timing_index = setup
            .timing_index
            .min(ALCHEMY_TIMINGS.len().saturating_sub(1));
        self.alchemy.slots = setup.slots;
        self.alchemy.catalyst = setup.catalyst;
        self.runtime.status_text = ui_format("alchemy_repeat_loaded", &[]);
    }

    pub(super) fn current_item_quality_snapshot(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<(u32, String)> {
        let item = data.item(item_id)?;
        let mut quality = item.quality;
        let mut variant_name = String::new();
        for variant in &item.wild_variants {
            if variant
                .required_conditions
                .iter()
                .all(|condition| self.condition_matches(condition))
            {
                quality += variant.quality_bonus;
                variant_name = variant.name.clone();
                break;
            }
        }
        Some((quality.min(100), variant_name))
    }

    pub(super) fn condition_matches(&self, condition: &str) -> bool {
        let condition = condition.to_ascii_lowercase();
        condition.contains(self.current_season())
            || condition.contains(self.current_weather())
            || condition.contains(self.current_time_window())
    }
}
