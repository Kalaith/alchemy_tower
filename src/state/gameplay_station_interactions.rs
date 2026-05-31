use super::gameplay_overlay_types::OverlayScreen;
use super::GameplayState;
use crate::content::narrative_text;
use crate::data::{GameData, StationDefinition, StationKind};

#[path = "gameplay_station_interaction_text.rs"]
mod station_interaction_text;

impl GameplayState {
    pub(super) fn handle_station_interaction(
        &mut self,
        data: &GameData,
        station: &StationDefinition,
    ) -> bool {
        match station.kind {
            StationKind::Shop => {
                self.set_overlay(OverlayScreen::Shop);
                self.ui.shop_buy_tab = true;
                self.ui.shop_index = 0;
                self.runtime.status_text = station_interaction_text::opened_station(&station.name);
                true
            }
            StationKind::RuneWorkshop => {
                self.set_overlay(OverlayScreen::Rune);
                self.ui.rune_index = 0;
                self.runtime.status_text = station_interaction_text::opened_station(&station.name);
                true
            }
            StationKind::ArchiveConsole => {
                self.set_overlay(OverlayScreen::Archive);
                self.ui.archive_tab = 0;
                self.ui.archive_index = 0;
                self.runtime.status_text = station_interaction_text::opened_station(&station.name);
                true
            }
            StationKind::EndingFocus => {
                self.handle_ending_focus_interaction();
                true
            }
            StationKind::QuestBoard => {
                self.set_overlay(OverlayScreen::QuestBoard);
                self.ui.shop_index = 0;
                self.runtime.status_text = station_interaction_text::reading_quest_board();
                true
            }
            StationKind::RestBed => {
                self.sleep_until(data, 7.0 * 60.0, false);
                true
            }
            StationKind::Planter => {
                self.interact_with_planter(data, station);
                true
            }
            StationKind::Habitat => {
                self.interact_with_habitat(data, station);
                true
            }
            _ => false,
        }
    }

    fn handle_ending_focus_interaction(&mut self) {
        if self.has_journal_milestone("archive_revelation") {
            self.set_overlay(OverlayScreen::Ending);
            let milestone = &narrative_text().milestones.observatory_ending;
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            self.runtime.status_text = station_interaction_text::observatory_aligned();
        } else {
            self.runtime.status_text = station_interaction_text::observatory_locked();
        }
    }
}
