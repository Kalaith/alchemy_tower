use super::GameplayState;
use crate::content::{ui_format, ui_text};
use crate::data::GameData;
use macroquad::prelude::{is_key_pressed, Color, KeyCode};

impl GameplayState {
    pub(super) fn handle_quest_board_inputs(&mut self, data: &GameData) {
        if is_key_pressed(KeyCode::Escape) {
            self.clear_overlay();
            self.runtime.status_text = ui_text().statuses.closed_quest_board.clone();
            return;
        }
        let available = self.available_board_quests(data);
        if available.is_empty() {
            return;
        }
        if is_key_pressed(KeyCode::Up) {
            self.ui.shop_index = self.ui.shop_index.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            self.ui.shop_index = (self.ui.shop_index + 1).min(available.len().saturating_sub(1));
        }
        if is_key_pressed(KeyCode::Enter) {
            if let Some(quest_id) = available.get(self.ui.shop_index) {
                self.progression.started_quests.insert(quest_id.clone());
                if let Some(quest) = data.quest(quest_id) {
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
                }
                self.runtime.status_text = data
                    .quest(quest_id)
                    .map(|quest| {
                        ui_format(
                            "quests_board_accepted_status",
                            &[
                                ("title", &quest.title),
                                ("hint", &self.quest_location_hint(data, quest)),
                            ],
                        )
                    })
                    .unwrap_or_else(|| ui_format("quests_board_accepted_default", &[]));
            }
        }
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
}
