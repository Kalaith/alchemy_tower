use super::GameplayState;
use crate::content::ui_format;
use crate::data::QuestDefinition;

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
        let mut reasons = Vec::new();
        let missing_prereqs = quest
            .prerequisite_quests
            .iter()
            .filter(|quest_id| !self.progression.completed_quests.contains(*quest_id))
            .cloned()
            .collect::<Vec<_>>();
        if !missing_prereqs.is_empty() {
            reasons.push(ui_format(
                "quests_unlock_finish",
                &[("quests", &missing_prereqs.join(", "))],
            ));
        }
        if !quest.required_unlocked_warp.is_empty()
            && !self
                .progression
                .unlocked_warps
                .contains(&quest.required_unlocked_warp)
        {
            reasons.push(ui_format("quests_unlock_greenhouse", &[]));
        }
        if self.progression.total_brews < quest.minimum_total_brews {
            reasons.push(ui_format(
                "quests_unlock_brews",
                &[("brews", &quest.minimum_total_brews.to_string())],
            ));
        }
        if reasons.is_empty() {
            ui_format("quests_unlock_closed", &[])
        } else {
            ui_format(
                "quests_unlock_after",
                &[("reasons", &reasons.join(" and "))],
            )
        }
    }
}
