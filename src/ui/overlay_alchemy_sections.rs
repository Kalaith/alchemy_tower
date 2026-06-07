use super::{
    draw_overlay_section_box, draw_overlay_section_title, draw_selection_card, draw_state_banner,
};
use crate::alchemy_layout::material_row_rect_at;
use crate::view_models::alchemy::AlchemyMaterialsPanelView;

pub(crate) fn draw_alchemy_materials_panel_view(view: &AlchemyMaterialsPanelView, x: f32, y: f32) {
    draw_overlay_section_title(x + 20.0, y + 84.0, view.title, Some(&view.sort_text));
    draw_overlay_section_box(x + 18.0, y + 98.0, 286.0, 162.0);
    let iy = y + 82.0;
    if view.rows.is_empty() {
        draw_state_banner(x + 30.0, iy - 12.0, 262.0, &view.empty_text, false);
    } else {
        for (index, row) in view.rows.iter().enumerate() {
            let row_rect = material_row_rect_at(x, y, index);
            draw_selection_card(
                row_rect.x + 12.0,
                row_rect.y,
                262.0,
                row_rect.h,
                row.selected,
                row.enabled,
                &row.title,
                &row.detail,
                &row.meta,
            );
        }
    }
}
