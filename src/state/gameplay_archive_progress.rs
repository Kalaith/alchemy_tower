use super::GameplayState;
use crate::data::GameData;

pub(super) struct ArchiveTimelineSummary {
    pub(super) recent_milestones: Vec<ArchiveTimelineMilestone>,
    pub(super) total_brews: u32,
    pub(super) known_recipe_count: usize,
    pub(super) experiment_count: usize,
    pub(super) unlocked_route_count: usize,
    pub(super) reconstruction_ready: bool,
}

pub(super) struct ArchiveTimelineMilestone {
    pub(super) title: String,
    pub(super) text: String,
}

impl GameplayState {
    pub(super) fn can_reconstruct_archive(&self, data: &GameData) -> bool {
        data.config
            .archive_required_completed_quests
            .iter()
            .all(|quest_id| self.progression.completed_quests.contains(quest_id))
            && data
                .config
                .archive_required_journal_milestones
                .iter()
                .all(|milestone_id| self.has_journal_milestone(milestone_id))
    }

    pub(super) fn archive_timeline_summary(&self, data: &GameData) -> ArchiveTimelineSummary {
        ArchiveTimelineSummary {
            recent_milestones: self
                .progression
                .journal_milestones
                .iter()
                .rev()
                .take(7)
                .map(|milestone| ArchiveTimelineMilestone {
                    title: milestone.title.clone(),
                    text: milestone.text.clone(),
                })
                .collect(),
            total_brews: self.progression.total_brews,
            known_recipe_count: self.progression.known_recipes.len(),
            experiment_count: self.progression.experiment_log.len(),
            unlocked_route_count: self.progression.unlocked_warps.len(),
            reconstruction_ready: self.can_reconstruct_archive(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    #[test]
    fn the_archive_waits_for_iones_complete_recovered_record() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        for quest_id in &data.config.archive_required_completed_quests {
            if quest_id != "restored_record_for_ione" {
                state.progression.completed_quests.insert(quest_id.clone());
            }
        }
        for milestone_id in &data.config.archive_required_journal_milestones {
            if ![
                "record_reconciled",
                "eleven_months_restored",
                "the_previous_hand",
            ]
            .contains(&milestone_id.as_str())
            {
                state.push_journal_milestone(milestone_id, "", "");
            }
        }

        assert!(!state.can_reconstruct_archive(&data));

        state
            .progression
            .completed_quests
            .insert("restored_record_for_ione".to_owned());
        assert!(
            !state.can_reconstruct_archive(&data),
            "the quest flag alone should not stand in for its recovered evidence"
        );

        state.push_journal_milestone("record_reconciled", "", "");
        assert!(!state.can_reconstruct_archive(&data));
        state.push_journal_milestone("eleven_months_restored", "", "");
        assert!(!state.can_reconstruct_archive(&data));
        state.push_journal_milestone("the_previous_hand", "", "");

        assert!(state.can_reconstruct_archive(&data));
    }
}
