use crate::content::{ui_copy, ui_format};
use crate::data::{ItemDefinition, MorphDefinition, RecipeDefinition};

use super::matching::sequence_matches;

fn catalyst_matches(morph: &MorphDefinition, catalyst: Option<&ItemDefinition>) -> bool {
    morph.catalyst_tag.is_empty()
        || catalyst
            .map(|item| {
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == &morph.catalyst_tag)
            })
            .unwrap_or(false)
}

/// How far the current setup is from this branch. Used only to decide which
/// branch is worth talking about, never to decide whether one fires.
#[allow(clippy::too_many_arguments)]
fn unmet_requirement_count(
    morph: &MorphDefinition,
    quality_score: u32,
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing: &str,
    ingredients: &[ItemDefinition],
    room_bonus_applied: bool,
) -> usize {
    let checks = [
        quality_score < morph.minimum_quality,
        !catalyst_matches(morph, catalyst),
        heat != morph.required_heat,
        stirs != morph.required_stirs,
        !morph.required_timing.is_empty() && morph.required_timing != timing,
        !morph.required_sequence.is_empty()
            && !sequence_matches(ingredients, &morph.required_sequence),
        morph.room_bonus_required && !room_bonus_applied,
    ];
    checks.into_iter().filter(|unmet| *unmet).count()
}

pub(super) fn morph_output(
    recipe: &RecipeDefinition,
    quality_score: u32,
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing: &str,
    ingredients: &[ItemDefinition],
    room_bonus_applied: bool,
) -> Option<String> {
    for morph in &recipe.morph_targets {
        let catalyst_ok = catalyst_matches(morph, catalyst);
        let timing_ok = morph.required_timing.is_empty() || morph.required_timing == timing;
        let sequence_ok = sequence_matches(ingredients, &morph.required_sequence);
        if quality_score >= morph.minimum_quality
            && catalyst_ok
            && heat == morph.required_heat
            && stirs == morph.required_stirs
            && timing_ok
            && sequence_ok
            && (!morph.room_bonus_required || room_bonus_applied)
        {
            return Some(morph.output_item_id.clone());
        }
    }
    None
}

pub(super) fn morph_trigger_hint(
    recipe: &RecipeDefinition,
    quality_score: u32,
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing: &str,
    ingredients: &[ItemDefinition],
    room_bonus_applied: bool,
) -> Option<String> {
    // A recipe can fork into several branches on different catalysts. Hinting
    // at whichever happens to be listed first would hide every other branch at
    // the bench, so point at the one the current setup is closest to reaching.
    let morph = recipe.morph_targets.iter().min_by_key(|morph| {
        unmet_requirement_count(
            morph,
            quality_score,
            catalyst,
            heat,
            stirs,
            timing,
            ingredients,
            room_bonus_applied,
        )
    })?;

    if quality_score < morph.minimum_quality {
        return Some(ui_copy("morph_hint_quality").to_owned());
    }
    if !catalyst_matches(morph, catalyst) {
        return Some(ui_format(
            "morph_hint_catalyst",
            &[("tag", &morph.catalyst_tag)],
        ));
    }
    if heat != morph.required_heat {
        return Some(ui_format(
            "morph_hint_heat",
            &[("value", &morph.required_heat.to_string())],
        ));
    }
    if stirs != morph.required_stirs {
        return Some(ui_format(
            "morph_hint_stirs",
            &[("value", &morph.required_stirs.to_string())],
        ));
    }
    if !morph.required_timing.is_empty() && morph.required_timing != timing {
        return Some(ui_format(
            "morph_hint_timing",
            &[("value", &morph.required_timing)],
        ));
    }
    if !morph.required_sequence.is_empty()
        && !sequence_matches(ingredients, &morph.required_sequence)
    {
        return Some(ui_copy("morph_hint_sequence").to_owned());
    }
    if morph.room_bonus_required && !room_bonus_applied {
        return Some(ui_copy("morph_hint_room").to_owned());
    }

    Some(ui_copy("morph_hint_generic").to_owned())
}
