use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, ItemCategory};

impl GameplayState {
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

    pub(super) fn alchemy_slot_items(&self) -> Vec<Option<String>> {
        self.alchemy.slots.iter().cloned().collect()
    }
}
