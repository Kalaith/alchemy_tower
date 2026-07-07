use crate::content::ui_format;
use crate::data::{GameData, NpcDefinition};

pub(super) fn friendship_toast(npc_name: &str) -> String {
    ui_format("rapport_friend_toast", &[("name", npc_name)])
}

pub(super) fn friendship_status(data: &GameData, npc: &NpcDefinition) -> String {
    let mut reward = Vec::new();
    if npc.friendship_reward_coins > 0 {
        reward.push(ui_format(
            "rapport_friend_reward_coins",
            &[("coins", &npc.friendship_reward_coins.to_string())],
        ));
    }
    if !npc.friendship_reward_item_id.is_empty() && npc.friendship_reward_amount > 0 {
        reward.push(ui_format(
            "rapport_friend_reward_item",
            &[
                ("item", data.item_name(&npc.friendship_reward_item_id)),
                ("amount", &npc.friendship_reward_amount.to_string()),
            ],
        ));
    }

    if reward.is_empty() {
        ui_format("rapport_friend_status_bare", &[("name", &npc.name)])
    } else {
        ui_format(
            "rapport_friend_status",
            &[("name", &npc.name), ("reward", &reward.join(", "))],
        )
    }
}
