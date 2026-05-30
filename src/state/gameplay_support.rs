use super::{GatherFeedback, GatherToast, GameplayState};
use crate::content::{narrative_text, ui_copy, ui_format};
use crate::data::{
    GameData, JournalMilestoneEntry, WarpDefinition,
};
use macroquad::prelude::{screen_width, Color, Rect, Vec2};

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
        Some(ui_format(
            "journal_active_summary",
            &[
                ("title", &quest.title),
                ("requirements", &self.quest_requirement_summary(data, quest)),
                ("location", &location),
            ],
        ))
    }

    pub(super) fn milestone_status_lines(&self) -> Vec<(&'static str, String, bool)> {
        vec![
            (
                ui_copy("milestone_greenhouse_access"),
                if self
                    .progression
                    .unlocked_warps
                    .contains("entry_to_greenhouse")
                {
                    ui_copy("milestone_greenhouse_restored").to_owned()
                } else {
                    ui_copy("milestone_greenhouse_locked").to_owned()
                },
                self.progression
                    .unlocked_warps
                    .contains("entry_to_greenhouse"),
            ),
            (
                ui_copy("milestone_archive_reconstruction"),
                if self.has_journal_milestone("archive_revelation") {
                    ui_copy("milestone_archive_recovered").to_owned()
                } else if self.can_reconstruct_archive() {
                    ui_copy("milestone_archive_ready").to_owned()
                } else {
                    ui_copy("milestone_archive_locked").to_owned()
                },
                self.has_journal_milestone("archive_revelation") || self.can_reconstruct_archive(),
            ),
            (
                ui_copy("milestone_observatory_access"),
                if self.has_journal_milestone("archive_revelation") {
                    ui_copy("milestone_observatory_ready").to_owned()
                } else {
                    ui_copy("milestone_observatory_locked").to_owned()
                },
                self.has_journal_milestone("archive_revelation"),
            ),
        ]
    }

    pub(super) fn greenhouse_journal_unlocked(&self) -> bool {
        self.progression
            .unlocked_warps
            .contains("entry_to_greenhouse")
    }

    pub(super) fn journal_tabs(&self) -> Vec<&'static str> {
        let mut tabs = vec![
            ui_copy("journal_tab_routes"),
            ui_copy("journal_tab_notes"),
            ui_copy("journal_tab_brews"),
        ];
        if self.greenhouse_journal_unlocked() {
            tabs.push(ui_copy("journal_tab_greenhouse"));
        }
        tabs.push(ui_copy("journal_tab_rapport"));
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
        self.push_event_toast_with_icon(text, color, "");
    }

    pub(super) fn push_event_toast_with_icon(
        &mut self,
        _text: impl Into<String>,
        _color: Color,
        _icon_key: &str,
    ) {
        self.runtime.gather_toasts.insert(
            0,
            GatherToast {
                remaining_seconds: 2.2,
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
            return ui_copy("goal_reconstruct_archive").to_owned();
        }

        if let Some(quest) = data
            .quests
            .iter()
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .find(|quest| self.quest_is_available(quest))
        {
            return ui_format(
                "goal_accept_quest",
                &[
                    ("title", &quest.title),
                    ("location", &self.quest_location_hint(data, quest)),
                ],
            );
        }

        if let Some(warp) = self.next_locked_warp(data) {
            return ui_format(
                "goal_restore_route",
                &[
                    ("label", &warp.label),
                    ("requirements", &self.warp_requirement_summary(data, warp)),
                ],
            );
        }

        ui_copy("goal_keep_working").to_owned()
    }

}

pub(super) fn rgba(values: [u8; 4]) -> Color {
    Color::from_rgba(values[0], values[1], values[2], values[3])
}

pub(super) fn quality_band_rank(band: &str) -> u8 {
    match band {
        value if value == ui_copy("quality_band_crude") => 0,
        value if value == ui_copy("quality_band_serviceable") => 1,
        value if value == ui_copy("quality_band_fine") => 2,
        value if value == ui_copy("quality_band_excellent") => 3,
        value if value == ui_copy("quality_band_masterwork") => 4,
        _ => 0,
    }
}

pub(super) fn planter_stage_label(growth_days: u32, total_days: u32) -> &'static str {
    if growth_days == 0 {
        ui_copy("planter_stage_seeded")
    } else if growth_days >= total_days {
        ui_copy("planter_stage_ripe")
    } else if growth_days * 2 >= total_days {
        ui_copy("planter_stage_budding")
    } else {
        ui_copy("planter_stage_sprouting")
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
