use crate::content::ui_format;
use crate::data::{GameData, QuestDefinition};

pub(super) fn accepted_toast(quest_title: &str) -> String {
    ui_format("quests_accepted_toast", &[("title", quest_title)])
}

pub(super) fn accepted_status(
    _data: &GameData,
    quest: &QuestDefinition,
    location_hint: &str,
) -> String {
    ui_format(
        "quests_accepted_status",
        &[("title", &quest.title), ("hint", location_hint)],
    )
}

pub(super) fn complete_toast(quest_title: &str) -> String {
    ui_format("quests_complete_toast", &[("title", quest_title)])
}

pub(super) fn delivered_status(data: &GameData, quest: &QuestDefinition) -> String {
    ui_format(
        "quests_delivered_status",
        &[
            ("item", data.item_name(&quest.required_item_id)),
            ("coins", &quest.reward_coins.to_string()),
        ],
    )
}
