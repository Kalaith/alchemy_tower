use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use crate::view_models::alchemy::AlchemyFormulaePanelView;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_formulae_panel_view(view: &AlchemyFormulaePanelView, x: f32, y: f32) {
    draw_overlay_section_title(x + 20.0, y + 392.0, view.title, None);
    draw_overlay_section_box(x + 18.0, y + 406.0, 286.0, 142.0);
    let mut ky = y + 424.0;
    for row in &view.rows {
        draw_text(&row.title, x + 20.0, ky, 22.0, dark::TEXT_BRIGHT);
        ky += 22.0;
        draw_text(&row.meta, x + 20.0, ky, 18.0, dark::TEXT_DIM);
        ky += 20.0;
        draw_wrapped_text(&row.detail, x + 20.0, ky, 286.0, 16.0, 18.0, dark::TEXT_DIM);
        ky += 36.0;
        draw_wrapped_text(
            &row.lore_note,
            x + 20.0,
            ky,
            286.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        ky += 32.0;
    }
    if view.rows.is_empty() {
        draw_text(&view.empty_text, x + 32.0, ky, 20.0, dark::TEXT_DIM);
    }
}
