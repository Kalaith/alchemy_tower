use super::draw_wrapped_text;
use crate::view_models::archive::ArchiveExperimentRecordView;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_selected_experiment_record_view(
    record: &ArchiveExperimentRecordView,
    x: f32,
    y: f32,
    w: f32,
) {
    draw_ui_text(&record.title, x, y, 26.0, dark::TEXT_BRIGHT);
    draw_ui_text(&record.output_text, x, y + 34.0, 22.0, dark::TEXT_BRIGHT);
    draw_ui_text(&record.quality_text, x, y + 62.0, 20.0, dark::TEXT_DIM);
    draw_ui_text(&record.result_text, x, y + 86.0, 20.0, dark::TEXT_DIM);
    draw_ui_text(&record.catalyst_text, x, y + 110.0, 20.0, dark::TEXT_DIM);
    draw_ui_text(&record.morph_text, x, y + 134.0, 20.0, dark::TEXT_DIM);
    let Some(memory) = &record.recipe_memory else {
        return;
    };
    draw_ui_text(&memory.mastery_text, x, y + 160.0, 20.0, dark::TEXT_DIM);
    draw_ui_text(&memory.memory_text, x, y + 184.0, 18.0, dark::TEXT_DIM);
    draw_wrapped_text(
        &memory.detail_text,
        x,
        y + 210.0,
        w,
        18.0,
        20.0,
        dark::TEXT_DIM,
    );
}
