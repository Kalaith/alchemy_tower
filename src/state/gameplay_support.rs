use super::gameplay_npc::npc_motion_seed;
use super::*;
use crate::art::default_toast_icon_key;
use crate::content::{narrative_text, ui_copy, ui_format};

impl GameplayState {
    pub(super) fn locked_warps<'a>(&self, data: &'a GameData) -> Vec<&'a WarpDefinition> {
        data.areas
            .iter()
            .flat_map(|area| area.warps.iter())
            .filter(|warp| !self.warp_is_unlocked(warp))
            .collect()
    }

    pub(super) fn next_locked_warp<'a>(&self, data: &'a GameData) -> Option<&'a WarpDefinition> {
        self.locked_warps(data)
            .into_iter()
            .min_by_key(|warp| self.warp_progress_score(data, warp))
    }

    pub(super) fn warp_progress_score(&self, _data: &GameData, warp: &WarpDefinition) -> u32 {
        let owned = self
            .inventory
            .get(&warp.required_item_id)
            .copied()
            .unwrap_or_default();
        let item_missing = warp.required_item_amount.saturating_sub(owned);
        let milestone_missing = u32::from(
            !warp.required_journal_milestone.is_empty()
                && !self.has_journal_milestone(&warp.required_journal_milestone),
        );

        warp.required_total_brews
            .saturating_sub(self.progression.total_brews)
            .saturating_mul(100)
            .saturating_add(warp.required_coins.saturating_sub(self.coins))
            .saturating_add(item_missing.saturating_mul(25))
            .saturating_add(milestone_missing.saturating_mul(150))
    }

    pub(super) fn active_quest_summary(&self, data: &GameData) -> Option<String> {
        let quest = self
            .progression
            .started_quests
            .iter()
            .find_map(|quest_id| data.quest(quest_id))?;
        let location = self.quest_location_hint(data, quest);
        Some(format!(
            "{}  |  {}  |  {}",
            quest.title,
            self.quest_requirement_summary(data, quest),
            location
        ))
    }

    pub(super) fn milestone_status_lines(&self) -> Vec<(&'static str, String, bool)> {
        vec![
            (
                "Greenhouse access",
                if self.progression.unlocked_warps.contains("entry_to_greenhouse") {
                    "Greenhouse floor restored.".to_owned()
                } else {
                    "Reach 10 brews, 40 coins, and carry 2 Moon Fern.".to_owned()
                },
                self.progression.unlocked_warps.contains("entry_to_greenhouse"),
            ),
            (
                "Archive reconstruction",
                if self.has_journal_milestone("archive_revelation") {
                    "Archive revelation recovered.".to_owned()
                } else if self.can_reconstruct_archive() {
                    "Ready at the archive console.".to_owned()
                } else {
                    "Finish star_elixir_for_ione, containment_for_lyra, and restore the tower floors."
                        .to_owned()
                },
                self.has_journal_milestone("archive_revelation") || self.can_reconstruct_archive(),
            ),
            (
                "Observatory access",
                if self.has_journal_milestone("archive_revelation") {
                    "Portal Observatory can be opened from the archives.".to_owned()
                } else {
                    "Recover the archive revelation.".to_owned()
                },
                self.has_journal_milestone("archive_revelation"),
            ),
        ]
    }

    pub(super) fn greenhouse_journal_unlocked(&self) -> bool {
        self.progression.unlocked_warps.contains("entry_to_greenhouse")
    }

    pub(super) fn journal_tabs(&self) -> Vec<&'static str> {
        let mut tabs = vec!["Routes", "Notes", "Brews"];
        if self.greenhouse_journal_unlocked() {
            tabs.push("Greenhouse");
        }
        tabs.push("Rapport");
        tabs
    }

    pub(super) fn journal_tab_rect(&self, index: usize, tab_count: usize) -> Rect {
        let x = 120.0;
        let y = 72.0;
        let w = screen_width() - 240.0;
        let tab_y = y + 82.0;
        let tab_w = (w - 40.0) / tab_count.max(1) as f32;
        Rect::new(x + 20.0 + tab_w * index as f32, tab_y, tab_w - 8.0, 30.0)
    }

    pub(super) fn journal_close_rect(&self) -> Rect {
        let x = 120.0;
        let y = 72.0;
        let w = screen_width() - 240.0;
        Rect::new(x + w - 112.0, y + 16.0, 92.0, 28.0)
    }

    pub(super) fn push_event_toast(&mut self, text: impl Into<String>, color: Color) {
        self.push_event_toast_with_icon(text, color, default_toast_icon_key());
    }

    pub(super) fn push_event_toast_with_icon(
        &mut self,
        text: impl Into<String>,
        color: Color,
        icon_key: &str,
    ) {
        self.runtime.gather_toasts.insert(
            0,
            GatherToast {
                text: text.into(),
                remaining_seconds: 2.2,
                color,
                icon_key: icon_key.to_owned(),
            },
        );
        self.runtime.gather_toasts.truncate(3);
    }

    pub(super) fn trigger_camera_shake(&mut self, seconds: f32, intensity: f32) {
        self.runtime.camera_shake_seconds = self.runtime.camera_shake_seconds.max(seconds);
        self.runtime.camera_shake_intensity = self.runtime.camera_shake_intensity.max(intensity);
    }

    pub(super) fn trigger_world_feedback(
        &mut self,
        position: Vec2,
        color: Color,
        emphasis: bool,
        burst_scale: f32,
    ) {
        self.runtime.gather_feedbacks.push(GatherFeedback {
            position,
            remaining_seconds: if emphasis { 0.9 } else { 0.55 },
            color,
            emphasis,
            burst_scale,
        });
    }

    pub(super) fn resolve_area_collision(&self, area: &AreaDefinition, candidate: Vec2) -> Vec2 {
        let clamped = vec2(
            candidate
                .x
                .clamp(PLAYER_RADIUS, area.size[0] - PLAYER_RADIUS),
            candidate
                .y
                .clamp(PLAYER_RADIUS, area.size[1] - PLAYER_RADIUS),
        );
        if !self.collides(area, clamped) {
            return clamped;
        }
        let x_only = vec2(clamped.x, self.world.player.position.y);
        if !self.collides(area, x_only) {
            return x_only;
        }
        let y_only = vec2(self.world.player.position.x, clamped.y);
        if !self.collides(area, y_only) {
            return y_only;
        }
        self.world.player.position
    }

    pub(super) fn collides(&self, area: &AreaDefinition, point: Vec2) -> bool {
        area.blockers.iter().any(|blocker| {
            let nearest = vec2(
                point.x.clamp(blocker.x, blocker.x + blocker.w),
                point.y.clamp(blocker.y, blocker.y + blocker.h),
            );
            point.distance_squared(nearest) < PLAYER_RADIUS * PLAYER_RADIUS
        })
    }

    pub(super) fn camera_offset(&self, area: &AreaDefinition) -> Vec2 {
        let half = vec2(screen_width() * 0.5, screen_height() * 0.5);
        let unclamped = half - self.world.player.position;
        let min_x = screen_width() - area.size[0] - CAMERA_PADDING;
        let min_y = screen_height() - area.size[1] - CAMERA_PADDING;
        let mut offset = vec2(
            unclamped.x.clamp(min_x.min(CAMERA_PADDING), CAMERA_PADDING),
            unclamped.y.clamp(min_y.min(CAMERA_PADDING), CAMERA_PADDING),
        );
        if self.runtime.camera_shake_seconds > 0.0 && self.runtime.camera_shake_intensity > 0.0 {
            let t = get_time() as f32 * 45.0;
            let shake = vec2(t.sin(), (t * 1.37).cos())
                * self.runtime.camera_shake_intensity
                * (self.runtime.camera_shake_seconds / 0.25).min(1.0);
            offset += shake;
        }
        offset
    }

    pub(super) fn npc_draw_position(&self, npc: &NpcDefinition, runtime: &NpcRuntimeState) -> Vec2 {
        if !runtime.moving || runtime.direction.length_squared() <= 0.0 {
            return runtime.position;
        }
        let perpendicular = vec2(-runtime.direction.y, runtime.direction.x);
        let seed = npc_motion_seed(&npc.id);
        let sway = ((get_time() as f32 * 4.5) + seed).sin() * 1.6;
        runtime.position + perpendicular * sway
    }

    pub(super) fn item_card_meta(
        &self,
        data: &GameData,
        item_id: &str,
        amount: u32,
        extra: &str,
    ) -> String {
        let item = data.item(item_id);
        let base = format!(
            "{}  q{} r{}  x{}",
            item.map(|item| item.category.as_str()).unwrap_or("?"),
            item.map(|item| item.quality).unwrap_or_default(),
            item.map(|item| item.rarity).unwrap_or_default(),
            amount
        );
        if extra.is_empty() {
            base
        } else {
            format!("{base}  {extra}")
        }
    }

    pub(super) fn locked_state_text(&self, detail: &str) -> String {
        ui_format("locked_prefix", &[("detail", detail)])
    }

    pub(super) fn unavailable_state_text(&self, detail: &str) -> String {
        ui_format("unavailable_prefix", &[("detail", detail)])
    }

    pub(super) fn current_time_window(&self) -> &'static str {
        let total_minutes = self.current_clock_minutes() as i32;
        match total_minutes {
            360..=659 => "morning",
            660..=1019 => "day",
            1020..=1259 => "evening",
            _ => "night",
        }
    }

    pub(super) fn current_clock_minutes(&self) -> f32 {
        (self.world.day_clock_seconds / self.world.day_length_seconds) * 24.0 * 60.0
    }

    pub(super) fn set_clock_minutes(&mut self, minutes: f32) {
        self.world.day_clock_seconds =
            (minutes / (24.0 * 60.0)) * self.world.day_length_seconds;
    }

    pub(super) fn advance_to_next_day(&mut self, data: &GameData, with_feedback: bool) {
        self.world.day_index += 1;
        self.world.gathered_nodes.clear();
        self.advance_planters(data);
        self.refresh_available_nodes(data);
        if with_feedback {
            self.runtime.status_text = format!(
                "A new day begins: {} in {}.",
                self.current_weather(),
                self.current_season()
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(176, 226, 255, 255),
                true,
                1.5,
            );
            self.trigger_camera_shake(0.22, 6.0);
        }
    }

    pub(super) fn sleep_until(&mut self, data: &GameData, wake_minutes: f32, forced_home: bool) {
        let advanced_day = self.current_clock_minutes() >= wake_minutes;
        if advanced_day {
            self.advance_to_next_day(data, false);
        }
        self.set_clock_minutes(wake_minutes);
        if forced_home {
            if let Some(bed) = data.stations.iter().find(|station| station.id == "entry_rest_bed") {
                self.world.current_area_id = bed.area_id.clone();
                self.world.player.position = vec2(bed.position[0], bed.position[1] + 52.0);
                self.world.player.facing = vec2(0.0, -1.0);
                self.world.player.moving = false;
            }
            self.runtime.sleep_flash_seconds = 1.2;
            self.runtime.status_text = ui_copy("gameplay_fainted_home").to_owned();
        } else {
            self.runtime.status_text =
                ui_format("gameplay_slept_until", &[("time", "07:00")]);
        }
    }

    pub(super) fn handle_sleep_pressure(&mut self, data: &GameData) {
        let minutes = self.current_clock_minutes();
        if (60.0..120.0).contains(&minutes) {
            self.sleep_until(data, 10.0 * 60.0, true);
        }
    }

    pub(super) fn next_goal_summary(&self, data: &GameData) -> String {
        if let Some(quest) = self
            .progression
            .started_quests
            .iter()
            .find_map(|quest_id| data.quest(quest_id))
        {
            return format!(
                "{} ({})",
                quest.title,
                self.quest_requirement_summary(data, quest)
            );
        }

        if self.can_reconstruct_archive() && !self.has_journal_milestone("archive_revelation") {
            return "Reconstruct the archive timeline from the console.".to_owned();
        }

        if let Some(quest) = data
            .quests
            .iter()
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .find(|quest| self.quest_is_available(quest))
        {
            return format!(
                "Accept {} from {}.",
                quest.title,
                self.quest_location_hint(data, quest)
            );
        }

        if let Some(warp) = self.next_locked_warp(data) {
            return format!(
                "Restore {}: {}.",
                warp.label,
                self.warp_requirement_summary(data, warp)
            );
        }

        "Keep gathering, brewing, and filling the archive.".to_owned()
    }

    pub(super) fn station_world_label(
        &self,
        data: &GameData,
        station: &StationDefinition,
    ) -> Option<(String, Color)> {
        match station.kind {
            StationKind::Alchemy if self.progression.total_brews < 3 => Some((
                ui_copy("world_marker_brew_here").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            )),
            StationKind::QuestBoard if !self.available_board_quests(data).is_empty() => Some((
                ui_copy("world_marker_new_requests").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            )),
            StationKind::ArchiveConsole
                if self.can_reconstruct_archive()
                    && !self.has_journal_milestone("archive_revelation") =>
            {
                Some((
                    ui_copy("world_marker_rebuild_ready").to_owned(),
                    Color::from_rgba(176, 226, 255, 255),
                ))
            }
            StationKind::Planter => self
                .progression
                .planter_states
                .get(&station.id)
                .filter(|state| state.ready)
                .map(|_| {
                    (
                        ui_copy("world_marker_harvest_ready").to_owned(),
                        Color::from_rgba(188, 255, 220, 255),
                    )
                }),
            StationKind::Habitat => self
                .progression
                .habitat_states
                .get(&station.id)
                .filter(|state| {
                    !state.creature_item_id.is_empty()
                        && self.world.day_index
                            >= state
                                .last_harvest_day
                                .saturating_add(station.habitat_harvest_days.max(1))
                })
                .map(|_| {
                    (
                        ui_copy("world_marker_collect_ready").to_owned(),
                        Color::from_rgba(188, 255, 220, 255),
                    )
                }),
            StationKind::EndingFocus if self.has_journal_milestone("archive_revelation") => Some((
                ui_copy("world_marker_final_focus").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            )),
            _ => None,
        }
    }

    pub(super) fn npc_world_label(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
    ) -> Option<(String, Color)> {
        let quest = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()?;
        if self.progression.completed_quests.contains(&quest.id) {
            return None;
        }
        if self.progression.started_quests.contains(&quest.id) {
            if self.quest_requirements_met(data, quest) {
                Some((
                    ui_copy("world_marker_turn_in").to_owned(),
                    Color::from_rgba(188, 255, 220, 255),
                ))
            } else {
                Some((
                    ui_copy("world_marker_awaiting_brew").to_owned(),
                    Color::from_rgba(255, 230, 170, 255),
                ))
            }
        } else if self.quest_is_available(quest) {
            Some((
                ui_copy("world_marker_request").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else {
            None
        }
    }

    pub(super) fn update_tutorial_hints(&mut self, data: &GameData, frame_time: f32) {
        if self.runtime.tutorial.next_hint_delay_seconds > 0.0 {
            self.runtime.tutorial.next_hint_delay_seconds =
                (self.runtime.tutorial.next_hint_delay_seconds - frame_time).max(0.0);
        }
        if self.runtime.tutorial.next_hint_delay_seconds > 0.0
            || !self.runtime.gather_toasts.is_empty()
            || self.ui.journal_open
            || self.ui.quest_board_open
            || self.ui.shop_open
            || self.ui.rune_open
            || self.ui.archive_open
            || self.ui.dialogue_open
            || self.ui.ending_open
            || self.alchemy.open
        {
            return;
        }

        let near_alchemy = self
            .nearby_station(data)
            .map(|station| station.kind == StationKind::Alchemy)
            .unwrap_or(false);
        let near_quest_npc = self
            .nearby_npc(data)
            .and_then(|npc| self.npc_world_label(data, npc))
            .is_some();
        let nearby_available_node = data
            .area(&self.world.current_area_id)
            .map(|area| {
                area.gather_nodes.iter().any(|node| {
                    !self.world.gathered_nodes.contains(&node.id)
                        && self.node_is_available(node)
                        && self
                            .world
                            .player
                            .position
                            .distance(vec2(node.position[0], node.position[1]))
                            <= node.radius + data.config.interaction_range + 36.0
                })
            })
            .unwrap_or(false);
        let unlockable_warp_here = data
            .area(&self.world.current_area_id)
            .map(|area| {
                area.warps
                    .iter()
                    .any(|warp| !self.warp_is_unlocked(warp) && self.can_unlock_warp(warp))
            })
            .unwrap_or(false);
        let has_quick_potions = !self.quick_potions(data).is_empty();
        let next_hint = if !self.runtime.tutorial.crow_intro_hint_shown {
            self.runtime.tutorial.crow_intro_hint_shown = true;
            Some((
                ui_copy("tutorial_crow_intro").to_owned(),
                Color::from_rgba(176, 226, 255, 255),
            ))
        } else if !self.runtime.tutorial.save_hint_shown {
            self.runtime.tutorial.save_hint_shown = true;
            Some((
                ui_copy("tutorial_save").to_owned(),
                Color::from_rgba(176, 226, 255, 255),
            ))
        } else if !self.runtime.tutorial.journal_hint_shown {
            self.runtime.tutorial.journal_hint_shown = true;
            Some((
                ui_copy("tutorial_journal").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.alchemy_hint_shown && near_alchemy {
            self.runtime.tutorial.alchemy_hint_shown = true;
            Some((
                ui_copy("tutorial_alchemy_open").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.brew_goal_hint_shown && self.progression.total_brews == 0 && near_alchemy {
            self.runtime.tutorial.brew_goal_hint_shown = true;
            Some((
                ui_copy("tutorial_brew_goal").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.potion_hint_shown && has_quick_potions {
            self.runtime.tutorial.potion_hint_shown = true;
            Some((
                ui_copy("tutorial_potions").to_owned(),
                Color::from_rgba(255, 214, 132, 255),
            ))
        } else if !self.runtime.tutorial.gather_hint_shown
            && self.progression.field_journal.is_empty()
            && nearby_available_node
        {
            self.runtime.tutorial.gather_hint_shown = true;
            Some((
                ui_copy("tutorial_gather").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.mira_hint_shown
            && self.progression.total_brews > 0
            && !self.progression.completed_quests.contains("healing_for_mira")
        {
            self.runtime.tutorial.mira_hint_shown = true;
            Some((
                ui_copy("tutorial_mira_delivery").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.rowan_hint_shown
            && self.progression.completed_quests.contains("healing_for_mira")
            && !self.progression.completed_quests.contains("glow_for_rowan")
        {
            self.runtime.tutorial.rowan_hint_shown = true;
            Some((
                ui_copy("tutorial_rowan_goal").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.quest_hint_shown
            && self.progression.started_quests.is_empty()
            && self.progression.completed_quests.is_empty()
            && (near_quest_npc || !self.available_board_quests(data).is_empty())
        {
            self.runtime.tutorial.quest_hint_shown = true;
            Some((
                ui_copy("tutorial_quest").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else if !self.runtime.tutorial.delivery_hint_shown
            && self
                .progression
                .started_quests
                .iter()
                .filter_map(|quest_id| data.quest(quest_id))
                .any(|quest| self.quest_requirements_met(data, quest))
        {
            self.runtime.tutorial.delivery_hint_shown = true;
            Some((
                ui_copy("tutorial_delivery_ready").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            ))
        } else if !self.runtime.tutorial.route_hint_shown && unlockable_warp_here {
            self.runtime.tutorial.route_hint_shown = true;
            Some((
                ui_copy("tutorial_route_ready").to_owned(),
                Color::from_rgba(176, 226, 255, 255),
            ))
        } else {
            None
        };

        if let Some((text, color)) = next_hint {
            self.push_event_toast(text, color);
            self.runtime.tutorial.next_hint_delay_seconds = 6.0;
        }
    }
}

pub(super) fn rgba(values: [u8; 4]) -> Color {
    Color::from_rgba(values[0], values[1], values[2], values[3])
}

pub(super) fn clock_text(day_clock_seconds: f32, full_day_seconds: f32) -> String {
    let total_minutes = ((day_clock_seconds / full_day_seconds) * 24.0 * 60.0) as i32;
    format!(
        "{:02}:{:02}",
        (total_minutes / 60).rem_euclid(24),
        total_minutes.rem_euclid(60)
    )
}

pub(super) fn effect_name(kind: EffectKind) -> &'static str {
    match kind {
        EffectKind::Glow => "Glow",
        EffectKind::Speed => "Quickstep",
        EffectKind::Misfire => "Residue",
        EffectKind::Restore => "Restore",
    }
}

pub(super) fn quality_band_rank(band: &str) -> u8 {
    match band {
        "Crude" => 0,
        "Serviceable" => 1,
        "Fine" => 2,
        "Excellent" => 3,
        "Masterwork" => 4,
        _ => 0,
    }
}

pub(super) fn planter_stage_label(growth_days: u32, total_days: u32) -> &'static str {
    if growth_days == 0 {
        "seeded"
    } else if growth_days >= total_days {
        "ripe"
    } else if growth_days * 2 >= total_days {
        "budding"
    } else {
        "sprouting"
    }
}

pub(super) fn starting_day_time(data: &GameData) -> f32 {
    data.config.day_length_seconds * 0.30
}

pub(super) fn initial_journal_milestones() -> Vec<JournalMilestoneEntry> {
    vec![narrative_text()
        .milestones
        .entry_lab_recovered
        .to_journal_entry()]
}

#[cfg(test)]
mod tests {
    use super::*;

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


