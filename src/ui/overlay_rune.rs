use crate::content::{input_bindings, ui_copy, ui_text};
use crate::view_models::rune::RuneOverlayView;
use super::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_overlay_section_box, draw_overlay_section_title, draw_selection_card, draw_state_banner,
};
use macroquad::prelude::*;

pub(crate) fn draw_rune_overlay_view(view: &RuneOverlayView) {
        draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, &view.station_name);
        draw_overlay_subtitle(x, y, &ui_text().overlays.rune_subtitle);
        draw_overlay_section_title(x + 20.0, y + 124.0, ui_copy("overlay_rune_drafts"), None);
        draw_overlay_section_box(x + 20.0, y + 138.0, w - 40.0, h - 200.0);
        let mut row_y = y + 172.0;
        if view.entries.is_empty() {
            draw_state_banner(
                x + 32.0,
                row_y - 16.0,
                w - 64.0,
                &view.empty_text,
                false,
            );
        } else {
            for entry in &view.entries {
                draw_selection_card(
                    x + 32.0,
                    row_y - 24.0,
                    w - 64.0,
                    58.0,
                    entry.selected,
                    true,
                    &entry.title,
                    &entry.detail,
                    &entry.meta,
                );
                row_y += 64.0;
            }
        }
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_copy("overlay_rune_footer")
                .replace("{select}", &input_bindings().navigation.select)
                .replace("{confirm}", &input_bindings().global.confirm)
                .replace("{close}", &input_bindings().global.cancel),
        );
}
