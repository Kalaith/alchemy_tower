use crate::view_models::journal::JournalGreenhouseTabView;
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_journal_greenhouse_tab_view(
    view: &JournalGreenhouseTabView,
    x: f32,
    y: f32,
    _w: f32,
    h: f32,
) {
    draw_text(view.title, x + 20.0, y + 136.0, 26.0, dark::TEXT_BRIGHT);
    let mut bed_y = y + 168.0;
    for bed in &view.beds {
        draw_text(&bed.title, x + 20.0, bed_y, 22.0, dark::TEXT_BRIGHT);
        bed_y += 22.0;
        draw_text(&bed.summary, x + 20.0, bed_y, 18.0, dark::TEXT_DIM);
        bed_y += 30.0;
        if bed_y > y + h - 40.0 {
            break;
        }
    }
    if view.beds.is_empty() {
        draw_text(&view.empty_text, x + 20.0, bed_y, 20.0, dark::TEXT_DIM);
    }
}
