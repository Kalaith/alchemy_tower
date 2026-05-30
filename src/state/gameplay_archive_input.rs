use super::gameplay_overlay_types::ARCHIVE_TABS;
use super::GameplayState;
use crate::content::{narrative_text, ui_copy, ui_text};
use crate::data::GameData;
use crate::input::{
    archive_filter_pressed, cancel_pressed, confirm_pressed, select_next_pressed, select_previous_pressed,
    switch_next_pressed, switch_previous_pressed,
};
use macroquad::prelude::Color;

impl GameplayState {
    pub(super) fn handle_archive_inputs(&mut self, data: &GameData) {
        if switch_previous_pressed() {
            self.ui.archive_tab = self.ui.archive_tab.saturating_sub(1);
            self.ui.archive_index = 0;
        }
        if switch_next_pressed() {
            self.ui.archive_tab =
                (self.ui.archive_tab + 1).min(ARCHIVE_TABS.len().saturating_sub(1));
            self.ui.archive_index = 0;
        }

        let selection_len = self.archive_selection_len(data);
        if selection_len > 0 {
            if select_previous_pressed() {
                self.ui.archive_index = self.ui.archive_index.saturating_sub(1);
            }
            if select_next_pressed() {
                self.ui.archive_index =
                    (self.ui.archive_index + 1).min(selection_len.saturating_sub(1));
            }
        } else {
            self.ui.archive_index = 0;
        }

        if ARCHIVE_TABS[self.ui.archive_tab] == "experiments" && archive_filter_pressed() {
            self.cycle_archive_experiment_filter();
        }

        if confirm_pressed() {
            match ARCHIVE_TABS[self.ui.archive_tab] {
                "timeline" => self.handle_archive_timeline_submit(),
                "disassembly" => {
                    let recipes = self.available_disassembly_recipes(data);
                    if let Some(recipe) = recipes.get(self.ui.archive_index).copied() {
                        self.disassemble_recipe(data, recipe);
                    }
                }
                "duplication" => {
                    let items = self.duplication_candidates(data);
                    if let Some(item_id) = items.get(self.ui.archive_index) {
                        self.duplicate_item(data, item_id);
                    }
                }
                _ => {}
            }
        }
        if cancel_pressed() {
            self.clear_overlay();
            self.runtime.status_text = ui_text().statuses.closed_archive.clone();
        }
    }

    fn handle_archive_timeline_submit(&mut self) {
        if self.can_reconstruct_archive() {
            let milestone = &narrative_text().milestones.archive_revelation;
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            self.push_event_toast_with_icon(
                ui_copy("archive_timeline_restored_toast"),
                Color::from_rgba(176, 226, 255, 255),
                "journal_note",
            );
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(176, 226, 255, 255),
                true,
                2.2,
            );
            self.trigger_camera_shake(0.2, 5.2);
            self.runtime.status_text =
                narrative_text().statuses.archive_timeline_complete.clone();
        } else {
            self.runtime.status_text = narrative_text()
                .statuses
                .archive_timeline_incomplete
                .clone();
        }
    }
}
