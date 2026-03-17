use crate::data::{GameData, RecipeDefinition, StationDefinition};

use super::fallback::{fallback_traits, infer_trait_output, salvage_quality};
use super::matching::{match_recipe, selected_item_defs, total_elements};
use super::morphs::{morph_output, morph_trigger_hint};
use super::quality::{calculate_quality, quality_band, room_bonus_applies};
use super::traits::inherited_traits;

pub struct BrewResolution<'a> {
    pub recipe: Option<&'a RecipeDefinition>,
    pub output_item_id: String,
    pub output_amount: u32,
    pub process_match: bool,
    pub quality_score: u32,
    pub quality_band: &'static str,
    pub inherited_traits: Vec<String>,
    pub mastery_stage: &'static str,
    pub morph_output_item_id: Option<String>,
    pub timing_match: bool,
    pub sequence_match: bool,
    pub catalyst_match: bool,
    pub room_bonus_applied: bool,
    pub minimum_quality_met: bool,
    pub minimum_elements_met: bool,
    pub failure_reasons: Vec<String>,
    pub morph_hint: Option<String>,
}

pub fn resolve_brew<'a>(
    data: &'a GameData,
    station: &StationDefinition,
    selected_items: &[String],
    catalyst_item: Option<&str>,
    heat: i32,
    stirs: u32,
    timing: &str,
    mastery_brews: u32,
) -> BrewResolution<'a> {
    if let Some(recipe) = match_recipe(data, station, selected_items) {
        let ingredient_items = selected_item_defs(data, selected_items);
        let catalyst = catalyst_item.and_then(|item_id| data.item(item_id));
        let timing_match = recipe.required_timing.is_empty() || recipe.required_timing == timing;
        let sequence_match =
            super::matching::sequence_matches(data, selected_items, &recipe.required_sequence);
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

        return BrewResolution {
            recipe: Some(recipe),
            output_item_id,
            output_amount,
            process_match,
            quality_score,
            quality_band: quality_band(quality_score),
            inherited_traits,
            mastery_stage: super::quality::mastery_stage(mastery_brews + u32::from(stable)),
            morph_output_item_id,
            timing_match,
            sequence_match,
            catalyst_match,
            room_bonus_applied,
            minimum_quality_met,
            minimum_elements_met,
            failure_reasons,
            morph_hint,
        };
    }

    let ingredient_items = selected_item_defs(data, selected_items);
    let catalyst = catalyst_item.and_then(|item_id| data.item(item_id));
    let quality_score = salvage_quality(&ingredient_items, catalyst);
    BrewResolution {
        recipe: None,
        output_item_id: infer_trait_output(data, selected_items).to_owned(),
        output_amount: 1,
        process_match: false,
        quality_score,
        quality_band: quality_band(quality_score),
        inherited_traits: fallback_traits(&ingredient_items, catalyst),
        mastery_stage: super::quality::mastery_stage(0),
        morph_output_item_id: None,
        timing_match: false,
        sequence_match: false,
        catalyst_match: catalyst.is_none(),
        room_bonus_applied: false,
        minimum_quality_met: false,
        minimum_elements_met: false,
        failure_reasons: vec!["No known base recipe.".to_owned()],
        morph_hint: None,
    }
}

fn brew_failure_reasons(
    recipe: &RecipeDefinition,
    heat: i32,
    stirs: u32,
    timing_match: bool,
    sequence_match: bool,
    catalyst_match: bool,
    minimum_quality_met: bool,
    minimum_elements_met: bool,
) -> Vec<String> {
    let mut reasons = Vec::new();

    if heat < recipe.required_heat {
        reasons.push("Heat too low.".to_owned());
    } else if heat > recipe.required_heat {
        reasons.push("Heat too high.".to_owned());
    }

    if stirs < recipe.required_stirs {
        reasons.push("Too few stirs.".to_owned());
    } else if stirs > recipe.required_stirs {
        reasons.push("Too many stirs.".to_owned());
    }

    if !timing_match && !recipe.required_timing.is_empty() {
        reasons.push("Timing is off.".to_owned());
    }
    if !sequence_match && !recipe.required_sequence.is_empty() {
        reasons.push("Wrong ingredient sequence.".to_owned());
    }
    if !catalyst_match && !recipe.catalyst_tag.is_empty() {
        reasons.push("Catalyst mismatch.".to_owned());
    }
    if !minimum_elements_met && recipe.minimum_elements.total() > 0 {
        reasons.push("Element threshold missed.".to_owned());
    }
    if !minimum_quality_met && recipe.minimum_quality > 0 {
        reasons.push("Quality too low to stabilize the recipe.".to_owned());
    }

    reasons
}

fn mastery_output_bonus(mastery_brews: u32) -> u32 {
    if mastery_brews >= 6 {
        1
    } else {
        0
    }
}
