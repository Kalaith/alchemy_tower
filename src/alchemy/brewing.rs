use crate::content::ui_copy;
use crate::data::{GameData, ItemDefinition, RecipeDefinition, StationDefinition};

use super::fallback::{fallback_traits, infer_trait_output, salvage_quality};
use super::matching::match_recipe;
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
    /// Deterministic overcharge/volatility pressure (0 = calm). At or above
    /// `INSTABILITY_LIMIT` the brew collapses; see `destabilized`.
    pub(crate) instability: u32,
    /// True when instability has crossed the collapse threshold, forcing the
    /// unstable output even if the recipe/process/quality checks pass.
    pub(crate) destabilized: bool,
    pub(crate) failure_reasons: Vec<String>,
    pub(crate) morph_hint: Option<String>,
}

impl BrewResolution<'_> {
    /// Whether this brew came out clean. Every consumer asks here rather than
    /// re-deriving it: the experiment log used to spell the rule out itself and
    /// left `destabilized` off, so an overcharge collapse filed as a stable
    /// brew in the archive — and because potion memory is rebuilt from that log
    /// on load, a save/load flipped the recorded stability of the collapse.
    pub(crate) fn is_stable(&self) -> bool {
        stable_brew(
            self.process_match,
            self.minimum_quality_met,
            self.minimum_elements_met,
            self.destabilized,
        )
    }
}

/// The one definition of a clean brew. `resolve_known_recipe_brew` needs the
/// answer while it is still assembling the resolution — to pick the output item
/// and decide whether a morph triggers — so the rule lives here as a function
/// rather than only as a method.
pub(super) fn stable_brew(
    process_match: bool,
    minimum_quality_met: bool,
    minimum_elements_met: bool,
    destabilized: bool,
) -> bool {
    process_match && minimum_quality_met && minimum_elements_met && !destabilized
}

/// `ingredients` is what the brew is actually made from — the same reagents the
/// slots name, but with any wild variant the player gathered folded in. Passing
/// them rather than looking them up by id is what lets a variant's quality,
/// traits and elements reach every calculation below without any of them
/// knowing variants exist. Callers with no variant stock to consult use
/// `selected_item_defs`.
pub(crate) fn resolve_brew<'a>(
    data: &'a GameData,
    station: &StationDefinition,
    selected_items: &[String],
    ingredients: &[ItemDefinition],
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
            ingredients,
            catalyst_item,
            heat,
            stirs,
            timing,
            mastery_brews,
            recipe,
        );
    }

    let ingredient_items = ingredients.iter().collect::<Vec<_>>();
    let catalyst = catalyst_item.and_then(|item_id| data.item(item_id));
    let quality_score = salvage_quality(&ingredient_items, catalyst, mastery_brews);
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
        instability: 0,
        destabilized: false,
        failure_reasons: vec![ui_copy("brew_failure_no_recipe").to_owned()],
        morph_hint: None,
    }
}
