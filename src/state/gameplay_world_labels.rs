use super::GameplayState;
use crate::content::ui_copy;
use crate::data::{GameData, NpcDefinition, StationDefinition, StationKind};
use macroquad::prelude::Color;

impl GameplayState {
    pub(super) fn station_world_label(
        &self,
        data: &GameData,
        station: &StationDefinition,
    ) -> Option<(String, Color)> {
        match station.kind {
            StationKind::Alchemy if self.progression.total_brews < 3 => Some((
                ui_copy("world_marker_brew_here").to_owned(),
                Color::from_rgba(188, 255, 220, 255),
            )),
            StationKind::QuestBoard if !self.available_board_quests(data).is_empty() => Some((
                ui_copy("world_marker_new_requests").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            )),
            StationKind::ArchiveConsole
                if self.can_reconstruct_archive()
                    && !self.has_journal_milestone("archive_revelation") =>
            {
                Some((
                    ui_copy("world_marker_rebuild_ready").to_owned(),
                    Color::from_rgba(176, 226, 255, 255),
                ))
            }
            StationKind::Planter => self
                .progression
                .planter_states
                .get(&station.id)
                .filter(|state| state.ready)
                .map(|_| {
                    (
                        ui_copy("world_marker_harvest_ready").to_owned(),
                        Color::from_rgba(188, 255, 220, 255),
                    )
                }),
            StationKind::Habitat => self
                .progression
                .habitat_states
                .get(&station.id)
                .filter(|state| {
                    !state.creature_item_id.is_empty()
                        && self.world.day_index
                            >= state
                                .last_harvest_day
                                .saturating_add(station.habitat_harvest_days.max(1))
                })
                .map(|_| {
                    (
                        ui_copy("world_marker_collect_ready").to_owned(),
                        Color::from_rgba(188, 255, 220, 255),
                    )
                }),
            StationKind::EndingFocus if self.has_journal_milestone("archive_revelation") => Some((
                ui_copy("world_marker_final_focus").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            )),
            _ => None,
        }
    }

    pub(super) fn npc_world_label(
        &self,
        data: &GameData,
        npc: &NpcDefinition,
    ) -> Option<(String, Color)> {
        let quest = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten()?;
        if self.progression.completed_quests.contains(&quest.id) {
            return None;
        }
        if self.progression.started_quests.contains(&quest.id) {
            if self.quest_requirements_met(data, quest) {
                Some((
                    ui_copy("world_marker_turn_in").to_owned(),
                    Color::from_rgba(188, 255, 220, 255),
                ))
            } else {
                Some((
                    ui_copy("world_marker_awaiting_brew").to_owned(),
                    Color::from_rgba(255, 230, 170, 255),
                ))
            }
        } else if self.quest_is_available(quest) {
            Some((
                ui_copy("world_marker_request").to_owned(),
                Color::from_rgba(255, 230, 170, 255),
            ))
        } else {
            None
        }
    }
}
