use super::GameplayState;
use crate::content::{input_bindings, ui_format};
use crate::data::{GameData, NpcDefinition};

impl GameplayState {
    pub(super) fn current_dialogue_text(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let dialogue = self.npc_dialogue_selection(data, npc);
        let Some(quest) = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()
        else {
            return self.append_npc_story_line(&npc.id, dialogue.complete.to_owned());
        };

        if self.progression.completed_quests.contains(&quest.id) {
            return self.append_npc_story_line(&npc.id, dialogue.complete.to_owned());
        }
        if !self.progression.started_quests.contains(&quest.id) {
            // Whether or not the quest is available yet, the NPC just speaks in
            // their own voice. Lock reasons and rewards belong in the footer, not
            // dumped into the middle of the conversation.
            return self.append_npc_story_line(&npc.id, dialogue.start.to_owned());
        }

        if self.quest_requirements_met(data, quest) {
            self.append_npc_story_line(
                &npc.id,
                ui_format(
                    "quests_dialogue_smell",
                    &[
                        ("progress", dialogue.progress),
                        ("item", data.item_name(&quest.required_item_id)),
                    ],
                ),
            )
        } else {
            self.append_npc_story_line(&npc.id, dialogue.progress.to_owned())
        }
    }

    pub(super) fn current_dialogue_footer(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(quest) = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()
        else {
            return dialogue_footer_text("quests_dialogue_footer_default", &[]);
        };

        if self.progression.completed_quests.contains(&quest.id) {
            return dialogue_footer_text("quests_dialogue_footer_default", &[]);
        }
        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                return self.locked_state_text(&self.quest_unlock_summary(quest));
            }
            return dialogue_footer_text(
                "quests_dialogue_footer_reward",
                &[("coins", &quest.reward_coins.to_string())],
            );
        }

        if self.quest_requirements_met(data, quest) {
            dialogue_footer_text(
                "quests_dialogue_footer_delivery",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                    ("coins", &quest.reward_coins.to_string()),
                ],
            )
        } else {
            dialogue_footer_text(
                "quests_dialogue_footer_unavailable",
                &[
                    (
                        "requirements",
                        &self.unavailable_state_text(&self.quest_requirement_summary(data, quest)),
                    ),
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                ],
            )
        }
    }
}

fn dialogue_footer_text(copy_key: &str, replacements: &[(&str, &str)]) -> String {
    let mut pairs = Vec::with_capacity(replacements.len() + 2);
    pairs.extend_from_slice(replacements);
    pairs.push(("confirm", input_bindings().global.confirm.as_str()));
    pairs.push(("close", input_bindings().global.cancel.as_str()));
    ui_format(copy_key, &pairs)
}
