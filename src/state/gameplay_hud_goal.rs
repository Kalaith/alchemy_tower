use super::GameplayState;
use crate::view_models::hud::HudGoal;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, QuestDefinition};

impl GameplayState {
    pub(super) fn hud_goal(&self, data: &GameData) -> HudGoal {
        if let Some(goal) = self.active_hud_quest_goal(data) {
            return goal;
        }

        if self.can_reconstruct_archive() && !self.has_journal_milestone("archive_revelation") {
            return HudGoal {
                title: ui_copy("hud_goal_archive_title").to_owned(),
                body: ui_copy("goal_reconstruct_archive").to_owned(),
                detail: String::new(),
                action: String::new(),
                icon_id: None,
                amount_text: String::new(),
            };
        }

        if let Some(quest) = data
            .quests
            .iter()
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .find(|quest| self.quest_is_available(quest))
        {
            let location = self.quest_hud_location(data, quest);
            return HudGoal {
                title: quest.title.clone(),
                body: quest.description.clone(),
                detail: ui_format("hud_goal_accept_from", &[("location", &location)]),
                action: ui_format(
                    "hud_goal_item_need",
                    &[
                        ("item", data.item_name(&quest.required_item_id)),
                        ("amount", &quest.required_amount.to_string()),
                    ],
                ),
                icon_id: Some(quest.required_item_id.clone()),
                amount_text: ui_format(
                    "hud_goal_item_amount",
                    &[("amount", &quest.required_amount.to_string())],
                ),
            };
        }

        if let Some(warp) = self.next_locked_warp(data) {
            return HudGoal {
                title: ui_copy("hud_goal_restore_title").to_owned(),
                body: warp.label.clone(),
                detail: ui_format(
                    "hud_goal_need",
                    &[("requirements", &self.warp_requirement_summary(data, warp))],
                ),
                action: String::new(),
                icon_id: None,
                amount_text: String::new(),
            };
        }

        HudGoal {
            title: ui_copy("hud_goal_work_title").to_owned(),
            body: ui_copy("goal_keep_working").to_owned(),
            detail: String::new(),
            action: String::new(),
            icon_id: None,
            amount_text: String::new(),
        }
    }

    fn active_hud_quest_goal(&self, data: &GameData) -> Option<HudGoal> {
        let quest = self
            .progression
            .started_quests
            .iter()
            .find_map(|quest_id| data.quest(quest_id))?;
        let location = self.quest_hud_location(data, quest);
        let requirements = self.quest_requirement_summary(data, quest);
        let action = if self.quest_requirements_met(data, quest) {
            ui_format("hud_goal_ready_to_deliver", &[("location", &location)])
        } else {
            ui_format(
                "hud_goal_item_need_with_requirements",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                    ("requirements", &requirements),
                ],
            )
        };

        Some(HudGoal {
            title: quest.title.clone(),
            body: quest.description.clone(),
            detail: ui_format("hud_goal_find", &[("location", &location)]),
            action,
            icon_id: Some(quest.required_item_id.clone()),
            amount_text: ui_format(
                "hud_goal_item_amount",
                &[("amount", &quest.required_amount.to_string())],
            ),
        })
    }

    fn quest_hud_location(&self, data: &GameData, quest: &QuestDefinition) -> String {
        let Some(npc) = data.npc(&quest.giver_npc_id) else {
            return ui_copy("hud_goal_request_board").to_owned();
        };
        let runtime = self.npc_runtime_state(data, npc);
        let area_name = data
            .area(&runtime.area_id)
            .map(|area| area.name.as_str())
            .unwrap_or(ui_copy("npc_hint_somewhere"));
        ui_format(
            "hud_goal_meet_npc",
            &[("npc", &npc.name), ("area", area_name)],
        )
    }
}
