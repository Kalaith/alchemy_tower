use std::collections::BTreeMap;

use crate::data::{ItemDefinition, RecipeDefinition};

pub(super) fn inherited_traits(
    recipe: &RecipeDefinition,
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> Vec<String> {
    let mut inherited = Vec::<String>::new();
    for guaranteed in &recipe.guaranteed_traits {
        if !inherited.contains(guaranteed) {
            inherited.push(guaranteed.clone());
        }
    }

    let mut counts = BTreeMap::<String, u32>::new();
    for item in ingredients {
        for item_trait in &item.traits {
            *counts.entry(item_trait.clone()).or_insert(0) += 1;
        }
    }
    if let Some(catalyst) = catalyst {
        for item_trait in &catalyst.traits {
            *counts.entry(item_trait.clone()).or_insert(0) += 1;
        }
    }

    let mut ranked = counts.into_iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(&right.0)));

    for preferred in &recipe.preferred_traits {
        if ranked.iter().any(|(trait_name, _)| trait_name == preferred)
            && !inherited.contains(preferred)
        {
            inherited.push(preferred.clone());
        }
    }

    for (trait_name, count) in ranked {
        if count > 1 && !inherited.contains(&trait_name) {
            inherited.push(trait_name);
        }
        if inherited.len() >= 2 {
            break;
        }
    }

    if inherited.len() < 2 {
        for item in ingredients {
            for item_trait in &item.traits {
                if !inherited.contains(item_trait) {
                    inherited.push(item_trait.clone());
                }
                if inherited.len() >= 2 {
                    break;
                }
            }
            if inherited.len() >= 2 {
                break;
            }
        }
    }

    inherited.truncate(2);
    inherited
}
