use super::GameplayState;
use crate::content::narrative_text;
use crate::data::{GameData, JournalMilestoneEntry};

#[path = "gameplay_journal_support_text.rs"]
mod journal_support_text;

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
        Some(journal_support_text::active_quest_summary(
            &quest.title,
            &self.quest_requirement_summary(data, quest),
            &location,
        ))
    }

    pub(super) fn milestone_status_lines(&self) -> Vec<(&'static str, String, bool)> {
        let greenhouse_restored = self
            .progression
            .unlocked_warps
            .contains("entry_to_greenhouse");
        let archive_recovered = self.has_journal_milestone("archive_revelation");
        let archive_ready = self.can_reconstruct_archive();

        vec![
            journal_support_text::greenhouse_status(greenhouse_restored),
            journal_support_text::archive_status(archive_recovered, archive_ready),
            journal_support_text::observatory_status(archive_recovered),
        ]
    }

    pub(super) fn journal_tabs(&self) -> Vec<&'static str> {
        journal_support_text::tabs(self.greenhouse_journal_unlocked())
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

    pub(super) fn greenhouse_journal_unlocked(&self) -> bool {
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
