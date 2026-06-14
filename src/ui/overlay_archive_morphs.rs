use super::{draw_selection_card, draw_state_banner, draw_wrapped_text};
use crate::view_models::archive::ArchiveMorphsSectionView;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_archive_morphs_section_view(
    view: &ArchiveMorphsSectionView,
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
        &view.detail_title,
        x + 410.0,
        y + 122.0,
        26.0,
        dark::TEXT_BRIGHT,
    );
    let Some(detail) = &view.detail else {
        return;
    };
    if let Some(last_morph_text) = &detail.last_morph_text {
        draw_ui_text(last_morph_text, x + 410.0, y + 146.0, 20.0, dark::TEXT_DIM);
    }
    let mut detail_y = y + 176.0;
    for target in &detail.targets {
        draw_ui_text(&target.title, x + 410.0, detail_y, 22.0, dark::TEXT_BRIGHT);
        detail_y += 22.0;
        draw_wrapped_text(
            &target.conditions,
            x + 410.0,
            detail_y,
            w - 430.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
        detail_y += 32.0;
    }
}
