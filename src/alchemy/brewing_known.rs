use crate::data::{GameData, RecipeDefinition, StationDefinition};

use super::brewing_failures::brew_failure_reasons;
use super::super::matching::{selected_item_defs, sequence_matches, total_elements};
use super::super::morphs::{morph_output, morph_trigger_hint};
use super::super::quality::{calculate_quality, mastery_stage, quality_band, room_bonus_applies};
use super::super::traits::inherited_traits;
use super::BrewResolution;

pub(super) fn resolve_known_recipe_brew<'a>(
    data: &'a GameData,
    station: &StationDefinition,
    selected_items: &[String],
    catalyst_item: Option<&str>,
    heat: i32,
    stirs: u32,
    timing: &str,
    mastery_brews: u32,
    recipe: &'a RecipeDefinition,
) -> BrewResolution<'a> {
    let ingredient_items = selected_item_defs(data, selected_items);
    let catalyst = catalyst_item.and_then(|item_id| data.item(item_id));
    let timing_match = recipe.required_timing.is_empty() || recipe.required_timing == timing;
    let sequence_match = sequence_matches(data, selected_items, &recipe.required_sequence);
    let catalyst_match = recipe.catalyst_tag.is_empty()
        || catalyst
            .map(|item| {
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == &recipe.catalyst_tag)
            })
            .unwrap_or(false);
    let elements = total_elements(&ingredient_items, catalyst);
    let minimum_elements_met = elements.meets(&recipe.minimum_elements);
    let room_bonus_applied = room_bonus_applies(station, &ingredient_items, catalyst);
    let quality_score = calculate_quality(
        recipe,
        station,
        &ingredient_items,
        catalyst,
        heat,
        stirs,
        timing_match,
        sequence_match,
        catalyst_match,
        room_bonus_applied,
        minimum_elements_met,
        mastery_brews,
    );
    let minimum_quality_met = quality_score >= recipe.minimum_quality;
    let process_match = recipe.required_heat == heat
        && recipe.required_stirs == stirs
        && timing_match
        && sequence_match
        && catalyst_match;
    let stable = process_match && minimum_quality_met && minimum_elements_met;
    let inherited_traits = inherited_traits(recipe, &ingredient_items, catalyst);
    let morph_output_item_id = if stable {
        morph_output(
            data,
            recipe,
            quality_score,
            catalyst,
            heat,
            stirs,
            timing,
            selected_items,
            room_bonus_applied,
        )
    } else {
        None
    };
    let failure_reasons = brew_failure_reasons(
        recipe,
        heat,
        stirs,
        timing_match,
        sequence_match,
        catalyst_match,
        minimum_quality_met,
        minimum_elements_met,
    );
    let morph_hint = if stable && morph_output_item_id.is_none() {
        morph_trigger_hint(
            data,
            recipe,
            quality_score,
            catalyst,
            heat,
            stirs,
            timing,
            selected_items,
            room_bonus_applied,
        )
    } else {
        None
    };
    let output_item_id = morph_output_item_id.clone().unwrap_or_else(|| {
        if stable {
            recipe.output_item_id.clone()
        } else {
            recipe.unstable_output_item_id.clone()
        }
    });
    let output_amount = recipe.output_amount + mastery_output_bonus(mastery_brews);

    BrewResolution {
        recipe: Some(recipe),
        output_item_id,
        output_amount,
        process_match,
        quality_score,
        quality_band: quality_band(quality_score),
        inherited_traits,
        mastery_stage: mastery_stage(mastery_brews + u32::from(stable)),
        morph_output_item_id,
        timing_match,
        sequence_match,
        catalyst_match,
        room_bonus_applied,
        minimum_quality_met,
        minimum_elements_met,
        failure_reasons,
        morph_hint,
    }
}

fn mastery_output_bonus(mastery_brews: u32) -> u32 {
    if mastery_brews >= 6 {
        1
    } else {
        0
    }
}
