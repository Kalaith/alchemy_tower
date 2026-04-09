use std::collections::BTreeMap;

use crate::content::ui_copy;
use crate::data::{ItemDefinition, RecipeDefinition, StationDefinition};

pub fn quality_band(score: u32) -> &'static str {
    match score {
        0..=19 => ui_copy("quality_band_crude"),
        20..=39 => ui_copy("quality_band_serviceable"),
        40..=59 => ui_copy("quality_band_fine"),
        60..=79 => ui_copy("quality_band_excellent"),
        _ => ui_copy("quality_band_masterwork"),
    }
}

pub fn mastery_stage(successful_brews: u32) -> &'static str {
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

fn total_synthesis_weight(ingredients: &[&ItemDefinition]) -> u32 {
    ingredients
        .iter()
        .map(|item| item.synthesis_weight.max(1))
        .sum::<u32>()
        .max(1)
}

pub(super) fn weighted_quality_average(ingredients: &[&ItemDefinition]) -> u32 {
    if ingredients.is_empty() {
        return 0;
    }

    let total_weight = total_synthesis_weight(ingredients);
    ingredients
        .iter()
        .map(|item| item.quality * item.synthesis_weight.max(1))
        .sum::<u32>()
        / total_weight
}

fn synthesis_efficiency_bonus(ingredients: &[&ItemDefinition], total_weight: u32) -> u32 {
    if ingredients.is_empty() {
        return 0;
    }

    ingredients
        .iter()
        .map(|item| item.synthesis_value)
        .sum::<u32>()
        .saturating_mul(2)
        / total_weight
}

fn shared_trait_bonus(ingredients: &[&ItemDefinition]) -> u32 {
    let mut counts = BTreeMap::<String, u32>::new();
    for item in ingredients {
        for item_trait in &item.traits {
            *counts.entry(item_trait.clone()).or_insert(0) += 1;
        }
    }
    counts.values().filter(|count| **count > 1).count() as u32
}

fn preferred_trait_matches(
    recipe: &RecipeDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> usize {
    recipe
        .preferred_traits
        .iter()
        .filter(|preferred| {
            ingredients.iter().any(|item| {
                item.traits
                    .iter()
                    .any(|item_trait| item_trait == *preferred)
            }) || catalyst
                .map(|item| {
                    item.traits
                        .iter()
                        .any(|item_trait| item_trait == *preferred)
                })
                .unwrap_or(false)
        })
        .count()
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
            ingredients.iter().any(|item| item.category.as_str() == category)
                || catalyst
                    .map(|item| item.category.as_str() == category)
                    .unwrap_or(false)
        });

    favored_trait_hit || favored_category_hit
}

#[cfg(test)]
mod tests {
    use crate::data::{ElementProfile, ItemCategory, ItemDefinition};

    use super::{synthesis_efficiency_bonus, total_synthesis_weight, weighted_quality_average};

    fn test_item(quality: u32, synthesis_weight: u32, synthesis_value: u32) -> ItemDefinition {
        ItemDefinition {
            id: format!("item_{quality}_{synthesis_weight}_{synthesis_value}"),
            name: "Test Item".to_owned(),
            category: ItemCategory::Ingredient,
            base_value: 1,
            color: [0, 0, 0, 255],
            description: String::new(),
            quality,
            rarity: 1,
            elements: ElementProfile::default(),
            traits: Vec::new(),
            source_conditions: Vec::new(),
            wild_variants: Vec::new(),
            synthesis_weight,
            synthesis_value,
            catalyst_tags: Vec::new(),
            effects: Vec::new(),
        }
    }

    #[test]
    fn weighted_quality_average_uses_synthesis_weight() {
        let light = test_item(20, 1, 2);
        let heavy = test_item(80, 3, 2);
        let ingredients = vec![&light, &heavy];

        assert_eq!(weighted_quality_average(&ingredients), 65);
    }

    #[test]
    fn synthesis_efficiency_bonus_drops_for_heavier_mixes() {
        let light_a = test_item(20, 1, 3);
        let light_b = test_item(20, 1, 3);
        let heavy_a = test_item(20, 3, 3);
        let heavy_b = test_item(20, 3, 3);

        let light_mix = vec![&light_a, &light_b];
        let heavy_mix = vec![&heavy_a, &heavy_b];

        assert_eq!(
            synthesis_efficiency_bonus(&light_mix, total_synthesis_weight(&light_mix)),
            6
        );
        assert_eq!(
            synthesis_efficiency_bonus(&heavy_mix, total_synthesis_weight(&heavy_mix)),
            2
        );
    }
}
