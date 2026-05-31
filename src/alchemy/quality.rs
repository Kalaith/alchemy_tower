use crate::content::ui_copy;
use crate::data::{ItemDefinition, RecipeDefinition, StationDefinition};

#[path = "quality_factors.rs"]
mod quality_factors;

pub(super) use self::quality_factors::weighted_quality_average;
use self::quality_factors::{
    preferred_trait_matches, shared_trait_bonus, synthesis_efficiency_bonus,
    total_synthesis_weight,
};

pub(crate) fn quality_band(score: u32) -> &'static str {
    match score {
        0..=19 => ui_copy("quality_band_crude"),
        20..=39 => ui_copy("quality_band_serviceable"),
        40..=59 => ui_copy("quality_band_fine"),
        60..=79 => ui_copy("quality_band_excellent"),
        _ => ui_copy("quality_band_masterwork"),
    }
}

pub(crate) fn mastery_stage(successful_brews: u32) -> &'static str {
    match successful_brews {
        0 => ui_copy("mastery_stage_unknown"),
        1 => ui_copy("mastery_stage_guessed"),
        2..=3 => ui_copy("mastery_stage_discovered"),
        4..=6 => ui_copy("mastery_stage_refined"),
        _ => ui_copy("mastery_stage_mastered"),
    }
}

pub(super) fn calculate_quality(
    recipe: &RecipeDefinition,
    station: &StationDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing_match: bool,
    sequence_match: bool,
    catalyst_match: bool,
    room_bonus_applied: bool,
    minimum_elements_met: bool,
    mastery_brews: u32,
) -> u32 {
    let total_weight = total_synthesis_weight(ingredients);
    let mut score = weighted_quality_average(ingredients);

    score += synthesis_efficiency_bonus(ingredients, total_weight);
    score += shared_trait_bonus(ingredients) * 3;
    score += preferred_trait_matches(recipe, ingredients, catalyst) as u32 * 4;
    score += mastery_brews.min(6) * 3;

    if heat == recipe.required_heat {
        score += 6;
    } else {
        score = score.saturating_sub((heat - recipe.required_heat).unsigned_abs() * 4);
    }

    if stirs == recipe.required_stirs {
        score += 5;
    } else {
        score = score.saturating_sub(stirs.abs_diff(recipe.required_stirs) * 2);
    }

    if timing_match {
        score += 4;
    } else if !recipe.required_timing.is_empty() {
        score = score.saturating_sub(4);
    }

    if sequence_match {
        score += 5;
    } else if !recipe.required_sequence.is_empty() {
        score = score.saturating_sub(5);
    }

    if catalyst_match {
        if let Some(catalyst) = catalyst {
            score +=
                catalyst.quality / 4 + catalyst.synthesis_value + recipe.catalyst_quality_bonus;
        }
    } else if !recipe.catalyst_tag.is_empty() {
        score = score.saturating_sub(6);
    }

    if room_bonus_applied {
        score += station.room_bonus.quality_bonus;
    }

    if minimum_elements_met {
        score += 5;
    } else if recipe.minimum_elements.total() > 0 {
        score = score.saturating_sub(8);
    }

    score.min(100)
}

pub(super) fn room_bonus_applies(
    station: &StationDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> bool {
    if station.room_bonus.quality_bonus == 0 {
        return false;
    }
    let favored_trait_hit = station.room_bonus.favored_traits.iter().any(|trait_name| {
        ingredients.iter().any(|item| {
            item.traits
                .iter()
                .any(|item_trait| item_trait == trait_name)
        }) || catalyst
            .map(|item| {
                item.traits
                    .iter()
                    .any(|item_trait| item_trait == trait_name)
            })
            .unwrap_or(false)
    });
    let favored_category_hit = station
        .room_bonus
        .favored_categories
        .iter()
        .any(|category| {
            ingredients
                .iter()
                .any(|item| item.category.as_str() == category)
                || catalyst
                    .map(|item| item.category.as_str() == category)
                    .unwrap_or(false)
        });

    favored_trait_hit || favored_category_hit
}
