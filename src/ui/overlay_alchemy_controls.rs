use crate::content::{input_bindings, ui_copy, ui_format};
use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_controls_panel_view(x: f32, y: f32) {
    draw_overlay_section_title(
        x + 20.0,
        y + 270.0,
        ui_copy("overlay_alchemy_controls"),
        None,
    );
    draw_overlay_section_box(x + 18.0, y + 284.0, 286.0, 74.0);
    draw_wrapped_text(
        &ui_format(
            "overlay_alchemy_controls_line1",
            &[
                ("browse", &input_bindings().navigation.select),
                ("heat", &input_bindings().alchemy.heat),
            ],
        ),
        x + 32.0,
        y + 304.0,
        262.0,
        16.0,
        18.0,
        dark::TEXT_DIM,
    );
    draw_wrapped_text(
        &ui_format(
            "overlay_alchemy_controls_line2",
            &[
                ("fill", &input_bindings().alchemy.fill_slots),
                ("catalyst", &input_bindings().alchemy.catalyst),
                ("brew", &input_bindings().alchemy.brew),
                ("brew_alt", &input_bindings().alchemy.brew_alternate),
                ("repeat", &input_bindings().alchemy.repeat),
                ("sort", &input_bindings().global.sort),
                ("clear", &input_bindings().alchemy.clear),
            ],
        ),
        x + 32.0,
        y + 334.0,
        262.0,
        16.0,
        18.0,
        dark::TEXT_DIM,
    );
}
