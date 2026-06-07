use super::draw_wrapped_text;
use crate::view_models::journal::JournalRapportTabView;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_journal_rapport_tab_view(
    view: &JournalRapportTabView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_text(view.title, x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
    let mut rapport_y = y + 168.0;
    for row in &view.rows {
        draw_text(&row.title, x + 20.0, rapport_y, 20.0, dark::TEXT_BRIGHT);
        rapport_y += 20.0;
        draw_text(&row.now_text, x + 20.0, rapport_y, 17.0, dark::TEXT_DIM);
        rapport_y += 18.0;
        draw_text(&row.later_text, x + 20.0, rapport_y, 17.0, dark::TEXT_DIM);
        rapport_y += 18.0;
        draw_wrapped_text(
            &row.usually_text,
            x + 20.0,
            rapport_y,
            w - 40.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        rapport_y += 34.0;
        if rapport_y > y + h - 40.0 {
            break;
        }
    }
}
