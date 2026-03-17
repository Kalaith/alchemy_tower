use super::gameplay_support::rgba;
use super::*;

impl GameplayState {
    pub(super) fn node_is_available(&self, node: &crate::data::GatherNodeDefinition) -> bool {
        self.world.available_nodes.contains(&node.id)
    }

    pub(super) fn gather_unavailable_reason(
        &self,
        node: &crate::data::GatherNodeDefinition,
    ) -> String {
        let season_ok = node.seasons.is_empty()
            || node
                .seasons
                .iter()
                .any(|season| season == self.current_season());
        if !season_ok {
            return "Out of season.".to_owned();
        }

        let weather_ok = node.weathers.is_empty()
            || node
                .weathers
                .iter()
                .any(|weather| weather == self.current_weather());
        if !weather_ok {
            if node.weathers.iter().any(|weather| weather == "rain") {
                return "Requires rain.".to_owned();
            }
            return format!("Needs {} weather.", node.weathers.join(" or "));
        }

        let time_ok = node.time_windows.is_empty()
            || node
                .time_windows
                .iter()
                .any(|time| time == self.current_time_window());
        if !time_ok {
            if node.time_windows.iter().any(|time| time == "evening") {
                return "Only appears at dusk.".to_owned();
            }
            return format!("Appears during {}.", node.time_windows.join(" or "));
        }

        "No sign of it today.".to_owned()
    }

    pub(super) fn record_field_discovery(
        &mut self,
        data: &GameData,
        node: &crate::data::GatherNodeDefinition,
    ) -> FieldDiscoveryFeedback {
        let (best_quality, variant_name) = self
            .current_item_quality_snapshot(data, node.item_id.as_str())
            .unwrap_or((0, String::new()));
        let previous = self.progression.field_journal.get(&node.item_id).cloned();
        let variant_discovered = !variant_name.is_empty()
            && previous
                .as_ref()
                .map(|entry| entry.variant_name != variant_name)
                .unwrap_or(true);
        let improved_quality = previous
            .as_ref()
            .map(|entry| best_quality > entry.best_quality)
            .unwrap_or(true);
        let entry = FieldJournalEntry {
            item_id: node.item_id.clone(),
            route_id: node.route_id.clone(),
            season: self.current_season().to_owned(),
            weather: self.current_weather().to_owned(),
            time_window: self.current_time_window().to_owned(),
            note: node.note.clone(),
            best_quality,
            best_quality_band: quality_band(best_quality).to_owned(),
            variant_name,
        };
        self.progression.field_journal.insert(node.item_id.clone(), entry);
        FieldDiscoveryFeedback {
            new_note: previous.is_none(),
            improved_quality,
            variant_discovered,
        }
    }

    pub(super) fn trigger_gather_feedback(
        &mut self,
        data: &GameData,
        node: &crate::data::GatherNodeDefinition,
        discovery: &FieldDiscoveryFeedback,
    ) {
        let center = vec2(node.position[0], node.position[1]);
        let base_color = rgba(node.color);
        let emphasis = discovery.variant_discovered || discovery.improved_quality;
        let duration = if emphasis { 0.8 } else { 0.45 };
        self.runtime.gather_feedbacks.push(GatherFeedback {
            position: center,
            remaining_seconds: duration,
            color: base_color,
            emphasis,
        });

        if discovery.new_note {
            let route_name = data
                .route(&node.route_id)
                .map(|route| route.name.as_str())
                .unwrap_or("field notes");
            self.push_gather_toast(
                format!("Journal note added: {}.", route_name),
                Color::from_rgba(176, 226, 255, 255),
            );
        }
        if discovery.improved_quality {
            self.push_gather_toast(
                format!("Better than previous quality: {}.", node.name),
                Color::from_rgba(255, 228, 150, 255),
            );
        }
        if discovery.variant_discovered {
            self.push_gather_toast(
                format!("Variant discovered: {}.", node.name),
                Color::from_rgba(188, 255, 220, 255),
            );
        }
        if emphasis {
            self.runtime.gather_pause_seconds = self.runtime.gather_pause_seconds.max(0.08);
        }
    }

    pub(super) fn push_gather_toast(&mut self, text: String, color: Color) {
        self.push_event_toast(text, color);
    }

    pub(super) fn gather_status_text(
        &self,
        data: &GameData,
        node: &crate::data::GatherNodeDefinition,
        discovery: &FieldDiscoveryFeedback,
    ) -> String {
        let route_name = data
            .route(&node.route_id)
            .map(|route| route.name.as_str())
            .unwrap_or("unknown route");
        if discovery.variant_discovered {
            format!("Collected {}. Variant noted in {}.", node.name, route_name)
        } else if discovery.improved_quality {
            format!(
                "Collected {}. Best specimen updated for {}.",
                node.name, route_name
            )
        } else {
            format!("Collected {} from {}.", node.name, route_name)
        }
    }
}

