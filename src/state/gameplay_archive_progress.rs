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
