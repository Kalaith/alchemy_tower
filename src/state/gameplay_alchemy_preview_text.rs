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
            ("band", preview.quality_band),
            ("mastery", preview.mastery_stage),
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

pub(super) fn read_line(preview: &BrewResolution<'_>, known: bool, stable_preview: bool) -> String {
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
