use super::draw_wrapped_text;
use crate::view_models::journal::JournalNotesTabView;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_journal_notes_tab_view(
    view: &JournalNotesTabView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_text(view.title, x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
    draw_text(
        view.active_title,
        x + 20.0,
        y + 168.0,
        24.0,
        dark::TEXT_BRIGHT,
    );
    draw_rectangle(
        x + 20.0,
        y + 182.0,
        w - 40.0,
        72.0,
        Color::from_rgba(38, 40, 50, 255),
    );
    draw_rectangle_lines(x + 20.0, y + 182.0, w - 40.0, 72.0, 2.0, dark::ACCENT);
    draw_wrapped_text(
        &view.active_summary,
        x + 34.0,
        y + 206.0,
        w - 68.0,
        18.0,
        20.0,
        dark::TEXT_DIM,
    );

    draw_text(
        view.milestones_title,
        x + 20.0,
        y + 286.0,
        24.0,
        dark::TEXT_BRIGHT,
    );
    let mut milestone_y = y + 318.0;
    for row in &view.milestone_rows {
        draw_text(&row.title, x + 20.0, milestone_y, 20.0, dark::TEXT_BRIGHT);
        milestone_y += 20.0;
        draw_wrapped_text(
            &row.detail,
            x + 20.0,
            milestone_y,
            w - 40.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        milestone_y += 34.0;
    }

    let mut note_y = y + 448.0;
    for milestone in &view.recent_milestones {
        draw_text(&milestone.title, x + 20.0, note_y, 22.0, dark::TEXT_BRIGHT);
        note_y += 22.0;
        draw_wrapped_text(
            &milestone.text,
            x + 20.0,
            note_y,
            w - 40.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
        note_y += 52.0;
        if note_y > y + h - 40.0 {
            break;
        }
    }
}
