use crate::alchemy::BrewResolution;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;

pub(super) fn preview_title(
    data: &GameData,
    preview: &BrewResolution<'_>,
    known: bool,
    stable_preview: bool,
    preview_uncertain: bool,
) -> String {
    if preview.recipe.is_none() {
        ui_copy("overlay_alchemy_preview_unknown_salvage").to_owned()
    } else if known && stable_preview && !preview_uncertain {
        ui_format(
            "overlay_known_result",
            &[("item", data.item_name(&preview.output_item_id))],
        )
    } else if !known {
        ui_copy("overlay_alchemy_preview_unlogged").to_owned()
    } else if preview_uncertain {
        ui_copy("overlay_alchemy_preview_uncertain").to_owned()
    } else if !preview.process_match {
        ui_copy("overlay_alchemy_preview_unstable_process").to_owned()
    } else if !preview.minimum_elements_met {
        ui_copy("overlay_alchemy_preview_element_shortfall").to_owned()
    } else if !preview.minimum_quality_met {
        ui_copy("overlay_alchemy_preview_quality_shortfall").to_owned()
    } else {
        ui_copy("overlay_alchemy_preview_imperfect").to_owned()
    }
}

pub(super) fn preview_detail(
    data: &GameData,
    preview: &BrewResolution<'_>,
    known: bool,
    stable_preview: bool,
    preview_uncertain: bool,
) -> String {
    preview
        .recipe
        .map(|recipe| {
            if let Some(morph_output_item_id) = &preview.morph_output_item_id {
                ui_format(
                    "overlay_alchemy_morph_ready",
                    &[
                        ("recipe", &recipe.name),
                        ("item", data.item_name(morph_output_item_id)),
                    ],
                )
            } else if let Some(morph_hint) = &preview.morph_hint {
                morph_hint.clone()
            } else if known && preview_uncertain {
                ui_copy("overlay_alchemy_unknown_catalyst_branch").to_owned()
            } else if known && stable_preview {
                recipe.description.clone()
            } else if !preview.process_match {
                ui_copy("overlay_alchemy_distort").to_owned()
            } else if !preview.minimum_elements_met {
                ui_copy("overlay_alchemy_missing_elements").to_owned()
            } else if !preview.minimum_quality_met {
                ui_copy("overlay_alchemy_missing_quality").to_owned()
            } else {
                ui_copy("overlay_alchemy_not_proven").to_owned()
            }
        })
        .unwrap_or_else(|| ui_copy("overlay_alchemy_collapse").to_owned())
}
