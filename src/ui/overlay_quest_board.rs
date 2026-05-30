use crate::content::{input_bindings, ui_copy, ui_text};
use crate::view_models::quest_board::QuestBoardOverlayView;
use super::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_overlay_section_box, draw_overlay_section_title, draw_selection_card, draw_state_banner,
    draw_wrapped_text,
};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_quest_board_overlay_view(view: &QuestBoardOverlayView) {
        draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, ui_copy("overlay_quest_board_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.quest_board_subtitle);
        draw_overlay_section_title(
            x + 20.0,
            y + 122.0,
            ui_copy("overlay_quest_available"),
            None,
        );
        draw_overlay_section_box(x + 20.0, y + 136.0, w - 40.0, 232.0);
        let mut row_y = y + 168.0;
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
        draw_text(
            ui_copy("overlay_quest_locked"),
            x + 20.0,
            y + h - 200.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_overlay_section_box(x + 20.0, y + h - 186.0, w - 40.0, 54.0);
        draw_wrapped_text(
            &view.locked_text,
            x + 32.0,
            y + h - 164.0,
            w - 64.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            ui_copy("overlay_quest_active"),
            x + 20.0,
            y + h - 120.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_overlay_section_box(x + 20.0, y + h - 106.0, w - 40.0, 52.0);
        draw_wrapped_text(
            &view.active_text,
            x + 32.0,
            y + h - 84.0,
            w - 64.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
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
