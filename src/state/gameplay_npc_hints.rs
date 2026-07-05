use super::GameplayState;
use crate::data::{GameData, NpcDefinition, QuestDefinition};

#[path = "gameplay_npc_hint_text.rs"]
mod npc_hint_text;

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
            .unwrap_or(npc_hint_text::somewhere());
        if runtime.moving {
            let target_name = runtime
                .target_area_id
                .as_ref()
                .and_then(|area_id| data.area(area_id))
                .map(|area| area.name.as_str())
                .unwrap_or(area_name);
            npc_hint_text::travelling(area_name, target_name)
        } else {
            npc_hint_text::here_now(area_name, self.current_time_window())
        }
    }

    pub(super) fn npc_later_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        let Some(current_index) = self.active_schedule_index(npc) else {
            return npc_hint_text::routine_unclear();
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
                    .unwrap_or(npc_hint_text::unknown())
            });
        npc_hint_text::later(later_area, next_window)
    }

    pub(super) fn npc_usual_hint(&self, data: &GameData, npc: &NpcDefinition) -> String {
        ["morning", "day", "evening"]
            .iter()
            .filter_map(|time_window| {
                self.npc_schedule_area_for_time(npc, time_window)
                    .and_then(|area_id| data.area(area_id))
                    .map(|area| npc_hint_text::usual(time_window, &area.name))
            })
            .collect::<Vec<_>>()
            .join("  |  ")
    }

    pub(super) fn quest_location_hint(&self, data: &GameData, quest: &QuestDefinition) -> String {
        data.npc(&quest.giver_npc_id)
            .map(|npc| {
                npc_hint_text::quest_location(
                    &self.npc_now_hint(data, npc),
                    &self.npc_later_hint(data, npc),
                )
            })
            .unwrap_or_else(npc_hint_text::quest_location_fallback)
    }
}
