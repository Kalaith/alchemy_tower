use super::GameplayState;
use crate::data::GameData;
use std::collections::BTreeSet;

#[path = "gameplay_gathering_conditions_text.rs"]
mod conditions_text;

impl GameplayState {
    pub(super) fn learned_gathering_conditions(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<String> {
        if !self.item_has_field_notes(item_id) {
            return None;
        }

        let mut seasons = BTreeSet::new();
        let mut weathers = BTreeSet::new();
        let mut times = BTreeSet::new();
        let mut found = false;
        for node in data
            .areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter())
            .filter(|node| node.item_id == item_id)
        {
            found = true;
            seasons.extend(node.seasons.iter().cloned());
            weathers.extend(node.weathers.iter().cloned());
            times.extend(node.time_windows.iter().cloned());
        }
        if !found {
            return None;
        }

        Some(conditions_text::known_conditions(seasons, weathers, times))
    }
}
