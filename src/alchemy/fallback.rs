use std::collections::BTreeMap;

use crate::data::{GameData, ItemDefinition};

use super::quality::weighted_quality_average;

pub(super) fn salvage_quality(
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> u32 {
    let base = weighted_quality_average(ingredients) * ingredients.len().min(3) as u32 / 3;
    let catalyst_bonus = catalyst.map(|item| item.quality / 6).unwrap_or_default();
    (base + catalyst_bonus).min(40)
}

pub(super) fn fallback_traits(
    ingredients: &[&ItemDefinition],
    catalyst: Option<&ItemDefinition>,
) -> Vec<String> {
    let mut traits = Vec::new();
    for item in ingredients {
        for item_trait in &item.traits {
            if !traits.contains(item_trait) {
                traits.push(item_trait.clone());
            }
            if traits.len() >= 2 {
                return traits;
            }
        }
    }
    if let Some(catalyst) = catalyst {
        for item_trait in &catalyst.traits {
            if !traits.contains(item_trait) {
                traits.push(item_trait.clone());
            }
            if traits.len() >= 2 {
                break;
            }
        }
    }
    traits
}

/// What the salvage path can hand back when a mixture matches no recipe. None of
/// these is any recipe's declared output, so the content checks that reason about
/// what the game can produce have to be told about them separately.
#[cfg(test)]
pub(crate) const SALVAGE_OUTPUT_ITEM_IDS: [&str; 4] = [
    "soothing_tonic",
    "lantern_leak",
    "rush_draught",
    "murky_concoction",
];

pub(super) fn infer_trait_output<'a>(data: &'a GameData, selected_items: &[String]) -> &'a str {
    let mut traits = BTreeMap::<String, u32>::new();
    for item_id in selected_items {
        if let Some(item) = data.item(item_id) {
            for item_trait in &item.traits {
                *traits.entry(item_trait.clone()).or_insert(0) += 1;
            }
        }
    }

    let dominant = traits
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then(left.0.cmp(&right.0)))
        .map(|entry| entry.0);

    match dominant.as_deref() {
        Some("healing") => "soothing_tonic",
        Some("luminous") => "lantern_leak",
        Some("vigor") | Some("volatile") => "rush_draught",
        _ => "murky_concoction",
    }
}

#[cfg(test)]
mod tests {
    use super::{infer_trait_output, SALVAGE_OUTPUT_ITEM_IDS};

    /// The salvage list is duplicated knowledge by necessity — the match maps
    /// traits to ids and cannot be derived from the constant. Keep them in step
    /// by driving every branch and checking where it lands.
    #[test]
    fn every_salvage_branch_lands_in_the_declared_list() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let probes = [
            "whisper_moss",  // healing
            "arcane_dust",   // luminous
            "ember_root",    // vigor / volatile
            "quarry_lichen", // neither, falls through
        ];
        for probe in probes {
            let output = infer_trait_output(&data, &[probe.to_owned()]);
            assert!(
                SALVAGE_OUTPUT_ITEM_IDS.contains(&output),
                "{probe} salvaged to {output}, which is not in the declared salvage list"
            );
            assert!(
                data.item(output).is_some(),
                "salvage output {output} is not an item"
            );
        }
    }
}
