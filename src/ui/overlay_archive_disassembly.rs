use super::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use crate::view_models::archive::ArchiveDisassemblySectionView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_archive_disassembly_section_view(
    view: &ArchiveDisassemblySectionView,
    x: f32,
    y: f32,
    w: f32,
    _h: f32,
) {
    draw_ui_text(&view.title, x + 20.0, y + 122.0, 26.0, dark::TEXT_BRIGHT);
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

    draw_ui_text(
        &view.selected_inputs_title,
        x + 410.0,
        y + 122.0,
        26.0,
        dark::TEXT_BRIGHT,
    );
    let mut detail_y = y + 156.0;
    for input in &view.selected_inputs {
        draw_ui_text(input, x + 410.0, detail_y, 22.0, dark::TEXT_DIM);
        detail_y += 24.0;
    }
    draw_wrapped_text(
        &view.help_text,
        x + 410.0,
        detail_y + 18.0,
        w - 430.0,
        18.0,
        20.0,
        dark::TEXT_DIM,
    );
}
