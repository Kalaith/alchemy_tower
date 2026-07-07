use super::{draw_overlay_section_box, draw_overlay_section_title, draw_wrapped_text};
use crate::alchemy_layout::{
    right_column_width, AL_BOX_BOTTOM_MARGIN, AL_PREV_BOX_Y, AL_PREV_TITLE_Y, AL_RX,
};
use crate::view_models::alchemy::{
    AlchemyPreviewPanelState, AlchemyPreviewPanelView, AlchemyResolvedPreviewView,
};
use macroquad::prelude::Color;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::draw_ui_text;

pub(crate) fn draw_alchemy_preview_panel_view(
    view: &AlchemyPreviewPanelView,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let rw = right_column_width(w);
    let box_h = (h - AL_BOX_BOTTOM_MARGIN - AL_PREV_BOX_Y).max(120.0);
    draw_overlay_section_title(x + AL_RX, y + AL_PREV_TITLE_Y, view.title, None);
    draw_overlay_section_box(x + AL_RX - 2.0, y + AL_PREV_BOX_Y, rw, box_h);

    let content_x = x + AL_RX + 18.0;
    let content_y = y + AL_PREV_BOX_Y + 30.0;
    match &view.state {
        AlchemyPreviewPanelState::EmptySelection => {
            draw_ui_text(
                view.empty_text,
                content_x,
                content_y + 8.0,
                22.0,
                dark::TEXT_DIM,
            );
        }
        AlchemyPreviewPanelState::NoStation => {}
        AlchemyPreviewPanelState::Resolved(preview) => {
            draw_resolved_brew_preview_view(preview, content_x, content_y, rw - 36.0);
        }
    }
}

fn draw_resolved_brew_preview_view(preview: &AlchemyResolvedPreviewView, x: f32, y: f32, w: f32) {
    draw_ui_text(&preview.title, x, y, 24.0, dark::TEXT_BRIGHT);
    draw_output_summary_view(preview, x, y + 30.0);
    let mut read_y = y + 96.0;
    if let Some(instability_line) = &preview.instability_line {
        // Warm amber for a live overcharge, red once it will collapse — so the
        // risk of pushing reads at a glance without parsing the number.
        let color = if preview.destabilized {
            Color::from_rgba(224, 122, 108, 255)
        } else {
            Color::from_rgba(224, 176, 108, 255)
        };
        draw_ui_text(instability_line, x, read_y, 18.0, color);
        read_y += 24.0;
    }
    if let Some(quest_line) = &preview.quest_line {
        // Warm gold so the "who this is for" note reads as a story beat, not
        // another mechanical stat line.
        draw_ui_text(
            quest_line,
            x,
            read_y,
            18.0,
            Color::from_rgba(242, 205, 126, 255),
        );
        read_y += 24.0;
    }
    draw_ui_text(&preview.read_line, x, read_y, 18.0, dark::TEXT_DIM);

    let process_y = draw_brew_process_diagnostics_view(preview, x, read_y + 24.0);
    let detail_y = if preview.has_recipe && !preview.failure_reason_lines.is_empty() {
        process_y + 8.0
    } else {
        process_y + 4.0
    };
    draw_wrapped_text(&preview.detail, x, detail_y, w, 18.0, 20.0, dark::TEXT_DIM);
}

fn draw_output_summary_view(preview: &AlchemyResolvedPreviewView, x: f32, y: f32) {
    draw_ui_text(&preview.output_line, x, y, 22.0, dark::TEXT);
    draw_ui_text(&preview.quality_line, x, y + 22.0, 18.0, dark::TEXT_DIM);
    draw_ui_text(&preview.traits_line, x, y + 44.0, 18.0, dark::TEXT_DIM);
}

fn draw_brew_process_diagnostics_view(preview: &AlchemyResolvedPreviewView, x: f32, y: f32) -> f32 {
    let (Some(requirements_line), Some(process_flags_line)) =
        (&preview.requirements_line, &preview.process_flags_line)
    else {
        return y;
    };

    let mut next_y = y;
    draw_ui_text(requirements_line, x, next_y, 18.0, dark::TEXT_DIM);
    next_y += 22.0;
    draw_ui_text(process_flags_line, x, next_y, 18.0, dark::TEXT_DIM);
    next_y += 24.0;
    draw_failure_reasons_view(preview, x, next_y)
}

fn draw_failure_reasons_view(preview: &AlchemyResolvedPreviewView, x: f32, y: f32) -> f32 {
    if preview.failure_reason_lines.is_empty() {
        return y;
    }

    let mut next_y = y;
    draw_ui_text(
        preview.failure_reasons_title,
        x,
        next_y,
        18.0,
        dark::TEXT_BRIGHT,
    );
    next_y += 20.0;
    for reason_line in &preview.failure_reason_lines {
        draw_ui_text(reason_line, x + 12.0, next_y, 18.0, dark::TEXT_DIM);
        next_y += 20.0;
    }
    next_y
}
