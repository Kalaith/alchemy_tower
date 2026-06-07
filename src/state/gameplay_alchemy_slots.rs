use super::GameplayState;
use crate::data::{GameData, ItemCategory};

#[path = "gameplay_alchemy_slot_text.rs"]
mod alchemy_slot_text;

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
            self.runtime.status_text = self.unavailable_state_text(
                &alchemy_slot_text::fill_slot_requires_ingredient(&item.name),
            );
            return;
        }
        let total = self.inventory.get(item_id).copied().unwrap_or_default();
        let reserved = self.reserved_count(item_id)
            - u32::from(self.alchemy.slots[slot].as_deref() == Some(item_id));
        if total <= reserved {
            self.runtime.status_text =
                self.unavailable_state_text(&alchemy_slot_text::no_more_ready(data, item_id));
            return;
        }
        self.alchemy.slots[slot] = Some(item_id.clone());
        self.runtime.status_text = alchemy_slot_text::added_slot(data, item_id, slot);
    }

    pub(super) fn fill_catalyst(&mut self, data: &GameData, items: &[String]) {
        let Some(item_id) = items.get(self.alchemy.index) else {
            return;
        };
        let Some(item) = data.item(item_id) else {
            return;
        };
        if item.category != ItemCategory::Catalyst {
            self.runtime.status_text =
                self.unavailable_state_text(&alchemy_slot_text::fill_catalyst_invalid(&item.name));
            return;
        }
        let total = self.inventory.get(item_id).copied().unwrap_or_default();
        let reserved = self.reserved_count(item_id)
            - u32::from(self.alchemy.catalyst.as_deref() == Some(item_id));
        if total <= reserved {
            self.runtime.status_text =
                self.unavailable_state_text(&alchemy_slot_text::no_more_ready_name(&item.name));
            return;
        }
        self.alchemy.catalyst = Some(item_id.clone());
        self.runtime.status_text = alchemy_slot_text::prepared_catalyst(&item.name);
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
