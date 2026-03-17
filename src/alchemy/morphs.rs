use crate::data::{GameData, ItemDefinition, RecipeDefinition};

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
        return Some("A finer brew might reveal a hidden branch.".to_owned());
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
        return Some(format!(
            "A {} catalyst could bend this recipe into a new shape.",
            morph.catalyst_tag
        ));
    }
    if heat != morph.required_heat {
        return Some(format!(
            "A different heat could push this brew into a new branch. Try heat {}.",
            morph.required_heat
        ));
    }
    if stirs != morph.required_stirs {
        return Some(format!(
            "An altered stirring pattern may trigger a morph. Aim for {} stir(s).",
            morph.required_stirs
        ));
    }
    if !morph.required_timing.is_empty() && morph.required_timing != timing {
        return Some(format!(
            "The branch seems sensitive to timing. Try a {} finish.",
            morph.required_timing
        ));
    }
    if !morph.required_sequence.is_empty()
        && !sequence_matches(data, selected_items, &morph.required_sequence)
    {
        return Some("A different ingredient order may unlock a hidden branch.".to_owned());
    }
    if morph.room_bonus_required && !room_bonus_applied {
        return Some("This formula may branch only when the room itself favors it.".to_owned());
    }

    Some("The recipe is straining toward another form.".to_owned())
}
