use super::{
    draw_overlay_section_box, draw_overlay_section_title, draw_selection_card, draw_state_banner,
    truncate_text_to_width,
};
use crate::alchemy_layout::{
    material_row_rect_at, AL_LW, AL_LX, AL_MAT_BOX_H, AL_MAT_BOX_Y, AL_MAT_TITLE_Y,
    AL_MAT_VISIBLE_ROWS,
};
use crate::view_models::alchemy::AlchemyMaterialsPanelView;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(crate) fn draw_alchemy_materials_panel_view(view: &AlchemyMaterialsPanelView, x: f32, y: f32) {
    draw_overlay_section_title(x + AL_LX, y + AL_MAT_TITLE_Y, view.title, None);
    // Sort mode, right-aligned inside the column so it can't spill into the
    // slots panel the way the old fixed-offset meta did.
    let sort_label = if view.rows.len() > AL_MAT_VISIBLE_ROWS {
        format!("{} ({})", view.sort_text, view.rows.len())
    } else {
        view.sort_text.clone()
    };
    let sort_label = truncate_text_to_width(&sort_label, 168.0, 15.0);
    let sort_width = measure_ui_text(&sort_label, None, 15, 1.0).width;
    draw_ui_text(
        &sort_label,
        x + AL_LX - 2.0 + AL_LW - sort_width - 8.0,
        y + AL_MAT_TITLE_Y,
        15.0,
        dark::TEXT_DIM,
    );
    draw_overlay_section_box(x + AL_LX - 2.0, y + AL_MAT_BOX_Y, AL_LW, AL_MAT_BOX_H);

    if view.rows.is_empty() {
        draw_state_banner(
            x + AL_LX + 8.0,
            y + AL_MAT_BOX_Y + 16.0,
            AL_LW - 24.0,
            &view.empty_text,
            false,
        );
        return;
    }

    // Scroll window that keeps the selected material on screen without letting
    // the list overrun its box.
    let selected = view.rows.iter().position(|row| row.selected).unwrap_or(0);
    let start = selected
        .saturating_sub(AL_MAT_VISIBLE_ROWS - 1)
        .min(view.rows.len().saturating_sub(AL_MAT_VISIBLE_ROWS));
    let end = (start + AL_MAT_VISIBLE_ROWS).min(view.rows.len());

    for (offset, row) in view.rows[start..end].iter().enumerate() {
        let rect = material_row_rect_at(x, y, offset);
        // Description is omitted here (it appears in the preview) so each row
        // stays compact enough to avoid overlapping its neighbor.
        draw_selection_card(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            row.selected,
            row.enabled,
            &row.title,
            "",
            &row.meta,
        );
    }
}
