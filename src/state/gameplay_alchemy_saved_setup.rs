use super::gameplay_alchemy_types::{SavedAlchemySetup, ALCHEMY_TIMINGS};
use super::GameplayState;
use crate::data::GameData;
use std::collections::BTreeMap;

#[path = "gameplay_alchemy_saved_setup_text.rs"]
mod saved_setup_text;

impl GameplayState {
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
            self.runtime.status_text = saved_setup_text::repeat_none();
            return;
        };

        let needed = required_saved_setup_items(&setup);
        for (item_id, required) in &needed {
            let available = self.inventory.get(item_id).copied().unwrap_or_default();
            if available < *required {
                self.runtime.status_text = self.unavailable_state_text(
                    &saved_setup_text::repeat_missing(data, item_id, *required),
                );
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
        self.runtime.status_text = saved_setup_text::repeat_loaded();
    }
}

fn required_saved_setup_items(setup: &SavedAlchemySetup) -> BTreeMap<String, u32> {
    let mut needed = BTreeMap::<String, u32>::new();
    for item_id in setup.slots.iter().flatten() {
        *needed.entry(item_id.clone()).or_insert(0) += 1;
    }
    if let Some(item_id) = &setup.catalyst {
        *needed.entry(item_id.clone()).or_insert(0) += 1;
    }
    needed
}
