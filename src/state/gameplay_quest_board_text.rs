use crate::content::{ui_format, ui_text};
use crate::data::QuestDefinition;

pub(super) fn closed() -> String {
    ui_text().statuses.closed_quest_board.clone()
}

pub(super) fn accepted_toast(quest: &QuestDefinition) -> String {
    ui_format("quests_accepted_toast", &[("title", &quest.title)])
}

pub(super) fn accepted_status(quest: &QuestDefinition, location_hint: &str) -> String {
    ui_format(
        "quests_board_accepted_status",
        &[("title", &quest.title), ("hint", location_hint)],
    )
}

pub(super) fn accepted_default() -> String {
    ui_format("quests_board_accepted_default", &[])
}

pub(super) fn locked_line(quest: &QuestDefinition, requirements: &str) -> String {
    ui_format(
        "overlay_quest_locked_line",
        &[("title", &quest.title), ("requirements", requirements)],
    )
}
