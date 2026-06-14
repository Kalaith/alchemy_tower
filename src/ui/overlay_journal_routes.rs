use super::draw_wrapped_text;
use crate::view_models::journal::{JournalHerbMemoriesView, JournalRoutesTabView};
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, Color};
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_journal_routes_tab_view(
    view: &JournalRoutesTabView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_ui_text(view.title, x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
    let mut route_y = y + 168.0;
    for route in &view.route_rows {
        draw_ui_text(&route.title, x + 20.0, route_y, 22.0, dark::TEXT_BRIGHT);
        route_y += 22.0;
        draw_ui_text(&route.detail, x + 20.0, route_y, 18.0, dark::TEXT_DIM);
        route_y += 28.0;
        if route_y > y + h - 40.0 {
            break;
        }
    }

    draw_journal_herb_memories_view(
        &view.herb_memories,
        x + 420.0,
        y + 136.0,
        w - 440.0,
        y + h - 170.0,
    );
    draw_ui_text(
        view.progress_title,
        x + 20.0,
        y + h - 156.0,
        24.0,
        dark::TEXT_BRIGHT,
    );
    draw_rectangle(
        x + 20.0,
        y + h - 140.0,
        w - 40.0,
        96.0,
        Color::from_rgba(38, 40, 50, 255),
    );
    draw_rectangle_lines(x + 20.0, y + h - 140.0, w - 40.0, 96.0, 2.0, dark::ACCENT);
    if let Some(all_restored_text) = &view.route_progress.all_restored_text {
        draw_ui_text(
            all_restored_text,
            x + 34.0,
            y + h - 108.0,
            20.0,
            dark::TEXT_DIM,
        );
    } else {
        let mut unlock_y = y + h - 108.0;
        for line in &view.route_progress.locked_lines {
            draw_wrapped_text(
                line,
                x + 34.0,
                unlock_y,
                w - 68.0,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            unlock_y += 34.0;
        }
    }
}

fn draw_journal_herb_memories_view(
    view: &JournalHerbMemoriesView,
    x: f32,
    title_y: f32,
    text_width: f32,
    bottom_limit: f32,
) {
    draw_ui_text(view.title, x, title_y, 26.0, dark::TEXT_BRIGHT);
    let mut entry_y = title_y + 32.0;
    if view.entries.is_empty() {
        draw_ui_text(&view.empty_text, x, entry_y, 22.0, dark::TEXT_DIM);
        return;
    }

    for entry in &view.entries {
        draw_ui_text(&entry.title, x, entry_y, 22.0, dark::TEXT_BRIGHT);
        entry_y += 22.0;
        draw_ui_text(&entry.state_line, x, entry_y, 18.0, dark::TEXT_DIM);
        entry_y += 20.0;
        draw_ui_text(&entry.route_line, x, entry_y, 18.0, dark::TEXT_DIM);
        entry_y += 20.0;
        draw_wrapped_text(
            &entry.summary,
            x,
            entry_y,
            text_width,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        entry_y += 40.0;
        draw_wrapped_text(
            &entry.conditions,
            x,
            entry_y,
            text_width,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        entry_y += 28.0;
        if let Some(best_specimen_text) = &entry.best_specimen_text {
            draw_ui_text(best_specimen_text, x, entry_y, 18.0, dark::TEXT_DIM);
            entry_y += 20.0;
        }
        if let Some(variant_text) = &entry.variant_text {
            draw_ui_text(variant_text, x, entry_y, 18.0, dark::TEXT_DIM);
            entry_y += 20.0;
        }
        if let Some(note_text) = &entry.note_text {
            draw_wrapped_text(
                note_text,
                x,
                entry_y,
                text_width,
                16.0,
                18.0,
                dark::TEXT_DIM,
            );
            entry_y += 30.0;
        }
        if entry_y > bottom_limit {
            break;
        }
    }
}
