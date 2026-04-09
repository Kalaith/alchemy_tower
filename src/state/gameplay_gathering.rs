use super::gameplay_support::rgba;
use super::*;
use crate::content::ui_format;
use std::collections::BTreeSet;

impl GameplayState {
    pub(super) fn node_is_available(&self, node: &crate::data::GatherNodeDefinition) -> bool {
        self.world.available_nodes.contains(&node.id)
    }

    pub(super) fn item_has_field_notes(&self, item_id: &str) -> bool {
        self.progression
            .herb_memories
            .get(item_id)
            .map(|entry| entry.learned)
            .unwrap_or(false)
    }

    pub(super) fn herb_memory(&self, item_id: &str) -> Option<&HerbMemoryEntry> {
        self.progression.herb_memories.get(item_id)
    }

    pub(super) fn herb_memories<'a>(&'a self, data: &'a GameData) -> Vec<&'a HerbMemoryEntry> {
        let mut entries = self
            .progression
            .herb_memories
            .values()
            .filter(|entry| {
                data.item(&entry.item_id)
                    .map(|item| item.category == ItemCategory::Ingredient)
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| {
            right
                .learned
                .cmp(&left.learned)
                .then(right.best_quality.cmp(&left.best_quality))
                .then(data.item_name(&left.item_id).cmp(data.item_name(&right.item_id)))
        });
        entries
    }

    pub(super) fn herb_memory_state_key(&self, item_id: &str) -> &'static str {
        match self.herb_memory(item_id) {
            Some(entry) if entry.learned => "journal_memory_state_learned",
            Some(entry) if entry.seen => "journal_memory_state_seen",
            _ => "journal_memory_state_unseen",
        }
    }

    pub(super) fn gather_attempt_status(
        &self,
        _data: &GameData,
        node: &crate::data::GatherNodeDefinition,
    ) -> String {
        if self.item_has_field_notes(&node.item_id) {
            ui_format("gather_attempt_known", &[("name", &node.name)])
        } else {
            ui_format("gather_attempt_none", &[])
        }
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

    pub(super) fn record_field_discovery(
        &mut self,
        data: &GameData,
        node: &crate::data::GatherNodeDefinition,
    ) -> FieldDiscoveryFeedback {
        let (best_quality, variant_name) = self
            .current_item_quality_snapshot(data, node.item_id.as_str())
            .unwrap_or((0, String::new()));
        let previous = self.progression.herb_memories.get(&node.item_id).cloned();
        let variant_discovered = !variant_name.is_empty()
            && previous
                .as_ref()
                .map(|entry| entry.variant_name != variant_name)
                .unwrap_or(true);
        let improved_quality = previous
            .as_ref()
            .map(|entry| best_quality > entry.best_quality)
            .unwrap_or(true);
        let entry = self
            .progression
            .herb_memories
            .entry(node.item_id.clone())
            .or_insert_with(|| HerbMemoryEntry {
                item_id: node.item_id.clone(),
                first_seen_day: self.world.day_index,
                first_seen_route_id: node.route_id.clone(),
                seen: true,
                learned: false,
                learned_day: 0,
                learned_route_id: String::new(),
                note: String::new(),
                best_quality: 0,
                best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
                variant_name: String::new(),
            });
        if !entry.seen {
            entry.seen = true;
            entry.first_seen_day = self.world.day_index;
            entry.first_seen_route_id = node.route_id.clone();
        }
        entry.learned = true;
        entry.learned_day = self.world.day_index;
        entry.learned_route_id = node.route_id.clone();
        entry.note = node.note.clone();
        if best_quality >= entry.best_quality {
            entry.best_quality = best_quality;
            entry.best_quality_band = quality_band(best_quality).to_owned();
            entry.variant_name = variant_name;
        }
        FieldDiscoveryFeedback {
            new_note: previous.map(|entry| !entry.learned).unwrap_or(true),
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
            burst_scale: if emphasis { 1.35 } else { 1.0 },
        });

        if discovery.new_note {
            let route_name = data
                .route(&node.route_id)
                .map(|route| route.name.as_str())
                .unwrap_or("field notes");
            self.push_gather_toast(
                ui_format("gather_toast_journal", &[("route", route_name)]),
                Color::from_rgba(176, 226, 255, 255),
            );
        }
        if discovery.improved_quality {
            self.push_gather_toast(
                ui_format("gather_toast_quality", &[("name", &node.name)]),
                Color::from_rgba(255, 228, 150, 255),
            );
        }
        if discovery.variant_discovered {
            self.push_gather_toast(
                ui_format("gather_toast_variant", &[("name", &node.name)]),
                Color::from_rgba(188, 255, 220, 255),
            );
        }
        if emphasis {
            self.runtime.gather_pause_seconds = self.runtime.gather_pause_seconds.max(0.08);
            self.trigger_camera_shake(0.08, 2.4);
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
            ui_format("gather_status_variant", &[("name", &node.name), ("route", route_name)])
        } else if discovery.improved_quality {
            ui_format("gather_status_best", &[("name", &node.name), ("route", route_name)])
        } else {
            ui_format("gather_status_collected", &[("name", &node.name), ("route", route_name)])
        }
    }

    pub(super) fn ensure_herb_memory_seen(&mut self, item_id: &str, route_id: Option<&str>) {
        let entry = self
            .progression
            .herb_memories
            .entry(item_id.to_owned())
            .or_insert_with(|| HerbMemoryEntry {
                item_id: item_id.to_owned(),
                first_seen_day: self.world.day_index,
                first_seen_route_id: route_id.unwrap_or_default().to_owned(),
                seen: true,
                learned: false,
                learned_day: 0,
                learned_route_id: String::new(),
                note: String::new(),
                best_quality: 0,
                best_quality_band: ui_copy("inventory_best_unlogged").to_owned(),
                variant_name: String::new(),
            });
        if !entry.seen {
            entry.seen = true;
            entry.first_seen_day = self.world.day_index;
        }
        if entry.first_seen_route_id.is_empty() {
            entry.first_seen_route_id = route_id.unwrap_or_default().to_owned();
        }
    }
}

