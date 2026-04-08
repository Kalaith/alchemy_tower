use super::*;
use crate::content::ui_format;

pub(super) struct NpcDialogueSelection<'a> {
    pub(super) start: &'a str,
    pub(super) progress: &'a str,
    pub(super) complete: &'a str,
}

impl GameplayState {
    pub(super) fn phase1_town_recovery_reached(&self) -> bool {
        self.has_journal_milestone("greenhouse_repaired")
            || self.progression.completed_quests.contains("cultivation_for_brin")
    }

    pub(super) fn phase1_first_relief_reached(&self) -> bool {
        self.has_journal_milestone("first_town_relief")
            || self.progression.completed_quests.contains("healing_for_mira")
    }

    pub(super) fn phase1_first_brew_reached(&self) -> bool {
        self.has_journal_milestone("first_true_brew") || self.progression.total_brews > 0
    }

    pub(super) fn npc_dialogue_selection<'a>(&'a self, data: &GameData, npc: &'a NpcDefinition) -> NpcDialogueSelection<'a> {
        let mut selection = NpcDialogueSelection {
            start: npc.dialogue_start.as_str(),
            progress: npc.dialogue_progress.as_str(),
            complete: npc.dialogue_complete.as_str(),
        };

        if npc.id == "crow_guide" {
            let crow = &npc.crow_phase1_dialogue;
            if !crow.first_meeting.is_empty() {
                let line = if self.phase1_town_recovery_reached() && !crow.first_tower_restoration.is_empty() {
                    crow.first_tower_restoration.as_str()
                } else if self.phase1_first_relief_reached() && !crow.first_quest_complete.is_empty() {
                    crow.first_quest_complete.as_str()
                } else if self.phase1_first_brew_reached() && !crow.first_brew.is_empty() {
                    crow.first_brew.as_str()
                } else {
                    crow.first_meeting.as_str()
                };
                selection.start = line;
                selection.progress = line;
                selection.complete = line;
            }
            return selection;
        }

        let phase1 = &npc.phase1_dialogue;
        let quest = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten();
        let quest_started = quest
            .map(|quest| self.progression.started_quests.contains(&quest.id))
            .unwrap_or(false);
        let quest_completed = quest
            .map(|quest| self.progression.completed_quests.contains(&quest.id))
            .unwrap_or(false);
        let quest_available = quest.map(|quest| self.quest_is_available(quest)).unwrap_or(false);

        if self.phase1_town_recovery_reached() && !phase1.town_recovery_observation.is_empty() {
            selection.start = phase1.town_recovery_observation.as_str();
            selection.progress = phase1.town_recovery_observation.as_str();
            selection.complete = phase1.town_recovery_observation.as_str();
            return selection;
        }
        if quest_completed && !phase1.post_help_relief.is_empty() {
            selection.complete = phase1.post_help_relief.as_str();
        }
        if (quest_started || quest_available) && !phase1.active_request.is_empty() {
            selection.start = phase1.active_request.as_str();
            selection.progress = phase1.active_request.as_str();
        } else if self.phase1_first_brew_reached() && !phase1.pre_help_concern.is_empty() {
            selection.start = phase1.pre_help_concern.as_str();
            selection.progress = phase1.pre_help_concern.as_str();
        } else if !phase1.intro.is_empty() {
            selection.start = phase1.intro.as_str();
            selection.progress = phase1.intro.as_str();
        }

        selection
    }

    pub(super) fn npc_phase1_followup_line(&self, npc_id: &str) -> Option<&str> {
        let phase1 = &narrative_text().phase1;
        if self.phase1_town_recovery_reached() {
            return match npc_id {
                "crow_guide" => Some(phase1.crow_after_greenhouse.as_str()),
                "mayor_elric" => Some(phase1.elric_after_greenhouse.as_str()),
                "mira_apothecary" => Some(phase1.mira_after_greenhouse.as_str()),
                "rowan_herbalist" => Some(phase1.rowan_after_greenhouse.as_str()),
                _ => None,
            };
        }
        if self.progression.completed_quests.contains("glow_for_rowan") {
            return match npc_id {
                "crow_guide" => Some(phase1.crow_after_glow.as_str()),
                "mayor_elric" => Some(phase1.elric_after_glow.as_str()),
                "ione_archivist" => Some(phase1.ione_after_glow.as_str()),
                _ => None,
            };
        }
        if self.progression.completed_quests.contains("healing_for_mira") {
            return match npc_id {
                "crow_guide" => Some(phase1.crow_after_healing.as_str()),
                "mayor_elric" => Some(phase1.elric_after_healing.as_str()),
                "brin_groundskeeper" => Some(phase1.brin_after_healing.as_str()),
                _ => None,
            };
        }
        if npc_id == "crow_guide" {
            return Some(phase1.crow_default.as_str());
        }
        None
    }

    pub(super) fn append_npc_story_line(&self, npc_id: &str, base: String) -> String {
        if let Some(extra) = self.npc_phase1_followup_line(npc_id) {
            if base.contains(extra) {
                base
            } else {
                format!("{base} {extra}")
            }
        } else {
            base
        }
    }

    pub(super) fn initialize_npc_motion_states(&mut self, data: &GameData) {
        self.runtime.npc_motion_states.clear();
        let walk_speed = (data.config.move_speed * 0.16).max(24.0);
        for npc in &data.npcs {
            let tracker = self.initial_npc_motion_tracker(data, npc, walk_speed);
            self.runtime.npc_motion_states.insert(npc.id.clone(), tracker);
        }
    }

    pub(super) fn update_npc_motion(&mut self, data: &GameData, frame_time: f32) {
        let walk_speed = (data.config.move_speed * 0.16).max(24.0);
        for npc in &data.npcs {
            let Some(current_index) = self.active_schedule_index(npc) else {
                continue;
            };
            let next_index = (current_index + 1) % npc.schedule.len();
            let mut tracker = self
                .runtime
                .npc_motion_states
                .remove(&npc.id)
                .unwrap_or_else(|| self.initial_npc_motion_tracker(data, npc, walk_speed));

            let (elapsed_seconds, interval_seconds) =
                self.schedule_window_progress_seconds(data, npc, current_index);
            let current_entry = &npc.schedule[current_index];
            let next_entry = &npc.schedule[next_index];
            let current_position = vec2(current_entry.position[0], current_entry.position[1]);
            let next_position = vec2(next_entry.position[0], next_entry.position[1]);

            if tracker.target_schedule_index.is_none() {
                let anchored_to_current = tracker.schedule_index == Some(current_index);
                let already_at_next = tracker.schedule_index == Some(next_index);
                if !anchored_to_current && !already_at_next {
                    let recovery_route = self
                        .travel_segments(
                            data,
                            &tracker.area_id,
                            tracker.position,
                            &current_entry.area_id,
                            current_position,
                        )
                        .unwrap_or_default();
                    if recovery_route.is_empty() {
                        tracker.area_id = current_entry.area_id.clone();
                        tracker.position = current_position;
                        tracker.direction = Vec2::ZERO;
                        tracker.moving = false;
                        tracker.target_area_id = None;
                        tracker.schedule_index = Some(current_index);
                    } else {
                        tracker.target_area_id = Some(current_entry.area_id.clone());
                        tracker.target_schedule_index = Some(current_index);
                        tracker.route_segments = recovery_route;
                        tracker.route_segment_index = 0;
                        tracker.moving = true;
                    }
                } else {
                    let route_from_anchor = self
                        .travel_segments(
                            data,
                            &current_entry.area_id,
                            current_position,
                            &next_entry.area_id,
                            next_position,
                        )
                        .unwrap_or_default();
                    let travel_duration =
                        self.npc_travel_duration_seconds(&route_from_anchor, walk_speed);
                    let departure_seconds = (interval_seconds - travel_duration).max(0.0);

                    if anchored_to_current && elapsed_seconds >= departure_seconds {
                        let route = self
                            .travel_segments(
                                data,
                                &tracker.area_id,
                                tracker.position,
                                &next_entry.area_id,
                                next_position,
                            )
                            .unwrap_or_default();
                        if route.is_empty() {
                            tracker.area_id = next_entry.area_id.clone();
                            tracker.position = next_position;
                            tracker.direction = Vec2::ZERO;
                            tracker.moving = false;
                            tracker.target_area_id = None;
                            tracker.schedule_index = Some(next_index);
                        } else {
                            tracker.target_area_id = Some(next_entry.area_id.clone());
                            tracker.target_schedule_index = Some(next_index);
                            tracker.route_segments = route;
                            tracker.route_segment_index = 0;
                            tracker.moving = true;
                        }
                    }
                }
            }

            self.advance_npc_tracker(&mut tracker, walk_speed, frame_time);
            self.runtime.npc_motion_states.insert(npc.id.clone(), tracker);
        }
    }

    pub(super) fn active_quest_location_hint(&self, data: &GameData) -> Option<String> {
        self.progression.started_quests.iter().find_map(|quest_id| {
            let quest = data.quest(quest_id)?;
            let npc = data.npc(&quest.giver_npc_id)?;
            Some(format!(
                "Now {}  |  Later {}",
                self.npc_now_hint(data, npc),
                self.npc_later_hint(data, npc)
            ))
        })
    }

    pub(super) fn nearby_npc<'a>(&self, data: &'a GameData) -> Option<&'a NpcDefinition> {
        self.visible_npcs(data).into_iter().find(|npc| {
            self.world.player
                .position
                .distance(self.npc_runtime_state(data, npc).position)
                <= npc.interaction_radius
        })
    }

    pub(super) fn visible_npcs<'a>(&self, data: &'a GameData) -> Vec<&'a NpcDefinition> {
        data.npcs
            .iter()
            .filter(|npc| self.npc_runtime_state(data, npc).area_id == self.world.current_area_id)
            .collect()
    }

    pub(super) fn npc_schedule_area_for_time<'a>(
        &self,
        npc: &'a NpcDefinition,
        time_window: &str,
    ) -> Option<&'a str> {
        npc.schedule
            .iter()
            .find(|entry| entry.time_window == time_window)
            .map(|entry| entry.area_id.as_str())
    }

    pub(super) fn npc_now_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let runtime = self.npc_runtime_state(data, npc);
        let area_name = data
            .area(&runtime.area_id)
            .map(|area| area.name.as_str())
            .unwrap_or("somewhere in town");
        if runtime.moving {
            let target_name = runtime
                .target_area_id
                .as_ref()
                .and_then(|area_id| data.area(area_id))
                .map(|area| area.name.as_str())
                .unwrap_or(area_name);
            ui_format("npc_travelling", &[("area", area_name), ("target", target_name)])
        } else {
            format!("{} this {}", area_name, self.current_time_window())
        }
    }

    pub(super) fn npc_later_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(current_index) = self.active_schedule_index(npc) else {
            return "routine unclear".to_owned();
        };
        let later_windows = ["day", "evening", "night", "morning"];
        let current_window = self.current_time_window();
        let next_window = later_windows
            .iter()
            .skip_while(|window| **window != current_window)
            .nth(1)
            .copied()
            .unwrap_or("morning");
        let later_area = self
            .npc_schedule_area_for_time(npc, next_window)
            .and_then(|area_id| data.area(area_id))
            .map(|area| area.name.as_str())
            .unwrap_or_else(|| {
                let next_index = (current_index + 1) % npc.schedule.len();
                data.area(&npc.schedule[next_index].area_id)
                    .map(|area| area.name.as_str())
                    .unwrap_or("unknown")
            });
        format!("{later_area} by {next_window}")
    }

    pub(super) fn npc_usual_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        ["morning", "day", "evening"]
            .iter()
            .filter_map(|time_window| {
                self.npc_schedule_area_for_time(npc, time_window)
                    .and_then(|area_id| data.area(area_id))
                    .map(|area| format!("{time_window} {}", area.name))
            })
            .collect::<Vec<_>>()
            .join("  |  ")
    }

    pub(super) fn npc_runtime_state(
        &self,
        _data: &GameData,
        npc: &NpcDefinition,
    ) -> NpcRuntimeState {
        if let Some(tracker) = self.runtime.npc_motion_states.get(&npc.id) {
            return NpcRuntimeState {
                area_id: tracker.area_id.clone(),
                position: tracker.position,
                direction: tracker.direction,
                moving: tracker.moving,
                target_area_id: tracker.target_area_id.clone(),
            };
        }

        let tracker = self.initial_npc_motion_tracker(_data, npc, 24.0);
        NpcRuntimeState {
            area_id: tracker.area_id,
            position: tracker.position,
            direction: tracker.direction,
            moving: tracker.moving,
            target_area_id: tracker.target_area_id,
        }
    }

    pub(super) fn travel_segments(
        &self,
        data: &GameData,
        start_area_id: &str,
        start_position: Vec2,
        target_area_id: &str,
        target_position: Vec2,
    ) -> Option<Vec<TravelSegment>> {
        if start_area_id == target_area_id {
            let area = data.area(start_area_id)?;
            return Some(self.local_travel_segments(
                area,
                start_area_id,
                start_position,
                target_position,
            ));
        }

        let path = self.area_path(data, start_area_id, target_area_id)?;
        let mut segments = Vec::new();
        let mut current_area_id = start_area_id.to_owned();
        let mut current_position = start_position;

        for warp_id in path {
            let area = data.area(&current_area_id)?;
            let warp = area.warps.iter().find(|warp| warp.id == warp_id)?;
            let exit_position = warp_center(warp);
            segments.extend(self.local_travel_segments(
                area,
                &current_area_id,
                current_position,
                exit_position,
            ));
            let arrival_position = matching_arrival_position(data, &current_area_id, warp)
                .unwrap_or_else(|| vec2(warp.target_position[0], warp.target_position[1]));
            current_area_id = warp.target_area.clone();
            current_position = arrival_position;
        }

        let final_area = data.area(&current_area_id)?;
        segments.extend(self.local_travel_segments(
            final_area,
            &current_area_id,
            current_position,
            target_position,
        ));
        Some(segments)
    }

    pub(super) fn local_travel_segments(
        &self,
        area: &AreaDefinition,
        area_id: &str,
        start: Vec2,
        end: Vec2,
    ) -> Vec<TravelSegment> {
        let points = self.local_path_points(area, start, end);
        points
            .windows(2)
            .map(|pair| TravelSegment {
                area_id: area_id.to_owned(),
                start: pair[0],
                end: pair[1],
            })
            .collect()
    }

    pub(super) fn local_path_points(
        &self,
        area: &AreaDefinition,
        start: Vec2,
        end: Vec2,
    ) -> Vec<Vec2> {
        let margin = PLAYER_RADIUS + 16.0;
        let corner_padding = 8.0;
        let start = clamp_npc_point(area, start);
        let end = clamp_npc_point(area, end);
        let mut nodes = vec![start, end];
        for blocker in &area.blockers {
            let left = (blocker.x - margin - corner_padding).max(PLAYER_RADIUS + corner_padding);
            let right = (blocker.x + blocker.w + margin + corner_padding)
                .min(area.size[0] - PLAYER_RADIUS - corner_padding);
            let top = (blocker.y - margin - corner_padding).max(PLAYER_RADIUS + corner_padding);
            let bottom = (blocker.y + blocker.h + margin + corner_padding)
                .min(area.size[1] - PLAYER_RADIUS - corner_padding);
            nodes.push(vec2(left, top));
            nodes.push(vec2(right, top));
            nodes.push(vec2(left, bottom));
            nodes.push(vec2(right, bottom));
        }
        nodes.retain(|point| !point_inside_expanded_blocker(area, *point, margin * 0.85));

        let mut distances = vec![f32::INFINITY; nodes.len()];
        let mut previous = vec![None::<usize>; nodes.len()];
        let mut visited = vec![false; nodes.len()];
        distances[0] = 0.0;

        for _ in 0..nodes.len() {
            let Some(current) = (0..nodes.len())
                .filter(|index| !visited[*index])
                .min_by(|left, right| distances[*left].total_cmp(&distances[*right]))
            else {
                break;
            };
            if current == 1 {
                break;
            }
            visited[current] = true;
            for next in 0..nodes.len() {
                if current == next || visited[next] {
                    continue;
                }
                if !segment_is_clear(area, nodes[current], nodes[next], margin) {
                    continue;
                }
                let candidate = distances[current] + nodes[current].distance(nodes[next]);
                if candidate < distances[next] {
                    distances[next] = candidate;
                    previous[next] = Some(current);
                }
            }
        }

        if !distances[1].is_finite() {
            for corner in [vec2(start.x, end.y), vec2(end.x, start.y)] {
                let corner = clamp_npc_point(area, corner);
                if segment_is_clear(area, start, corner, margin)
                    && segment_is_clear(area, corner, end, margin)
                {
                    return vec![start, corner, end];
                }
            }
            return vec![start];
        }

        let mut order = vec![1usize];
        let mut current = 1usize;
        while let Some(prev) = previous[current] {
            order.push(prev);
            current = prev;
            if current == 0 {
                break;
            }
        }
        order.reverse();
        order.into_iter().map(|index| nodes[index]).collect()
    }

    pub(super) fn area_path(
        &self,
        data: &GameData,
        start_area_id: &str,
        target_area_id: &str,
    ) -> Option<Vec<String>> {
        let mut frontier = vec![start_area_id.to_owned()];
        let mut came_from = BTreeMap::<String, (String, String)>::new();
        let mut index = 0;

        while index < frontier.len() {
            let area_id = frontier[index].clone();
            index += 1;
            if area_id == target_area_id {
                break;
            }
            let area = data.area(&area_id)?;
            for warp in &area.warps {
                if !came_from.contains_key(&warp.target_area) && warp.target_area != start_area_id {
                    came_from.insert(warp.target_area.clone(), (area_id.clone(), warp.id.clone()));
                    frontier.push(warp.target_area.clone());
                }
            }
        }

        if start_area_id != target_area_id && !came_from.contains_key(target_area_id) {
            return None;
        }

        let mut path = Vec::<String>::new();
        let mut current = target_area_id.to_owned();
        while current != start_area_id {
            let (previous_area, warp_id) = came_from.get(&current)?.clone();
            path.push(warp_id);
            current = previous_area;
        }
        path.reverse();
        Some(path)
    }

    pub(super) fn quest_location_hint(&self, data: &GameData, quest: &QuestDefinition) -> String {
        data.npc(&quest.giver_npc_id)
            .map(|npc| {
                format!(
                    "Now {}  |  Later {}",
                    self.npc_now_hint(data, npc),
                    self.npc_later_hint(data, npc)
                )
            })
            .unwrap_or_else(|| "Check the request board for delivery details.".to_owned())
    }

    pub(super) fn npc_context_line(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let relationship = self.progression.relationships.get(&npc.id).copied().unwrap_or_default();
        let base = format!(
            "Now: {}. Later: {}. Usually: {}. Rapport {}. Role {}.",
            self.npc_now_hint(data, npc),
            self.npc_later_hint(data, npc),
            self.npc_usual_hint(data, npc),
            relationship,
            if npc.role.is_empty() {
                "townsfolk"
            } else {
                npc.role.as_str()
            }
        );
        self.append_npc_story_line(&npc.id, base)
    }

    pub(super) fn active_schedule_index(&self, npc: &NpcDefinition) -> Option<usize> {
        if npc.schedule.is_empty() {
            return None;
        }
        let current_minutes = self.current_clock_minutes();
        let mut active_index = npc.schedule.len() - 1;
        for (index, entry) in npc.schedule.iter().enumerate() {
            if current_minutes >= schedule_start_minutes(&entry.time_window) {
                active_index = index;
            }
        }
        Some(active_index)
    }

    fn initial_npc_motion_tracker(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
        walk_speed: f32,
    ) -> NpcMotionTracker {
        let schedule_index = self.active_schedule_index(npc);
        let Some(current_index) = schedule_index else {
            return NpcMotionTracker {
                area_id: npc.area_id.clone(),
                position: vec2(npc.position[0], npc.position[1]),
                direction: Vec2::ZERO,
                moving: false,
                target_area_id: None,
                schedule_index: None,
                target_schedule_index: None,
                route_segments: Vec::new(),
                route_segment_index: 0,
            };
        };

        let next_index = (current_index + 1) % npc.schedule.len();
        let current_entry = &npc.schedule[current_index];
        let next_entry = &npc.schedule[next_index];
        let current_position = vec2(current_entry.position[0], current_entry.position[1]);
        let next_position = vec2(next_entry.position[0], next_entry.position[1]);
        let route = self
            .travel_segments(
                data,
                &current_entry.area_id,
                current_position,
                &next_entry.area_id,
                next_position,
            )
            .unwrap_or_default();
        let (elapsed_seconds, interval_seconds) =
            self.schedule_window_progress_seconds(data, npc, current_index);
        let travel_duration = self.npc_travel_duration_seconds(&route, walk_speed);
        let departure_seconds = (interval_seconds - travel_duration).max(0.0);

        let mut tracker = NpcMotionTracker {
            area_id: current_entry.area_id.clone(),
            position: current_position,
            direction: Vec2::ZERO,
            moving: false,
            target_area_id: None,
            schedule_index: Some(current_index),
            target_schedule_index: None,
            route_segments: Vec::new(),
            route_segment_index: 0,
        };

        if !route.is_empty() && elapsed_seconds >= departure_seconds {
            tracker.target_area_id = Some(next_entry.area_id.clone());
            tracker.target_schedule_index = Some(next_index);
            tracker.route_segments = route;
            tracker.route_segment_index = 0;
            tracker.moving = true;
            let pre_advance = elapsed_seconds - departure_seconds;
            self.advance_npc_tracker(&mut tracker, walk_speed, pre_advance);
        }

        tracker
    }

    fn schedule_window_progress_seconds(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
        current_index: usize,
    ) -> (f32, f32) {
        let current_entry = &npc.schedule[current_index];
        let next_index = (current_index + 1) % npc.schedule.len();
        let next_entry = &npc.schedule[next_index];
        let current_start = schedule_start_minutes(&current_entry.time_window);
        let mut next_start = schedule_start_minutes(&next_entry.time_window);
        if next_start <= current_start {
            next_start += 24.0 * 60.0;
        }
        let mut current_minutes = self.current_clock_minutes();
        if current_minutes < current_start {
            current_minutes += 24.0 * 60.0;
        }
        let interval_minutes = next_start - current_start;
        let elapsed_minutes = (current_minutes - current_start).clamp(0.0, interval_minutes);
        let seconds_per_minute = data.config.day_length_seconds / (24.0 * 60.0);
        (
            elapsed_minutes * seconds_per_minute,
            interval_minutes * seconds_per_minute,
        )
    }

    fn npc_travel_duration_seconds(&self, route: &[TravelSegment], walk_speed: f32) -> f32 {
        if route.is_empty() {
            0.0
        } else {
            route
                .iter()
                .map(|segment| segment.start.distance(segment.end))
                .sum::<f32>()
                / walk_speed.max(1.0)
        }
    }

    fn advance_npc_tracker(
        &self,
        tracker: &mut NpcMotionTracker,
        walk_speed: f32,
        frame_time: f32,
    ) {
        let mut remaining = walk_speed * frame_time;
        while remaining > 0.0 && tracker.route_segment_index < tracker.route_segments.len() {
            let segment = &tracker.route_segments[tracker.route_segment_index];
            let to_end = segment.end - tracker.position;
            let distance = to_end.length();

            if distance <= 0.5 {
                tracker.position = segment.end;
                tracker.area_id = segment.area_id.clone();
                tracker.route_segment_index += 1;
                if let Some(next_segment) = tracker.route_segments.get(tracker.route_segment_index)
                {
                    if next_segment.area_id != segment.area_id {
                        tracker.area_id = next_segment.area_id.clone();
                        tracker.position = next_segment.start;
                        tracker.direction = Vec2::ZERO;
                        tracker.moving = true;
                        break;
                    }
                }
                continue;
            }

            let step = remaining.min(distance);
            let direction = to_end / distance;
            tracker.position += direction * step;
            tracker.direction = direction;
            tracker.area_id = segment.area_id.clone();
            tracker.moving = true;
            remaining -= step;

            if step >= distance - 0.01 {
                tracker.position = segment.end;
                tracker.route_segment_index += 1;
                if let Some(next_segment) = tracker.route_segments.get(tracker.route_segment_index)
                {
                    if next_segment.area_id != segment.area_id {
                        tracker.area_id = next_segment.area_id.clone();
                        tracker.position = next_segment.start;
                        tracker.direction = Vec2::ZERO;
                        tracker.moving = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }

        if tracker.route_segment_index >= tracker.route_segments.len() {
            tracker.route_segments.clear();
            tracker.route_segment_index = 0;
            tracker.moving = false;
            tracker.direction = Vec2::ZERO;
            tracker.schedule_index = tracker.target_schedule_index.or(tracker.schedule_index);
            tracker.target_schedule_index = None;
            tracker.target_area_id = None;
        }
    }
}

pub(super) fn schedule_start_minutes(time_window: &str) -> f32 {
    match time_window {
        "morning" => 6.0 * 60.0,
        "day" => 11.0 * 60.0,
        "evening" => 17.0 * 60.0,
        _ => 21.0 * 60.0,
    }
}

pub(super) fn warp_center(warp: &WarpDefinition) -> Vec2 {
    vec2(
        warp.rect.x + warp.rect.w * 0.5,
        warp.rect.y + warp.rect.h * 0.5,
    )
}

fn matching_arrival_position(
    data: &GameData,
    source_area_id: &str,
    warp: &WarpDefinition,
) -> Option<Vec2> {
    let target_area = data.area(&warp.target_area)?;
    target_area
        .warps
        .iter()
        .find(|candidate| candidate.target_area == source_area_id)
        .map(warp_center)
}

pub(super) fn npc_motion_seed(id: &str) -> f32 {
    let mut value = 0u32;
    for byte in id.as_bytes() {
        value = value.wrapping_mul(33).wrapping_add(*byte as u32);
    }
    (value % 360) as f32 * 0.017453292
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn town_to_tower_path_routes_through_plains() {
        let data = GameData::fallback();
        let state = GameplayState::new(&data);

        let path = state
            .area_path(&data, "town_square", "tower_entry")
            .expect("town should connect to tower");

        assert_eq!(path, vec!["town_to_plains".to_owned(), "plains_to_entry".to_owned()]);
    }

    #[test]
    fn town_to_forest_path_routes_through_plains() {
        let data = GameData::fallback();
        let state = GameplayState::new(&data);

        let path = state
            .area_path(&data, "town_square", "moonlit_forest")
            .expect("town should connect to east forest");

        assert_eq!(path, vec!["town_to_plains".to_owned(), "plains_to_forest".to_owned()]);
    }
}

pub(super) fn segment_is_clear(area: &AreaDefinition, start: Vec2, end: Vec2, margin: f32) -> bool {
    !area
        .blockers
        .iter()
        .any(|blocker| line_intersects_expanded_rect(start, end, blocker, margin))
}

fn clamp_npc_point(area: &AreaDefinition, point: Vec2) -> Vec2 {
    vec2(
        point
            .x
            .clamp(PLAYER_RADIUS + 4.0, area.size[0] - PLAYER_RADIUS - 4.0),
        point
            .y
            .clamp(PLAYER_RADIUS + 4.0, area.size[1] - PLAYER_RADIUS - 4.0),
    )
}

fn point_inside_expanded_blocker(area: &AreaDefinition, point: Vec2, margin: f32) -> bool {
    area.blockers.iter().any(|rect| {
        point.x > rect.x - margin
            && point.x < rect.x + rect.w + margin
            && point.y > rect.y - margin
            && point.y < rect.y + rect.h + margin
    })
}

pub(super) fn line_intersects_expanded_rect(
    start: Vec2,
    end: Vec2,
    rect: &crate::data::RectDefinition,
    margin: f32,
) -> bool {
    let min_x = rect.x - margin;
    let max_x = rect.x + rect.w + margin;
    let min_y = rect.y - margin;
    let max_y = rect.y + rect.h + margin;

    if start.x > min_x && start.x < max_x && start.y > min_y && start.y < max_y {
        return true;
    }
    if end.x > min_x && end.x < max_x && end.y > min_y && end.y < max_y {
        return true;
    }

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let mut t0 = 0.0;
    let mut t1 = 1.0;

    for (p, q) in [
        (-dx, start.x - min_x),
        (dx, max_x - start.x),
        (-dy, start.y - min_y),
        (dy, max_y - start.y),
    ] {
        if p.abs() <= f32::EPSILON {
            if q < 0.0 {
                return false;
            }
            continue;
        }
        let r = q / p;
        if p < 0.0 {
            if r > t1 {
                return false;
            }
            if r > t0 {
                t0 = r;
            }
        } else {
            if r < t0 {
                return false;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }

    t0 <= t1
}


