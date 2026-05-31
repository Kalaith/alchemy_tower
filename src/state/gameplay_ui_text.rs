use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::GameData;

impl GameplayState {
    pub(super) fn item_card_meta(
        &self,
        data: &GameData,
        item_id: &str,
        amount: u32,
        extra: &str,
    ) -> String {
        let item = data.item(item_id);
        let quality = item
            .map(|item| item.quality)
            .unwrap_or_default()
            .to_string();
        let rarity = item.map(|item| item.rarity).unwrap_or_default().to_string();
        let amount = amount.to_string();
        let base = ui_format(
            "inventory_item_meta",
            &[
                ("category", item.map(|item| item.category.as_str()).unwrap_or("?")),
                ("quality", &quality),
                ("rarity", &rarity),
                ("amount", &amount),
            ],
        );
        if extra.is_empty() {
            base
        } else {
            ui_format("inventory_item_meta_extra", &[("base", &base), ("extra", extra)])
        }
    }

    pub(super) fn locked_state_text(&self, detail: &str) -> String {
        ui_format("locked_prefix", &[("detail", detail)])
    }

    pub(super) fn unavailable_state_text(&self, detail: &str) -> String {
        ui_format("unavailable_prefix", &[("detail", detail)])
    }

    pub(super) fn interact_prompt_copy(
        &self,
        copy_key: &str,
        replacements: &[(&str, &str)],
    ) -> String {
        let mut pairs = Vec::with_capacity(replacements.len() + 1);
        pairs.extend_from_slice(replacements);
        pairs.push(("interact", input_bindings().global.interact.as_str()));
        ui_format(copy_key, &pairs)
    }

    pub(super) fn alchemy_prompt_copy(&self, label: &str) -> String {
        self.interact_prompt_copy(
            "world_prompt_alchemy",
            &[("alchemy", input_bindings().alchemy.open.as_str()), ("label", label)],
        )
    }

    pub(super) fn next_goal_summary(&self, data: &GameData) -> String {
        if let Some(quest) = self
            .progression
            .started_quests
            .iter()
            .find_map(|quest_id| data.quest(quest_id))
        {
            return ui_format(
                "goal_active_quest",
                &[
                    ("title", &quest.title),
                    ("requirements", &self.quest_requirement_summary(data, quest)),
                ],
            );
        }

        if self.can_reconstruct_archive() && !self.has_journal_milestone("archive_revelation") {
            return ui_copy("goal_reconstruct_archive").to_owned();
        }

        if let Some(quest) = data
            .quests
            .iter()
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .find(|quest| self.quest_is_available(quest))
        {
            return ui_format(
                "goal_accept_quest",
                &[
                    ("title", &quest.title),
                    ("location", &self.quest_location_hint(data, quest)),
                ],
            );
        }

        if let Some(warp) = self.next_locked_warp(data) {
            return ui_format(
                "goal_restore_route",
                &[
                    ("label", &warp.label),
                    ("requirements", &self.warp_requirement_summary(data, warp)),
                ],
            );
        }

        ui_copy("goal_keep_working").to_owned()
    }
}
