use crate::content::ui_copy;
use crate::data::RecipeDefinition;

#[allow(clippy::too_many_arguments)]
pub(super) fn brew_failure_reasons(
    recipe: &RecipeDefinition,
    heat: i32,
    stirs: u32,
    timing_match: bool,
    sequence_match: bool,
    catalyst_match: bool,
    minimum_quality_met: bool,
    minimum_elements_met: bool,
    destabilized: bool,
) -> Vec<String> {
    let mut reasons = Vec::new();

    // Overcharging (heat/stirs above the target) is a deliberate risk, not a
    // fault, so only underfiring/understirring is reported here — the danger of
    // pushing too far surfaces as the destabilization reason below.
    if heat < recipe.required_heat {
        reasons.push(ui_copy("brew_failure_heat_low").to_owned());
    }

    if stirs < recipe.required_stirs {
        reasons.push(ui_copy("brew_failure_stirs_low").to_owned());
    }

    if destabilized {
        reasons.push(ui_copy("brew_failure_overcharged").to_owned());
    }

    if !timing_match && !recipe.required_timing.is_empty() {
        reasons.push(ui_copy("brew_failure_timing").to_owned());
    }
    if !sequence_match && !recipe.required_sequence.is_empty() {
        reasons.push(ui_copy("brew_failure_sequence").to_owned());
    }
    if !catalyst_match && !recipe.catalyst_tag.is_empty() {
        reasons.push(ui_copy("brew_failure_catalyst").to_owned());
    }
    if !minimum_elements_met && recipe.minimum_elements.total() > 0 {
        reasons.push(ui_copy("brew_failure_elements").to_owned());
    }
    if !minimum_quality_met && recipe.minimum_quality > 0 {
        reasons.push(ui_copy("brew_failure_quality").to_owned());
    }

    reasons
}
