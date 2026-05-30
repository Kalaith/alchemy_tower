use super::gameplay_alchemy_preview_text::{
    output_line, preview_detail, preview_title, process_flags_line, quality_line, read_line,
    requirements_line, traits_line,
};
use crate::alchemy::BrewResolution;
use crate::data::GameData;
use crate::view_models::alchemy::{
    AlchemyPreviewPanelState, AlchemyPreviewPanelView, AlchemyResolvedPreviewView,
};

impl AlchemyPreviewPanelView {
    pub(crate) fn empty_selection() -> Self {
        Self {
            state: AlchemyPreviewPanelState::EmptySelection,
        }
    }

    pub(crate) fn no_station() -> Self {
        Self {
            state: AlchemyPreviewPanelState::NoStation,
        }
    }

    pub(crate) fn resolved(
        data: &GameData,
        preview: &BrewResolution<'_>,
        known: bool,
        stable_preview: bool,
        preview_uncertain: bool,
    ) -> Self {
        Self {
            state: AlchemyPreviewPanelState::Resolved(AlchemyResolvedPreviewView {
                title: preview_title(data, preview, known, stable_preview, preview_uncertain),
                output_line: output_line(data, preview),
                quality_line: quality_line(preview),
                traits_line: traits_line(preview),
                read_line: read_line(preview, known, stable_preview),
                requirements_line: requirements_line(preview),
                process_flags_line: process_flags_line(preview),
                failure_reasons: preview
                    .failure_reasons
                    .iter()
                    .take(3)
                    .map(|reason| reason.to_string())
                    .collect(),
                detail: preview_detail(data, preview, known, stable_preview, preview_uncertain),
                has_recipe: preview.recipe.is_some(),
            }),
        }
    }
}
