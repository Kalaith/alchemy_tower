use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use crate::view_models::alchemy::{
    AlchemyPreviewPanelState, AlchemyPreviewPanelView, AlchemyResolvedPreviewView,
};
use macroquad::prelude::draw_text;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_preview_panel_view(
    view: &AlchemyPreviewPanelView,
    x: f32,
    y: f32,
    w: f32,
) {
    draw_overlay_section_title(x + 340.0, y + 240.0, view.title, None);
    draw_overlay_section_box(x + 340.0, y + 256.0, w - 360.0, 210.0);

    match &view.state {
        AlchemyPreviewPanelState::EmptySelection => draw_empty_selection(view.empty_text, x, y),
        AlchemyPreviewPanelState::NoStation => {}
        AlchemyPreviewPanelState::Resolved(preview) => {
            draw_resolved_brew_preview_view(preview, x + 360.0, y + 296.0, w - 392.0);
        }
    }
}

fn draw_empty_selection(empty_text: &str, x: f32, y: f32) {
    draw_text(
        empty_text,
        x + 360.0,
        y + 296.0,
        22.0,
        dark::TEXT_DIM,
    );
}

fn draw_resolved_brew_preview_view(
    preview: &AlchemyResolvedPreviewView,
    x: f32,
    y: f32,
    w: f32,
) {
    draw_text(&preview.title, x, y, 24.0, dark::TEXT_BRIGHT);
    draw_output_summary_view(preview, x, y + 30.0);
    draw_text(&preview.read_line, x, y + 96.0, 18.0, dark::TEXT_DIM);

    let process_y = draw_brew_process_diagnostics_view(preview, x, y + 120.0);
    let detail_y = if preview.has_recipe && !preview.failure_reason_lines.is_empty() {
        process_y + 8.0
    } else {
        process_y + 4.0
    };
    draw_wrapped_text(&preview.detail, x, detail_y, w, 18.0, 20.0, dark::TEXT_DIM);
}

fn draw_output_summary_view(preview: &AlchemyResolvedPreviewView, x: f32, y: f32) {
    draw_text(&preview.output_line, x, y, 22.0, dark::TEXT);
    draw_text(&preview.quality_line, x, y + 22.0, 18.0, dark::TEXT_DIM);
    draw_text(&preview.traits_line, x, y + 44.0, 18.0, dark::TEXT_DIM);
}

fn draw_brew_process_diagnostics_view(
    preview: &AlchemyResolvedPreviewView,
    x: f32,
    y: f32,
) -> f32 {
    let (Some(requirements_line), Some(process_flags_line)) =
        (&preview.requirements_line, &preview.process_flags_line)
    else {
        return y;
    };

    let mut next_y = y;
    draw_text(requirements_line, x, next_y, 18.0, dark::TEXT_DIM);
    next_y += 22.0;
    draw_text(process_flags_line, x, next_y, 18.0, dark::TEXT_DIM);
    next_y += 24.0;
    draw_failure_reasons_view(preview, x, next_y)
}

fn draw_failure_reasons_view(preview: &AlchemyResolvedPreviewView, x: f32, y: f32) -> f32 {
    if preview.failure_reason_lines.is_empty() {
        return y;
    }

    let mut next_y = y;
    draw_text(
        preview.failure_reasons_title,
        x,
        next_y,
        18.0,
        dark::TEXT_BRIGHT,
    );
    next_y += 20.0;
    for reason_line in &preview.failure_reason_lines {
        draw_text(
            reason_line,
            x + 12.0,
            next_y,
            18.0,
            dark::TEXT_DIM,
        );
        next_y += 20.0;
    }
    next_y
}
