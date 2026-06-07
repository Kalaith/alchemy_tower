use crate::content::ui_copy;
use crate::data::{GameData, RecipeDefinition, StationDefinition};

use super::fallback::{fallback_traits, infer_trait_output, salvage_quality};
use super::matching::{match_recipe, selected_item_defs};
use super::quality::quality_band;

#[path = "brewing_failures.rs"]
mod brewing_failures;
#[path = "brewing_known.rs"]
mod brewing_known;

use self::brewing_known::resolve_known_recipe_brew;

pub(crate) struct BrewResolution<'a> {
    pub(crate) recipe: Option<&'a RecipeDefinition>,
    pub(crate) output_item_id: String,
    pub(crate) output_amount: u32,
    pub(crate) process_match: bool,
    pub(crate) quality_score: u32,
    pub(crate) quality_band: &'static str,
    pub(crate) inherited_traits: Vec<String>,
    pub(crate) mastery_stage: &'static str,
    pub(crate) morph_output_item_id: Option<String>,
    pub(crate) timing_match: bool,
    pub(crate) sequence_match: bool,
    pub(crate) catalyst_match: bool,
    pub(crate) room_bonus_applied: bool,
    pub(crate) minimum_quality_met: bool,
    pub(crate) minimum_elements_met: bool,
    pub(crate) failure_reasons: Vec<String>,
    pub(crate) morph_hint: Option<String>,
}

pub(crate) fn resolve_brew<'a>(
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
        return resolve_known_recipe_brew(
            data,
            station,
            selected_items,
            catalyst_item,
            heat,
            stirs,
            timing,
            mastery_brews,
            recipe,
        );
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
        failure_reasons: vec![ui_copy("brew_failure_no_recipe").to_owned()],
        morph_hint: None,
    }
}
