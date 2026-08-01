use crate::content::ui_format;
use crate::data::{GameData, RecipeDefinition};

pub(super) fn meta(mastery: u32, best_quality_band: Option<&str>, catalyst_tag: &str) -> String {
    let best = best_quality_band
        .map(str::to_owned)
        .unwrap_or_else(|| ui_format("inventory_best_unlogged", &[]));
    let catalyst = if catalyst_tag.is_empty() {
        ui_format("inventory_catalyst_any", &[])
    } else {
        ui_format("inventory_catalyst_specific", &[("tag", catalyst_tag)])
    };
    ui_format(
        "inventory_memory_meta",
        &[
            ("mastery", &mastery.to_string()),
            ("best", &best),
            ("catalyst", &catalyst),
        ],
    )
}

pub(super) fn detail(
    data: &GameData,
    recipe: &RecipeDefinition,
    inherited_traits: &[String],
) -> String {
    let mut parts = vec![ui_format(
        "inventory_memory_output",
        &[("item", data.item_name(&recipe.output_item_id))],
    )];
    if !recipe.required_sequence.is_empty() {
        parts.push(order(data, recipe));
    }
    parts.push(brew_steps(recipe));
    if !inherited_traits.is_empty() {
        parts.push(ui_format(
            "inventory_memory_traits",
            &[("traits", &inherited_traits.join(", "))],
        ));
    }
    if !recipe.morph_targets.is_empty() {
        parts.push(ui_format("inventory_memory_morph", &[]));
    }
    parts.join("  ")
}

fn brew_steps(recipe: &RecipeDefinition) -> String {
    ui_format(
        "inventory_memory_brew_steps",
        &[
            ("heat", &recipe.required_heat.to_string()),
            ("stirs", &recipe.required_stirs.to_string()),
            ("timing", &recipe.required_timing),
        ],
    )
}

fn order(data: &GameData, recipe: &RecipeDefinition) -> String {
    let sequence = recipe
        .required_sequence
        .iter()
        .map(|token| sequence_step(data, token))
        .collect::<Vec<_>>()
        .join(" -> ");
    ui_format("inventory_memory_order", &[("sequence", &sequence)])
}

/// A sequence step names either a specific reagent or a property any reagent
/// can satisfy. Rendering both through the item-name lookup made a trait read
/// as an ingredient nobody had ever seen.
fn sequence_step(data: &GameData, token: &str) -> String {
    match data.item(token) {
        Some(item) => item.name.clone(),
        None => ui_format("inventory_memory_order_trait", &[("trait", token)]),
    }
}
