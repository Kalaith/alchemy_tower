use crate::data::{GameData, ItemDefinition, RecipeDefinition};
use crate::content::{ui_copy, ui_format};

use super::matching::sequence_matches;

pub(super) fn morph_output(
    data: &GameData,
    recipe: &RecipeDefinition,
    quality_score: u32,
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing: &str,
    selected_items: &[String],
    room_bonus_applied: bool,
) -> Option<String> {
    for morph in &recipe.morph_targets {
        let catalyst_ok = morph.catalyst_tag.is_empty()
            || catalyst
                .map(|item| {
                    item.catalyst_tags
                        .iter()
                        .any(|tag| tag == &morph.catalyst_tag)
                })
                .unwrap_or(false);
        let timing_ok = morph.required_timing.is_empty() || morph.required_timing == timing;
        let sequence_ok = sequence_matches(data, selected_items, &morph.required_sequence);
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
    data: &GameData,
    recipe: &RecipeDefinition,
    quality_score: u32,
    catalyst: Option<&ItemDefinition>,
    heat: i32,
    stirs: u32,
    timing: &str,
    selected_items: &[String],
    room_bonus_applied: bool,
) -> Option<String> {
    let morph = recipe.morph_targets.first()?;

    if quality_score < morph.minimum_quality {
        return Some(ui_copy("morph_hint_quality").to_owned());
    }
    if !morph.catalyst_tag.is_empty()
        && !catalyst
            .map(|item| {
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == &morph.catalyst_tag)
            })
            .unwrap_or(false)
    {
        return Some(ui_format("morph_hint_catalyst", &[("tag", &morph.catalyst_tag)]));
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
        && !sequence_matches(data, selected_items, &morph.required_sequence)
    {
        return Some(ui_copy("morph_hint_sequence").to_owned());
    }
    if morph.room_bonus_required && !room_bonus_applied {
        return Some(ui_copy("morph_hint_room").to_owned());
    }

    Some(ui_copy("morph_hint_generic").to_owned())
}
