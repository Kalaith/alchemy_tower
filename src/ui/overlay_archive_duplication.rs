use crate::content::ui_copy;
use crate::view_models::archive::ArchiveDuplicationSectionView;
use super::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_archive_duplication_section_view(
    view: &ArchiveDuplicationSectionView,
    x: f32,
    y: f32,
    w: f32,
    _h: f32,
) {
        draw_text(
            ui_copy("overlay_duplication"),
            x + 20.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
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

        let mut list_y = y + 154.0;
        for entry in &view.entries {
            draw_selection_card(
                x + 20.0,
                list_y - 24.0,
                360.0,
                58.0,
                entry.selected,
                entry.enabled,
                &entry.title,
                &entry.detail,
                &entry.meta,
            );
            list_y += 64.0;
        }

        draw_text(
            ui_copy("overlay_duplication_cost"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let Some(detail) = &view.detail else {
            return;
        };
        draw_text(
            &detail.target_text,
            x + 410.0,
            y + 156.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &detail.coin_text,
            x + 410.0,
            y + 184.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &detail.catalyst_text,
            x + 410.0,
            y + 210.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            ui_copy("overlay_archive_duplication_help"),
            x + 410.0,
            y + 244.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
}
