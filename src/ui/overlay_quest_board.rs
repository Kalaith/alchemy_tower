use super::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_section_box,
    draw_overlay_section_title, draw_overlay_subtitle, draw_panel, draw_selection_card,
    draw_state_banner, draw_wrapped_text, standard_overlay_panel_rect,
};
use crate::view_models::quest_board::QuestBoardOverlayView;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_quest_board_overlay_view(view: &QuestBoardOverlayView) {
    draw_overlay_backdrop();
    let panel = standard_overlay_panel_rect();
    let x = panel.x;
    let y = panel.y;
    let w = panel.w;
    let h = panel.h;
    draw_panel(x, y, w, h, &view.title);
    draw_overlay_subtitle(x, y, &view.subtitle);
    draw_overlay_section_title(x + 20.0, y + 122.0, &view.available_title, None);
    draw_overlay_section_box(x + 20.0, y + 136.0, w - 40.0, 232.0);
    let mut row_y = y + 168.0;
    if view.entries.is_empty() {
        draw_state_banner(x + 32.0, row_y - 16.0, w - 64.0, &view.empty_text, false);
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
        &view.locked_title,
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
        &view.active_title,
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
    draw_overlay_footer(x, y, w, h, &view.footer_text);
}
