use crate::view_models::alchemy::AlchemyControlsPanelView;
use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_controls_panel_view(view: &AlchemyControlsPanelView, x: f32, y: f32) {
    draw_overlay_section_title(
        x + 20.0,
        y + 270.0,
        view.title,
        None,
    );
    draw_overlay_section_box(x + 18.0, y + 284.0, 286.0, 74.0);
    draw_wrapped_text(
        &view.browse_heat_text,
        x + 32.0,
        y + 304.0,
        262.0,
        16.0,
        18.0,
        dark::TEXT_DIM,
    );
    draw_wrapped_text(
        &view.action_text,
        x + 32.0,
        y + 334.0,
        262.0,
        16.0,
        18.0,
        dark::TEXT_DIM,
    );
}
