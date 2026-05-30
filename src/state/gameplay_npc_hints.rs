use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, NpcDefinition, QuestDefinition};

impl GameplayState {
    pub(super) fn npc_schedule_area_for_time<'a>(
        &self,
        npc: &'a NpcDefinition,
        time_window: &str,
    ) -> Option<&'a str> {
        npc.schedule
            .iter()
            .find(|entry| entry.time_window == time_window)
            .map(|entry| entry.area_id.as_str())
    }

    pub(super) fn npc_now_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let runtime = self.npc_runtime_state(data, npc);
        let area_name = data
            .area(&runtime.area_id)
            .map(|area| area.name.as_str())
            .unwrap_or(ui_copy("npc_hint_somewhere"));
        if runtime.moving {
            let target_name = runtime
                .target_area_id
                .as_ref()
                .and_then(|area_id| data.area(area_id))
                .map(|area| area.name.as_str())
                .unwrap_or(area_name);
            ui_format(
                "npc_travelling",
                &[("area", area_name), ("target", target_name)],
            )
        } else {
            ui_format(
                "npc_hint_here_now",
                &[("area", area_name), ("time", self.current_time_window())],
            )
        }
    }

    pub(super) fn npc_later_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(current_index) = self.active_schedule_index(npc) else {
            return ui_copy("npc_hint_routine_unclear").to_owned();
        };
        let later_windows = ["day", "evening", "night", "morning"];
        let current_window = self.current_time_window();
        let next_window = later_windows
            .iter()
            .skip_while(|window| **window != current_window)
            .nth(1)
            .copied()
            .unwrap_or("morning");
        let later_area = self
            .npc_schedule_area_for_time(npc, next_window)
            .and_then(|area_id| data.area(area_id))
            .map(|area| area.name.as_str())
            .unwrap_or_else(|| {
                let next_index = (current_index + 1) % npc.schedule.len();
                data.area(&npc.schedule[next_index].area_id)
                    .map(|area| area.name.as_str())
                    .unwrap_or(ui_copy("npc_hint_unknown"))
            });
        ui_format(
            "npc_hint_later",
            &[("area", later_area), ("time", next_window)],
        )
    }

    pub(super) fn npc_usual_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        ["morning", "day", "evening"]
            .iter()
            .filter_map(|time_window| {
                self.npc_schedule_area_for_time(npc, time_window)
                    .and_then(|area_id| data.area(area_id))
                    .map(|area| {
                        ui_format(
                            "npc_hint_usual",
                            &[("time", time_window), ("area", &area.name)],
                        )
                    })
            })
            .collect::<Vec<_>>()
            .join("  |  ")
    }

    pub(super) fn quest_location_hint(&self, data: &GameData, quest: &QuestDefinition) -> String {
        data.npc(&quest.giver_npc_id)
            .map(|npc| {
                ui_format(
                    "npc_quest_location",
                    &[
                        ("now", &self.npc_now_hint(data, npc)),
                        ("later", &self.npc_later_hint(data, npc)),
                    ],
                )
            })
            .unwrap_or_else(|| ui_copy("npc_quest_location_fallback").to_owned())
    }

    pub(super) fn npc_context_line(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let relationship = self
            .progression
            .relationships
            .get(&npc.id)
            .copied()
            .unwrap_or_default();
        let base = ui_format(
            "npc_context_line",
            &[
                ("now", &self.npc_now_hint(data, npc)),
                ("later", &self.npc_later_hint(data, npc)),
                ("usual", &self.npc_usual_hint(data, npc)),
                ("rapport", &relationship.to_string()),
                (
                    "role",
                    if npc.role.is_empty() {
                        ui_copy("overlay_rapport_empty")
                    } else {
                        npc.role.as_str()
                    },
                ),
            ],
        );
        self.append_npc_story_line(&npc.id, base)
    }
}
