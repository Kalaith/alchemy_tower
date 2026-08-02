use super::GameplayState;
use crate::data::GameData;
use std::collections::BTreeSet;

#[path = "gameplay_gathering_conditions_text.rs"]
mod conditions_text;

impl GameplayState {
    /// What the player has *heard* about where a herb comes from, before they
    /// have worked its conditions out at a node.
    ///
    /// `source_conditions` is 61 authored strings — "rain and mist, when the
    /// floor takes up damp faster than the wards dry it", "the shelf below the
    /// binding, never the binding" — and nothing read a single one of them. It
    /// is the same information the learned line gives, in the valley's own
    /// voice and one step vaguer, which is exactly what a half-known herb
    /// should say: enough to know when to go looking, not enough to save the
    /// trip.
    pub(super) fn heard_gathering_conditions(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<String> {
        let item = data.item(item_id)?;
        if item.source_conditions.is_empty() {
            return None;
        }
        Some(conditions_text::heard_conditions(&item.source_conditions))
    }

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
