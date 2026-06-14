use super::{draw_state_banner, draw_wrapped_text};
use crate::view_models::archive::ArchiveTimelineSectionView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_archive_timeline_section_view(
    view: &ArchiveTimelineSectionView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_ui_text(&view.title, x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
    let mut row_y = y + 152.0;
    for milestone in &view.recent_milestones {
        draw_ui_text(&milestone.title, x + 20.0, row_y, 20.0, dark::TEXT_BRIGHT);
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

    draw_ui_text(
        &view.status_title,
        x + 500.0,
        y + 122.0,
        26.0,
        dark::TEXT_BRIGHT,
    );
    let mut status_y = y + 156.0;
    for line in &view.status_lines {
        draw_ui_text(line.as_str(), x + 500.0, status_y, 20.0, dark::TEXT_DIM);
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
