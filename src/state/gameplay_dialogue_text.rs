use super::GameplayState;
use crate::content::ui_format;
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
            if !self.quest_is_available(quest) {
                return self.append_npc_story_line(
                    &npc.id,
                    format!("{} {}", dialogue.start, self.quest_unlock_summary(quest)),
                );
            }
            return self.append_npc_story_line(
                &npc.id,
                format!("{} {}", dialogue.start, self.npc_context_line(data, npc)),
            );
        }

        if self.quest_requirements_met(data, quest) {
            self.append_npc_story_line(
                &npc.id,
                ui_format(
                    "quests_dialogue_smell",
                    &[
                        ("progress", dialogue.progress),
                        ("context", &self.npc_context_line(data, npc)),
                        ("item", data.item_name(&quest.required_item_id)),
                    ],
                ),
            )
        } else {
            self.append_npc_story_line(
                &npc.id,
                ui_format(
                    "quests_dialogue_requirements",
                    &[
                        ("progress", dialogue.progress),
                        ("context", &self.npc_context_line(data, npc)),
                        ("requirements", &self.quest_requirement_summary(data, quest)),
                    ],
                ),
            )
        }
    }

    pub(super) fn current_dialogue_footer(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(quest) = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()
        else {
            return ui_format("quests_dialogue_footer_default", &[]);
        };

        if self.progression.completed_quests.contains(&quest.id) {
            return ui_format("quests_dialogue_footer_default", &[]);
        }
        if !self.progression.started_quests.contains(&quest.id) {
            if !self.quest_is_available(quest) {
                return self.locked_state_text(&self.quest_unlock_summary(quest));
            }
            return ui_format(
                "quests_dialogue_footer_reward",
                &[("coins", &quest.reward_coins.to_string())],
            );
        }

        if self.quest_requirements_met(data, quest) {
            ui_format(
                "quests_dialogue_footer_delivery",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                    ("coins", &quest.reward_coins.to_string()),
                ],
            )
        } else {
            ui_format(
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
