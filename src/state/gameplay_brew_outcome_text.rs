use crate::alchemy::BrewResolution;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;

pub(super) fn recipe_logged(recipe_name: &str) -> String {
    ui_format("inventory_recipe_logged", &[("name", recipe_name)])
}

pub(super) fn recipe_discovered(
    recipe_name: &str,
    resolution: &BrewResolution<'_>,
    mastery_stage: &str,
) -> String {
    ui_format(
        "inventory_discovered_status",
        &[
            ("recipe", recipe_name),
            ("quality", resolution.quality_band),
            ("mastery", mastery_stage),
            ("traits", &resolution.inherited_traits.join(", ")),
        ],
    )
}

pub(super) fn brewed(
    data: &GameData,
    resolution: &BrewResolution<'_>,
    stable_brew: bool,
    mastery_stage: &str,
) -> String {
    ui_format(
        "inventory_brewed_status",
        &[
            ("item", data.item_name(&resolution.output_item_id)),
            ("amount", &resolution.output_amount.to_string()),
            ("quality", resolution.quality_band),
            ("result", brew_result_text(stable_brew)),
            ("mastery", mastery_stage),
        ],
    )
}

pub(super) fn mastery_improved(recipe_name: &str, mastery_stage: &str) -> String {
    ui_format(
        "inventory_mastery_improved",
        &[("name", recipe_name), ("stage", mastery_stage)],
    )
}

pub(super) fn collapsed(data: &GameData, resolution: &BrewResolution<'_>) -> String {
    ui_format(
        "inventory_collapse_status",
        &[
            ("quality", resolution.quality_band),
            ("item", data.item_name(&resolution.output_item_id)),
            ("reasons", &resolution.failure_reasons.join(" ")),
        ],
    )
}

fn brew_result_text(stable_brew: bool) -> &'static str {
    if stable_brew {
        ui_copy("inventory_brew_result_stable")
    } else {
        ui_copy("inventory_brew_result_imperfect")
    }
}
