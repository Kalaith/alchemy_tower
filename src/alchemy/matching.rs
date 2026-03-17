use std::collections::BTreeMap;

use crate::data::{ElementProfile, GameData, ItemDefinition, RecipeDefinition, StationDefinition};

pub fn match_recipe<'a>(
    data: &'a GameData,
    station: &StationDefinition,
    selected_items: &[String],
) -> Option<&'a RecipeDefinition> {
    let selected_counts = item_counts(selected_items);

    data.recipes.iter().find(|recipe| {
        recipe.station_id == station.id
            && recipe.ingredients.len() == selected_counts.len()
            && recipe.ingredients.iter().all(|ingredient| {
                selected_counts.get(&ingredient.item_id) == Some(&ingredient.amount)
            })
    })
}

fn item_counts(selected_items: &[String]) -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    for item_id in selected_items {
        *counts.entry(item_id.clone()).or_insert(0) += 1;
    }
    counts
}

pub(super) fn selected_item_defs<'a>(
    data: &'a GameData,
    selected_items: &[String],
) -> Vec<&'a ItemDefinition> {
    selected_items
        .iter()
        .filter_map(|item_id| data.item(item_id))
        .collect()
}

pub(super) fn total_elements(
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> ElementProfile {
    let mut total = ElementProfile::default();
    for item in ingredients {
        total.add_assign(&item.elements);
    }
    if let Some(catalyst) = catalyst {
        total.add_assign(&catalyst.elements);
    }
    total
}

pub(super) fn sequence_matches(
    data: &GameData,
    selected_items: &[String],
    required_sequence: &[String],
) -> bool {
    if required_sequence.is_empty() {
        return true;
    }
    if selected_items.len() < required_sequence.len() {
        return false;
    }

    selected_items
        .iter()
        .zip(required_sequence.iter())
        .all(|(item_id, token)| {
            data.item(item_id)
                .map(|item| sequence_token_matches(item, token))
                .unwrap_or(false)
        })
}

fn sequence_token_matches(item: &ItemDefinition, token: &str) -> bool {
    item.id == token
        || item.category.as_str() == token
        || item.traits.iter().any(|item_trait| item_trait == token)
}
