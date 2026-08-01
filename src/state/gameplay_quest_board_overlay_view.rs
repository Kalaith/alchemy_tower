use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::view_models::quest_board::{QuestBoardOverlayEntry, QuestBoardOverlayView};

/// The available box is 232px tall and cards sit 64px apart, so three fit. A
/// fourth used to draw straight over the "Locked Requests" heading below it.
const VISIBLE_BOARD_ROWS: usize = 3;

/// The locked box is 54px, which is three lines, and one locked summary already
/// wraps to two of them. Listing every locked request ran the text through both
/// sections beneath it; two summaries still spilled the third line. One summary
/// plus a count is what actually fits, measured rather than assumed.
const VISIBLE_LOCKED_SUMMARIES: usize = 1;

fn locked_summary_text(locked: &[String]) -> String {
    if locked.is_empty() {
        return ui_copy("overlay_none").to_owned();
    }
    let shown = locked
        .iter()
        .take(VISIBLE_LOCKED_SUMMARIES)
        .cloned()
        .collect::<Vec<_>>()
        .join("  ");
    let hidden = locked.len().saturating_sub(VISIBLE_LOCKED_SUMMARIES);
    if hidden == 0 {
        return shown;
    }
    format!(
        "{shown}  {}",
        ui_format(
            "overlay_board_more_locked",
            &[("count", &hidden.to_string())]
        )
    )
}

impl GameplayState {
    pub(super) fn quest_board_overlay_view(&self, data: &GameData) -> QuestBoardOverlayView {
        let actions = self.board_actions(data);
        let total = actions.len();
        let start = visible_window_start(self.ui.shop_index, total, VISIBLE_BOARD_ROWS);
        let range_text = (total > VISIBLE_BOARD_ROWS).then(|| {
            ui_format(
                "overlay_board_range",
                &[
                    ("first", &(start + 1).to_string()),
                    ("last", &(start + VISIBLE_BOARD_ROWS).min(total).to_string()),
                    ("total", &total.to_string()),
                ],
            )
        });
        let entries = actions
            .iter()
            .enumerate()
            .skip(start)
            .take(VISIBLE_BOARD_ROWS)
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
            range_text,
            entries,
            locked_title: ui_copy("overlay_quest_locked").to_owned(),
            locked_text: locked_summary_text(&locked),
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

#[cfg(test)]
mod tests {
    use super::{locked_summary_text, GameplayState, VISIBLE_BOARD_ROWS};

    /// The locked box is 836px wide at 16px, which is about 95 characters, and
    /// 54px tall at an 18px line height, which is three lines. Measured against
    /// captures: every locked request at once ran through two sections below it,
    /// and even two summaries spilled the third line.
    const LOCKED_TEXT_BUDGET: usize = 280;

    #[test]
    fn the_locked_list_stays_inside_its_box() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);
        // A fresh save is the worst case: nearly every board request is locked.
        let locked = state.locked_board_quest_summaries(&data);
        assert!(
            locked.len() > 1,
            "this only proves anything while several requests are locked"
        );

        let text = locked_summary_text(&locked);
        assert!(
            text.chars().count() <= LOCKED_TEXT_BUDGET,
            "locked list is {} characters, over the {LOCKED_TEXT_BUDGET} its box shows:\n{text}",
            text.chars().count()
        );
        assert!(
            text.contains(&(locked.len() - 1).to_string()),
            "the summary must say how many it is not showing, not drop them silently"
        );
    }

    #[test]
    fn the_available_list_never_draws_more_cards_than_fit() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);
        let view = state.quest_board_overlay_view(&data);
        assert!(
            view.entries.len() <= VISIBLE_BOARD_ROWS,
            "{} cards drawn into a box that holds {VISIBLE_BOARD_ROWS}",
            view.entries.len()
        );
    }
}
