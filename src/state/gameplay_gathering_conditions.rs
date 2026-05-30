use super::GameplayState;
use crate::content::ui_format;
use crate::data::GameData;
use std::collections::BTreeSet;

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

        let mut parts = Vec::new();
        if !seasons.is_empty() {
            parts.push(format!(
                "season {}",
                seasons.into_iter().collect::<Vec<_>>().join(" or ")
            ));
        }
        if !weathers.is_empty() {
            parts.push(format!(
                "weather {}",
                weathers.into_iter().collect::<Vec<_>>().join(" or ")
            ));
        }
        if !times.is_empty() {
            parts.push(format!(
                "time {}",
                times.into_iter().collect::<Vec<_>>().join(" or ")
            ));
        }
        if parts.is_empty() {
            Some(ui_format("gather_known_conditions_none", &[]))
        } else {
            Some(ui_format(
                "gather_known_conditions",
                &[("conditions", &parts.join("  |  "))],
            ))
        }
    }
}
