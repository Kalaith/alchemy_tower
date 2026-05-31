use super::GameplayState;
use crate::data::QuestDefinition;

#[path = "gameplay_quest_unlock_text.rs"]
mod quest_unlock_text;

use self::quest_unlock_text::QuestUnlockRequirements;

impl GameplayState {
    pub(super) fn quest_is_available(&self, quest: &QuestDefinition) -> bool {
        quest
            .prerequisite_quests
            .iter()
            .all(|quest_id| self.progression.completed_quests.contains(quest_id))
            && (quest.required_unlocked_warp.is_empty()
                || self
                    .progression
                    .unlocked_warps
                    .contains(&quest.required_unlocked_warp))
            && self.progression.total_brews >= quest.minimum_total_brews
    }

    pub(super) fn quest_unlock_summary(&self, quest: &QuestDefinition) -> String {
        let missing_prereqs = quest
            .prerequisite_quests
            .iter()
            .filter(|quest_id| !self.progression.completed_quests.contains(*quest_id))
            .cloned()
            .collect::<Vec<_>>();
        let missing_warp = !quest.required_unlocked_warp.is_empty()
            && !self
                .progression
                .unlocked_warps
                .contains(&quest.required_unlocked_warp);
        let missing_total_brews = self.progression.total_brews < quest.minimum_total_brews;

        quest_unlock_text::summary(QuestUnlockRequirements {
            missing_prereqs,
            missing_warp,
            missing_total_brews,
            minimum_total_brews: quest.minimum_total_brews,
        })
    }
}
