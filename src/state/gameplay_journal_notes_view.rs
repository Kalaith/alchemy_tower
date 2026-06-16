use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{
    JournalMilestoneStatusView, JournalNotesTabView, JournalRecentMilestoneView,
};

impl GameplayState {
    pub(super) fn journal_notes_tab_view(&self, data: &GameData) -> JournalNotesTabView {
        JournalNotesTabView {
            title: ui_copy("overlay_tower_notes"),
            active_title: ui_copy("overlay_progress_active"),
            milestones_title: ui_copy("overlay_progress_milestones"),
            active_summary: self
                .active_quest_summary(data)
                .unwrap_or_else(|| self.next_goal_summary(data)),
            milestone_rows: self
                .milestone_status_lines(data)
                .into_iter()
                .map(|(label, detail, ready)| JournalMilestoneStatusView {
                    title: ui_format(
                        "journal_milestone_status_title",
                        &[
                            ("label", label),
                            (
                                "status",
                                if ready {
                                    ui_copy("overlay_progress_ready")
                                } else {
                                    ui_copy("overlay_progress_locked")
                                },
                            ),
                        ],
                    ),
                    detail,
                })
                .collect(),
            recent_milestones: self
                .recent_journal_milestones(5)
                .into_iter()
                .map(|milestone| JournalRecentMilestoneView {
                    title: milestone.title,
                    text: milestone.text,
                })
                .collect(),
        }
    }
}
