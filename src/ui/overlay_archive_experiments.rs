use crate::view_models::archive::ArchiveExperimentsSectionView;
use super::{draw_selection_card, draw_state_banner};
use super::draw_selected_experiment_record_view;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_archive_experiments_section_view(
    view: &ArchiveExperimentsSectionView,
    x: f32,
    y: f32,
    w: f32,
    _h: f32,
) {
        draw_text(
            &view.title,
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(&view.filter_text, x + 220.0, y + 122.0, 20.0, dark::TEXT_DIM);
        if view.entries.is_empty() {
            draw_state_banner(
                x + 20.0,
                y + 144.0,
                w - 40.0,
                &view.empty_text,
                false,
            );
            return;
        }

        if let Some(page_text) = &view.page_text {
            draw_text(page_text, x + 320.0, y + 122.0, 20.0, dark::TEXT_DIM);
        }
        let mut list_y = y + 154.0;
        for entry in &view.entries {
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                entry.selected,
                true,
                &entry.title,
                &entry.detail,
                &entry.meta,
            );
            list_y += 64.0;
        }

        if let Some(record) = &view.selected_record {
            draw_selected_experiment_record_view(record, x + 410.0, y + 122.0, w - 430.0);
        }
}
