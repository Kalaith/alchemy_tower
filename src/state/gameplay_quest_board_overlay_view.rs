use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::view_models::quest_board::{QuestBoardOverlayEntry, QuestBoardOverlayView};

impl GameplayState {
    pub(super) fn quest_board_overlay_view(&self, data: &GameData) -> QuestBoardOverlayView {
        let entries = self
            .board_actions(data)
            .iter()
            .enumerate()
            .filter_map(|(index, action)| {
                data.quest(&action.quest_id).map(|quest| {
                    let reward = ui_format(
                        "overlay_reward",
                        &[("coins", &quest.reward_coins.to_string())],
                    );
                    QuestBoardOverlayEntry {
                        title: if action.deliver {
                            ui_format("overlay_quest_deliver_title", &[("title", &quest.title)])
                        } else {
                            quest.title.clone()
                        },
                        detail: if action.deliver {
                            ui_copy("overlay_quest_deliver_detail").to_owned()
                        } else {
                            self.quest_location_hint(data, quest)
                        },
                        meta: reward,
                        selected: self.quest_board_entry_selected(index),
                    }
                })
            })
            .collect();

        let locked = self.locked_board_quest_summaries(data);
        let active = self.active_board_quest_titles(data);

        QuestBoardOverlayView {
            title: ui_copy("overlay_quest_board_title").to_owned(),
            subtitle: ui_text().overlays.quest_board_subtitle.clone(),
            available_title: ui_copy("overlay_quest_available").to_owned(),
            empty_text: self.unavailable_state_text(ui_copy("overlay_quest_none_available")),
            entries,
            locked_title: ui_copy("overlay_quest_locked").to_owned(),
            locked_text: if locked.is_empty() {
                ui_copy("overlay_none").to_owned()
            } else {
                locked.join("  ")
            },
            active_title: ui_copy("overlay_quest_active").to_owned(),
            active_text: if active.is_empty() {
                ui_copy("overlay_none").to_owned()
            } else {
                active.join(", ")
            },
            footer_text: quest_board_footer_text(),
        }
    }
}

fn quest_board_footer_text() -> String {
    ui_format(
        "overlay_quest_board_footer",
        &[
            ("select", &input_bindings().navigation.select),
            ("confirm", &input_bindings().global.confirm),
            ("close", &input_bindings().global.cancel),
        ],
    )
}
