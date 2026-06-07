use super::GameplayState;
use crate::data::GameData;
use crate::input::{cancel_pressed, confirm_pressed, select_next_pressed, select_previous_pressed};

#[path = "gameplay_quest_board_text.rs"]
mod quest_board_text;

impl GameplayState {
    pub(super) fn handle_quest_board_inputs(&mut self, data: &GameData) {
        if cancel_pressed() {
            self.clear_overlay();
            self.runtime.status_text = quest_board_text::closed();
            return;
        }
        let available = self.available_board_quests(data);
        if available.is_empty() {
            return;
        }
        if select_previous_pressed() {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if select_next_pressed() {
            self.ui.shop_index = (self.ui.shop_index + 1).min(available.len().saturating_sub(1));
        }
        if confirm_pressed() {
            if let Some(quest_id) = available.get(self.ui.shop_index) {
                self.progression.started_quests.insert(quest_id.clone());
                if let Some(quest) = data.quest(quest_id) {
                    self.trigger_quest_accepted_feedback(quest_board_text::accepted_toast(quest));
                }
                self.runtime.status_text = self.quest_board_accept_status(data, quest_id);
            }
        }
    }

    fn quest_board_accept_status(&self, data: &GameData, quest_id: &str) -> String {
        data.quest(quest_id)
            .map(|quest| {
                quest_board_text::accepted_status(quest, &self.quest_location_hint(data, quest))
            })
            .unwrap_or_else(quest_board_text::accepted_default)
    }

    pub(super) fn available_board_quests(&self, data: &GameData) -> Vec<String> {
        data.quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| self.quest_is_available(quest))
            .map(|quest| quest.id.clone())
            .collect()
    }

    pub(super) fn locked_board_quest_summaries(&self, data: &GameData) -> Vec<String> {
        data.quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| !self.quest_is_available(quest))
            .map(|quest| {
                let requirements = self.locked_state_text(&self.quest_unlock_summary(quest));
                quest_board_text::locked_line(quest, &requirements)
            })
            .collect()
    }

    pub(super) fn active_board_quest_titles(&self, data: &GameData) -> Vec<String> {
        self.progression
            .started_quests
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .map(|quest| quest.title.clone())
            .collect()
    }
}
