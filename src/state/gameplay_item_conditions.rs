use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
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
