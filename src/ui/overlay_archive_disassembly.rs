use crate::content::ui_copy;
use crate::view_models::archive::ArchiveDisassemblySectionView;
use super::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_archive_disassembly_section_view(
    view: &ArchiveDisassemblySectionView,
    x: f32,
    y: f32,
    w: f32,
    _h: f32,
) {
        draw_text(
            ui_copy("overlay_disassembly"),
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
                true,
                &entry.title,
                &entry.detail,
                &entry.meta,
            );
            list_y += 64.0;
        }

        draw_text(
            ui_copy("overlay_recovered_inputs"),
            x + 410.0,
            y + 122.0,
            26.0,
            dark::TEXT_BRIGHT,
        );
        let mut detail_y = y + 156.0;
        for input in &view.selected_inputs {
            draw_text(
                input,
                x + 410.0,
                detail_y,
                22.0,
                dark::TEXT_DIM,
            );
            detail_y += 24.0;
        }
        draw_wrapped_text(
            ui_copy("overlay_archive_disassembly_help"),
            x + 410.0,
            detail_y + 18.0,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
}
