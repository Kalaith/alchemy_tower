use crate::content::{ui_copy, ui_format};
use crate::view_models::alchemy::AlchemyMaterialsPanelView;
use super::{
    draw_overlay_section_box, draw_overlay_section_title, draw_selection_card, draw_state_banner,
};

pub(crate) fn draw_alchemy_materials_panel_view(
    view: &AlchemyMaterialsPanelView,
    x: f32,
    y: f32,
) {
        draw_overlay_section_title(
            x + 20.0,
            y + 84.0,
            ui_copy("overlay_materials"),
            Some(&ui_format("overlay_sort_mode", &[("mode", &view.sort_label)])),
        );
        draw_overlay_section_box(x + 18.0, y + 98.0, 286.0, 162.0);
        let mut iy = y + 82.0;
        if view.rows.is_empty() {
            draw_state_banner(
                x + 30.0,
                iy - 12.0,
                262.0,
                &view.empty_text,
                false,
            );
        } else {
            for row in &view.rows {
                draw_selection_card(
                    x + 30.0,
                    iy - 24.0,
                    262.0,
                    52.0,
                    row.selected,
                    row.enabled,
                    &row.title,
                    &row.detail,
                    &row.meta,
                );
                iy += 58.0;
            }
        }
}
