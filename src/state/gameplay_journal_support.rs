use super::GameplayState;
use crate::content::{narrative_text, ui_copy, ui_format};
use crate::data::{GameData, JournalMilestoneEntry};
use macroquad::prelude::{screen_width, Rect};

pub(super) struct JournalMilestoneSummary {
    pub(super) title: String,
    pub(super) text: String,
}

impl GameplayState {
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

    pub(super) fn recent_journal_milestones(&self, limit: usize) -> Vec<JournalMilestoneSummary> {
        self.progression
            .journal_milestones
            .iter()
            .rev()
            .take(limit)
            .map(|milestone| JournalMilestoneSummary {
                title: milestone.title.clone(),
                text: milestone.text.clone(),
            })
            .collect()
    }

    fn greenhouse_journal_unlocked(&self) -> bool {
        self.progression
            .unlocked_warps
            .contains("entry_to_greenhouse")
    }
}

pub(super) fn initial_journal_milestones() -> Vec<JournalMilestoneEntry> {
    vec![narrative_text()
        .milestones
        .entry_lab_recovered
        .to_journal_entry()]
}
