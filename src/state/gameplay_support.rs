use super::GameplayState;
use crate::data::GameData;

#[path = "gameplay_support_text.rs"]
mod support_text;

impl GameplayState {
    pub(super) fn update_area_banner(&mut self, data: &GameData, frame_time: f32) {
        self.runtime.area_banner_seconds = (self.runtime.area_banner_seconds - frame_time).max(0.0);
        if self.runtime.area_banner_area_id != self.world.current_area_id {
            self.runtime.area_banner_area_id = self.world.current_area_id.clone();
            self.runtime.area_banner_label = data
                .area(&self.world.current_area_id)
                .map(|area| area.name.clone())
                .unwrap_or_default();
            self.runtime.area_banner_seconds = 2.6;
        }
    }
}

pub(super) fn quality_band_rank(band: &str) -> u8 {
    support_text::quality_band_rank(band)
}

pub(super) fn planter_stage_label(growth_days: u32, total_days: u32) -> &'static str {
    support_text::planter_stage_label(growth_days, total_days)
}

pub(super) fn starting_day_time(data: &GameData) -> f32 {
    data.config.day_length_seconds * 0.30
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::GameData;

    fn first_gather_node_id(data: &GameData) -> String {
        data.areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter())
            .map(|node| node.id.clone())
            .next()
            .expect("fallback data should include at least one gather node")
    }

    #[test]
    fn sleeping_after_midnight_does_not_refresh_same_day_nodes() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        let node_id = first_gather_node_id(&data);

        state.world.day_index = 3;
        state.set_clock_minutes(30.0);
        state.world.gathered_nodes.insert(node_id.clone());
        state.refresh_available_nodes(&data);

        state.sleep_until(&data, 7.0 * 60.0, false);

        assert_eq!(state.world.day_index, 3);
        assert!((state.current_clock_minutes() - 420.0).abs() < 0.01);
        assert!(state.world.gathered_nodes.contains(&node_id));
    }

    #[test]
    fn sleeping_late_advances_day_and_clears_gathered_nodes() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        let node_id = first_gather_node_id(&data);

        state.world.day_index = 3;
        state.set_clock_minutes(22.0 * 60.0);
        state.world.gathered_nodes.insert(node_id);

        state.sleep_until(&data, 7.0 * 60.0, false);

        assert_eq!(state.world.day_index, 4);
        assert!((state.current_clock_minutes() - 420.0).abs() < 0.01);
        assert!(state.world.gathered_nodes.is_empty());
    }
}
