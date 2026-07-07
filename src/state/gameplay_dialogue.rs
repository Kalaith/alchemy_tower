use super::GameplayState;
use crate::data::{GameData, QuestDefinition};
use crate::input::dialogue_advance_pressed;

#[path = "gameplay_dialogue_quest_text.rs"]
mod dialogue_quest_text;

impl GameplayState {
    pub(super) fn handle_dialogue_inputs(&mut self, data: &GameData) {
        if !dialogue_advance_pressed() {
            return;
        }

        let Some(npc_id) = self.dialogue_npc_id().map(str::to_owned) else {
            self.clear_overlay();
            return;
        };
        let Some(npc) = data.npc(&npc_id) else {
            self.clear_overlay();
            return;
        };

        // A townsperson who now counts you a friend hands over their one-time
        // gift before anything else, turning rapport into a felt payoff.
        if self.try_grant_friendship_gift(data, npc) {
            self.clear_overlay();
            return;
        }

        if npc.quest_id.is_empty() {
            self.clear_overlay();
            return;
        }

        let Some(quest) = data.quest(&npc.quest_id) else {
            self.clear_overlay();
            return;
        };

        if self.progression.completed_quests.contains(&quest.id) {
            self.clear_overlay();
            return;
        }

        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                self.runtime.status_text = self.quest_unlock_summary(quest);
                return;
            }
            self.progression.started_quests.insert(quest.id.clone());
            *self
                .progression
                .relationships
                .entry(npc.id.clone())
                .or_insert(0) += 1;
            self.trigger_quest_accepted_feedback(dialogue_quest_text::accepted_toast(&quest.title));
            self.runtime.status_text = dialogue_quest_text::accepted_status(
                data,
                quest,
                &self.quest_location_hint(data, quest),
            );
            return;
        }

        if self.quest_requirements_met(data, quest) {
            if let Some(amount) = self.inventory.get_mut(&quest.required_item_id) {
                *amount -= quest.required_amount;
            }
            self.inventory.retain(|_, amount| *amount > 0);
            self.progression.started_quests.remove(&quest.id);
            self.progression.completed_quests.insert(quest.id.clone());
            self.coins += quest.reward_coins;
            if quest.giver_npc_id != "quest_board" {
                *self
                    .progression
                    .relationships
                    .entry(quest.giver_npc_id.clone())
                    .or_insert(0) += 2;
            }
            self.push_quest_completion_milestones(quest);
            self.trigger_quest_complete_feedback(dialogue_quest_text::complete_toast(&quest.title));
            self.runtime.status_text = dialogue_quest_text::delivered_status(data, quest);
        } else {
            self.clear_overlay();
        }
    }

    pub(super) fn push_quest_completion_milestones(&mut self, quest: &QuestDefinition) {
        for milestone in &quest.completion_milestones {
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
    }
}
