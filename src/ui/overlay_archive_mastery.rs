use super::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use crate::view_models::archive::ArchiveMasterySectionView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_archive_mastery_section_view(
    view: &ArchiveMasterySectionView,
    x: f32,
    y: f32,
    w: f32,
    _h: f32,
) {
    draw_text(&view.title, x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
    if view.entries.is_empty() {
        draw_state_banner(x + 20.0, y + 144.0, w - 40.0, &view.empty_text, false);
        return;
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

    draw_text(
        &view.detail_title,
        x + 410.0,
        y + 122.0,
        26.0,
        dark::TEXT_BRIGHT,
    );
    let Some(detail) = &view.detail else {
        return;
    };
    draw_text(&detail.title, x + 410.0, y + 156.0, 24.0, dark::TEXT_BRIGHT);
    draw_text(
        &detail.stage_text,
        x + 410.0,
        y + 184.0,
        20.0,
        dark::TEXT_DIM,
    );
    if let Some(best_result_text) = &detail.best_result_text {
        draw_text(best_result_text, x + 410.0, y + 210.0, 20.0, dark::TEXT_DIM);
    }
    if let Some(traits_text) = &detail.traits_text {
        draw_text(traits_text, x + 410.0, y + 236.0, 20.0, dark::TEXT_DIM);
    }
    if let Some(last_attempt_text) = &detail.last_attempt_text {
        draw_text(
            last_attempt_text,
            x + 410.0,
            y + 262.0,
            20.0,
            dark::TEXT_DIM,
        );
    }
    draw_wrapped_text(
        &detail.lore_note,
        x + 410.0,
        y + 292.0,
        w - 430.0,
        18.0,
        20.0,
        dark::TEXT_DIM,
    );
}
