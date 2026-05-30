use crate::alchemy::BrewResolution;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;

pub(super) fn output_line(data: &GameData, preview: &BrewResolution<'_>) -> String {
    ui_format(
        "overlay_alchemy_output",
        &[
            ("item", data.item_name(&preview.output_item_id)),
            ("amount", &preview.output_amount.to_string()),
        ],
    )
}

pub(super) fn quality_line(preview: &BrewResolution<'_>) -> String {
    ui_format(
        "overlay_alchemy_quality_forecast",
        &[
            ("quality", &preview.quality_score.to_string()),
            ("band", &preview.quality_band),
            ("mastery", &preview.mastery_stage),
        ],
    )
}

pub(super) fn traits_line(preview: &BrewResolution<'_>) -> String {
    ui_format(
        "overlay_alchemy_traits",
        &[(
            "traits",
            &if preview.inherited_traits.is_empty() {
                ui_copy("overlay_alchemy_traits_none").to_owned()
            } else {
                preview.inherited_traits.join(", ")
            },
        )],
    )
}

pub(super) fn read_line(
    preview: &BrewResolution<'_>,
    known: bool,
    stable_preview: bool,
) -> String {
    ui_format(
        "overlay_alchemy_read",
        &[(
            "text",
            ui_copy(if preview.recipe.is_none() {
                "overlay_alchemy_read_fallback"
            } else if !known {
                "overlay_alchemy_read_unlogged"
            } else if stable_preview {
                "overlay_alchemy_read_stable"
            } else {
                "overlay_alchemy_read_degraded"
            }),
        )],
    )
}

pub(super) fn requirements_line(preview: &BrewResolution<'_>) -> Option<String> {
    preview.recipe.map(|recipe| {
        ui_format(
            "overlay_alchemy_requirements",
            &[
                ("heat", &recipe.required_heat.to_string()),
                ("stirs", &recipe.required_stirs.to_string()),
                (
                    "timing",
                    if recipe.required_timing.is_empty() {
                        ui_copy("overlay_any")
                    } else {
                        recipe.required_timing.as_str()
                    },
                ),
                (
                    "process",
                    ui_copy(if preview.process_match {
                        "overlay_archive_state_stable"
                    } else {
                        "overlay_archive_state_unstable"
                    }),
                ),
                (
                    "quality",
                    ui_copy(if preview.minimum_quality_met {
                        "overlay_pass"
                    } else {
                        "overlay_fail"
                    }),
                ),
                (
                    "elements",
                    ui_copy(if preview.minimum_elements_met {
                        "overlay_pass"
                    } else {
                        "overlay_fail"
                    }),
                ),
            ],
        )
    })
}

pub(super) fn process_flags_line(preview: &BrewResolution<'_>) -> Option<String> {
    preview.recipe.map(|_| {
        ui_format(
            "overlay_alchemy_process_flags",
            &[
                (
                    "catalyst",
                    ui_copy(if preview.catalyst_match {
                        "overlay_match"
                    } else {
                        "overlay_miss"
                    }),
                ),
                (
                    "timing",
                    ui_copy(if preview.timing_match {
                        "overlay_match"
                    } else {
                        "overlay_miss"
                    }),
                ),
                (
                    "sequence",
                    ui_copy(if preview.sequence_match {
                        "overlay_match"
                    } else {
                        "overlay_miss"
                    }),
                ),
                (
                    "room",
                    ui_copy(if preview.room_bonus_applied {
                        "overlay_active"
                    } else {
                        "overlay_inactive"
                    }),
                ),
            ],
        )
    })
}

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
