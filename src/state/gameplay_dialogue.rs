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

        // Advancing is the player having read whatever this townsperson had to
        // say, so their next unsaid line comes up next time rather than being
        // shadowed by a later one forever.
        self.mark_followup_spoken(&npc_id);

        // A townsperson who now counts you a friend hands over their one-time
        // gift before anything else, turning rapport into a felt payoff.
        if self.try_grant_friendship_gift(data, npc) {
            self.clear_overlay();
            return;
        }

        // And the parting gift, once everything they asked for is done.
        if self.try_grant_trusted_gift(data, npc) {
            self.clear_overlay();
            return;
        }

        // Only the live step of this townsperson's arc is on the table; once
        // every step is done there is nothing left to accept or hand over.
        let Some(quest) = self.npc_active_quest(data, npc) else {
            self.clear_overlay();
            return;
        };

        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                self.runtime.status_text = self.quest_unlock_summary(data, quest);
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
            let delivered_rank = self.spend_bottles_for_quest(data, quest, quest.required_amount);
            self.take_from_inventory(&quest.required_item_id, quest.required_amount);
            self.progression.started_quests.remove(&quest.id);
            self.progression.completed_quests.insert(quest.id.clone());
            let bonus = self.quality_bonus_coins(quest, delivered_rank);
            self.coins = self
                .coins
                .saturating_sub(quest.coin_cost)
                .saturating_add(quest.reward_coins + bonus);
            if quest.giver_npc_id != "quest_board" {
                // Work that clearly beat what was asked for is remembered as
                // such. Quality reached the coin purse before it reached
                // anybody's opinion of the player, which was the wrong way
                // round for a game about being the town's alchemist.
                let exceptional = self.delivery_was_exceptional(quest, delivered_rank);
                *self
                    .progression
                    .relationships
                    .entry(quest.giver_npc_id.clone())
                    .or_insert(0) += 2 + i32::from(exceptional);
                if exceptional {
                    self.remark_on_exceptional_delivery(data, &quest.giver_npc_id);
                }
            }
            self.push_quest_completion_milestones(quest);
            self.refresh_available_nodes(data);
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
