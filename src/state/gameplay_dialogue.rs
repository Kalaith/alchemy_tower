use super::GameplayState;
use crate::content::{narrative_text, ui_format};
use crate::data::GameData;
use crate::input::dialogue_advance_pressed;
use macroquad::prelude::Color;

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
            self.push_event_toast_with_icon(
                ui_format("quests_accepted_toast", &[("title", &quest.title)]),
                Color::from_rgba(255, 230, 170, 255),
                "quest_accepted",
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(255, 230, 170, 255),
                false,
                1.2,
            );
            self.runtime.status_text = ui_format(
                "quests_accepted_status",
                &[
                    ("title", &quest.title),
                    ("hint", &self.quest_location_hint(data, quest)),
                ],
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
            self.push_quest_completion_milestone(&quest.id);
            self.push_event_toast_with_icon(
                ui_format("quests_complete_toast", &[("title", &quest.title)]),
                Color::from_rgba(188, 255, 220, 255),
                "quest_complete",
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(188, 255, 220, 255),
                true,
                1.8,
            );
            self.trigger_camera_shake(0.14, 3.8);
            self.runtime.status_text = ui_format(
                "quests_delivered_status",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("coins", &quest.reward_coins.to_string()),
                ],
            );
        } else {
            self.clear_overlay();
        }
    }

    fn push_quest_completion_milestone(&mut self, quest_id: &str) {
        let milestone = match quest_id {
            "healing_for_mira" => Some(&narrative_text().milestones.first_town_relief),
            "cultivation_for_brin" => Some(&narrative_text().milestones.greenhouse_expanded),
            "containment_for_lyra" => Some(&narrative_text().milestones.containment_stable),
            _ => None,
        };
        if let Some(milestone) = milestone {
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
    }

}
