use crate::content::ui_copy;
use crate::view_models::archive::ArchiveTimelineSectionView;
use super::{draw_state_banner, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_archive_timeline_section_view(
    view: &ArchiveTimelineSectionView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
        draw_text(
            ui_copy("overlay_archive_section_timeline"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut row_y = y + 152.0;
        for milestone in &view.recent_milestones {
            draw_text(&milestone.title, x + 20.0, row_y, 20.0, dark::TEXT_BRIGHT);
            row_y += 20.0;
            draw_wrapped_text(
                &milestone.text,
                x + 20.0,
                row_y,
                430.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            row_y += 40.0;
        }

        draw_text(
            ui_copy("overlay_tower_status"),
            x + 500.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut status_y = y + 156.0;
        for line in &view.status_lines {
            draw_text(&line, x + 500.0, status_y, 20.0, dark::TEXT_DIM);
            status_y += 24.0;
        }
        draw_state_banner(
            x + 500.0,
            y + h - 120.0,
            w - 520.0,
            &view.reconstruction_text,
            view.reconstruction_locked,
        );
}
