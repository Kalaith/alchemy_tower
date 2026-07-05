use super::gameplay_alchemy_preview_detail_text::{preview_detail, preview_title};
use super::gameplay_alchemy_preview_text::{
    output_line, process_flags_line, quality_line, read_line, requirements_line, traits_line,
};
use crate::alchemy::BrewResolution;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::alchemy::{
    AlchemyPreviewPanelState, AlchemyPreviewPanelView, AlchemyResolvedPreviewView,
};

impl AlchemyPreviewPanelView {
    pub(crate) fn empty_selection() -> Self {
        preview_panel_view(AlchemyPreviewPanelState::EmptySelection)
    }

    pub(crate) fn no_station() -> Self {
        preview_panel_view(AlchemyPreviewPanelState::NoStation)
    }

    pub(crate) fn resolved(
        data: &GameData,
        preview: &BrewResolution<'_>,
        known: bool,
        stable_preview: bool,
        preview_uncertain: bool,
        quest_line: Option<String>,
    ) -> Self {
        preview_panel_view(AlchemyPreviewPanelState::Resolved(
            AlchemyResolvedPreviewView {
                title: preview_title(data, preview, known, stable_preview, preview_uncertain),
                quest_line,
                output_line: output_line(data, preview),
                quality_line: quality_line(preview),
                traits_line: traits_line(preview),
                read_line: read_line(preview, known, stable_preview),
                requirements_line: requirements_line(preview),
                process_flags_line: process_flags_line(preview),
                failure_reasons_title: ui_copy("overlay_alchemy_instability_points"),
                failure_reason_lines: preview
                    .failure_reasons
                    .iter()
                    .take(3)
                    .map(|reason| {
                        let reason = reason.to_string();
                        ui_format("overlay_alchemy_failure_reason", &[("reason", &reason)])
                    })
                    .collect(),
                detail: preview_detail(data, preview, known, stable_preview, preview_uncertain),
                has_recipe: preview.recipe.is_some(),
            },
        ))
    }
}

fn preview_panel_view(state: AlchemyPreviewPanelState) -> AlchemyPreviewPanelView {
    AlchemyPreviewPanelView {
        title: ui_copy("overlay_preview"),
        empty_text: ui_copy("overlay_preview_empty"),
        state,
    }
}
