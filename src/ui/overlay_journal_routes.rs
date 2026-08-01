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
    if let Some(range_text) = &view.route_range_text {
        draw_ui_text(range_text, x + 200.0, y + 136.0, 16.0, dark::TEXT_DIM);
    }
    // The route column stops short of the herb column. Route descriptions used
    // to be drawn as one unwrapped line and ran straight through the herbs to
    // the right of them.
    const ROUTE_TEXT_WIDTH: f32 = 380.0;
    let route_limit = y + h - 170.0;
    let mut route_y = y + 168.0;
    for route in &view.route_rows {
        let colour = if route.selected {
            dark::TEXT_BRIGHT
        } else {
            dark::TEXT_DIM
        };
        draw_ui_text(&route.title, x + 20.0, route_y, 20.0, colour);
        route_y += 22.0;
    }
    if let Some(detail) = &view.route_detail {
        route_y += 8.0;
        if route_y <= route_limit {
            draw_wrapped_block(detail, x + 20.0, route_y, ROUTE_TEXT_WIDTH);
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

/// Draw a wrapped block and return the next y, advanced by the text's real
/// wrapped height so a long line cannot overlap whatever comes beneath it.
fn draw_wrapped_block(text: &str, x: f32, y: f32, text_width: f32) -> f32 {
    const FONT: f32 = 16.0;
    const LINE_HEIGHT: f32 = 18.0;
    const BLOCK_GAP: f32 = 8.0;
    draw_wrapped_text(text, x, y, text_width, FONT, LINE_HEIGHT, dark::TEXT_DIM);
    let lines = super::text::wrapped_lines(text, text_width, FONT)
        .len()
        .max(1) as f32;
    y + lines * LINE_HEIGHT + BLOCK_GAP
}

fn draw_journal_herb_memories_view(
    view: &JournalHerbMemoriesView,
    x: f32,
    title_y: f32,
    text_width: f32,
    bottom_limit: f32,
) {
    draw_ui_text(view.title, x, title_y, 26.0, dark::TEXT_BRIGHT);
    if let Some(range_text) = &view.range_text {
        draw_ui_text(range_text, x + 210.0, title_y, 16.0, dark::TEXT_DIM);
    }
    let mut entry_y = title_y + 32.0;
    if view.rows.is_empty() {
        draw_ui_text(&view.empty_text, x, entry_y, 22.0, dark::TEXT_DIM);
        return;
    }

    // One line each, so the whole shelf is legible at a glance.
    for row in &view.rows {
        let colour = if row.selected {
            dark::TEXT_BRIGHT
        } else {
            dark::TEXT_DIM
        };
        draw_ui_text(&row.title, x, entry_y, 20.0, colour);
        draw_ui_text(&row.state_line, x + 210.0, entry_y, 16.0, dark::TEXT_DIM);
        entry_y += 22.0;
    }

    // Everything known about the selected herb, under the list and clipped to
    // the column so a long note cannot run into the panel below.
    let Some(entry) = &view.detail else {
        return;
    };
    entry_y += 8.0;
    draw_ui_text(&entry.route_line, x, entry_y, 18.0, dark::TEXT_DIM);
    entry_y += 20.0;
    for block in [Some(&entry.summary), Some(&entry.conditions)]
        .into_iter()
        .flatten()
        .chain(entry.used_in_text.as_ref())
        .chain(entry.note_text.as_ref())
    {
        if entry_y > bottom_limit {
            return;
        }
        entry_y = draw_wrapped_block(block, x, entry_y, text_width);
    }
    for line in [
        entry.best_specimen_text.as_ref(),
        entry.variant_text.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        if entry_y > bottom_limit {
            return;
        }
        draw_ui_text(line, x, entry_y, 18.0, dark::TEXT_DIM);
        entry_y += 20.0;
    }
}
