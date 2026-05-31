use super::GameplayState;
use crate::content::{narrative_text, ui_copy, ui_format};
use crate::view_models::archive::{ArchiveTimelineMilestoneView, ArchiveTimelineSectionView};

impl GameplayState {
    pub(super) fn archive_timeline_section_view(&self) -> ArchiveTimelineSectionView {
        let summary = self.archive_timeline_summary();
        let reconstruction_text = if summary.reconstruction_ready {
            narrative_text()
                .statuses
                .archive_reconstruction_ready
                .clone()
        } else {
            self.locked_state_text(&narrative_text().statuses.archive_reconstruction_missing)
        };

        ArchiveTimelineSectionView {
            title: ui_copy("overlay_archive_section_timeline").to_string(),
            status_title: ui_copy("overlay_tower_status").to_string(),
            recent_milestones: summary
                .recent_milestones
                .into_iter()
                .map(|milestone| ArchiveTimelineMilestoneView {
                    title: milestone.title,
                    text: milestone.text,
                })
                .collect(),
            status_lines: vec![
                ui_format(
                    "overlay_brews_completed",
                    &[("count", &summary.total_brews.to_string())],
                ),
                ui_format(
                    "overlay_known_recipes",
                    &[("count", &summary.known_recipe_count.to_string())],
                ),
                ui_format(
                    "overlay_recorded_experiments",
                    &[("count", &summary.experiment_count.to_string())],
                ),
                ui_format(
                    "overlay_unlocked_routes",
                    &[("count", &summary.unlocked_route_count.to_string())],
                ),
            ],
            reconstruction_text,
            reconstruction_locked: !summary.reconstruction_ready,
        }
    }
}
