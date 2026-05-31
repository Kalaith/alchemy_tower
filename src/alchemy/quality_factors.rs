use std::collections::BTreeMap;

use crate::data::{ItemDefinition, RecipeDefinition};

pub(super) fn total_synthesis_weight(ingredients: &[&ItemDefinition]) -> u32 {
    ingredients
        .iter()
        .map(|item| item.synthesis_weight.max(1))
        .sum::<u32>()
        .max(1)
}

pub(in crate::alchemy) fn weighted_quality_average(ingredients: &[&ItemDefinition]) -> u32 {
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

pub(super) fn synthesis_efficiency_bonus(
    ingredients: &[&ItemDefinition],
    total_weight: u32,
) -> u32 {
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

pub(super) fn shared_trait_bonus(ingredients: &[&ItemDefinition]) -> u32 {
    let mut counts = BTreeMap::<String, u32>::new();
    for item in ingredients {
        for item_trait in &item.traits {
            *counts.entry(item_trait.clone()).or_insert(0) += 1;
        }
    }
    counts.values().filter(|count| **count > 1).count() as u32
}

pub(super) fn preferred_trait_matches(
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
