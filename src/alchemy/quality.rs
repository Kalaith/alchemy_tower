use crate::content::ui_copy;
use crate::data::{ItemDefinition, RecipeDefinition, StationDefinition};

#[path = "quality_factors.rs"]
mod quality_factors;

pub(super) use self::quality_factors::weighted_quality_average;
use self::quality_factors::{
    preferred_trait_matches, shared_trait_bonus, synthesis_efficiency_bonus, total_synthesis_weight,
};

pub(crate) fn quality_band(score: u32) -> &'static str {
    match score {
        0..=19 => ui_copy("quality_band_crude"),
        20..=39 => ui_copy("quality_band_serviceable"),
        40..=59 => ui_copy("quality_band_fine"),
        60..=79 => ui_copy("quality_band_excellent"),
        _ => ui_copy("quality_band_masterwork"),
    }
}

/// Successful brews at which a recipe reaches the "mastered" stage. Kept as a
/// named constant so progression gates (`required_mastered_recipe`) and
/// `mastery_stage` agree on the threshold.
pub(crate) const MASTERED_BREW_COUNT: u32 = 7;

pub(crate) fn mastery_stage(successful_brews: u32) -> &'static str {
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
    // The ramp runs all the way to mastery. It used to cap one brew short, so
    // the seventh — the brew that flips the label and opens the mastery gates —
    // was the only one in the run that changed nothing.
    score += mastery_brews.min(MASTERED_BREW_COUNT) * 3;

    // Heat and stirs at the target brew cleanly; overcharging past the target
    // adds potency (see `volatility::overcharge_potency`) at the cost of
    // instability; underfiring/understirring still degrades the brew.
    if heat >= recipe.required_heat {
        score += 6;
    } else {
        score = score.saturating_sub((recipe.required_heat - heat).unsigned_abs() * 4);
    }

    if stirs >= recipe.required_stirs {
        score += 5;
    } else {
        score = score.saturating_sub(stirs.abs_diff(recipe.required_stirs) * 2);
    }

    score += super::volatility::overcharge_potency(recipe, heat, stirs);

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

    // Mastery means being able to make one particular thing the same way twice,
    // so a mastered formula never scores below its own bar. Reagents and process
    // still decide how far *above* it the brew lands — this only removes the
    // possibility of a mastered recipe failing on quality, which is what makes
    // the seventh brew worth reaching rather than a label.
    if mastery_brews >= MASTERED_BREW_COUNT {
        score = score.max(recipe.minimum_quality);
    }

    score.min(100)
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
            ingredients
                .iter()
                .any(|item| item.category.as_str() == category)
                || catalyst
                    .map(|item| item.category.as_str() == category)
                    .unwrap_or(false)
        });

    favored_trait_hit || favored_category_hit
}

#[cfg(test)]
mod tests {
    use super::MASTERED_BREW_COUNT;
    use crate::alchemy::resolve_brew;
    use crate::data::GameData;

    /// Everything a brew of one recipe's own reagents needs, run to spec.
    fn brew_at_mastery(data: &GameData, mastery_brews: u32) -> (u32, u32) {
        let recipe = data
            .recipes
            .iter()
            .find(|recipe| recipe.id == "healing_draught_recipe")
            .expect("the healing draught recipe should exist");
        let station = data
            .stations
            .iter()
            .find(|station| station.id == recipe.station_id)
            .expect("its bench should exist");
        let selected = recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient.item_id.clone())
            .collect::<Vec<_>>();
        let ingredients = selected
            .iter()
            .filter_map(|item_id| data.item(item_id))
            .cloned()
            .collect::<Vec<_>>();
        let resolution = resolve_brew(
            data,
            station,
            &selected,
            &ingredients,
            None,
            recipe.required_heat,
            recipe.required_stirs,
            &recipe.required_timing,
            mastery_brews,
        );
        (resolution.quality_score, resolution.output_amount)
    }

    /// The seventh clean brew is what flips a formula to "Mastered" and opens
    /// the mastery gates, and it used to be the one brew in the run that did
    /// nothing: the quality ramp, the stability ramp and the extra bottle all
    /// capped at six. The payoff now lands on the step that names it.
    #[test]
    fn the_brew_that_earns_mastery_is_worth_making() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let (before_quality, before_output) = brew_at_mastery(&data, MASTERED_BREW_COUNT - 1);
        let (mastered_quality, mastered_output) = brew_at_mastery(&data, MASTERED_BREW_COUNT);

        assert!(
            mastered_quality > before_quality,
            "mastery brewed no better than the step before it: {mastered_quality} vs {before_quality}"
        );
        assert!(
            mastered_output > before_output,
            "mastery yielded no more bottles: {mastered_output} vs {before_output}"
        );
    }

    /// Mastery is defined in this crate as being able to make one particular
    /// thing the same way twice, so a mastered formula cannot fail its own
    /// quality bar however poor the reagents are. Process and stability still
    /// apply — this only removes the one failure a practised hand would not make.
    #[test]
    fn a_mastered_formula_never_falls_below_its_own_bar() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let recipe = data
            .recipes
            .iter()
            .find(|recipe| recipe.minimum_quality > 0)
            .expect("some recipe should set a quality bar");
        let station = data
            .stations
            .iter()
            .find(|station| station.id == recipe.station_id)
            .expect("its bench should exist");
        let selected = recipe
            .ingredients
            .iter()
            .map(|ingredient| ingredient.item_id.clone())
            .collect::<Vec<_>>();

        // Deliberately awful reagents: every quality field zeroed.
        let ingredients = selected
            .iter()
            .filter_map(|item_id| data.item(item_id))
            .map(|item| {
                let mut poor = item.clone();
                poor.quality = 0;
                poor.synthesis_value = 0;
                poor
            })
            .collect::<Vec<_>>();

        let unmastered = resolve_brew(
            &data,
            station,
            &selected,
            &ingredients,
            None,
            recipe.required_heat,
            recipe.required_stirs,
            &recipe.required_timing,
            0,
        );
        let mastered = resolve_brew(
            &data,
            station,
            &selected,
            &ingredients,
            None,
            recipe.required_heat,
            recipe.required_stirs,
            &recipe.required_timing,
            MASTERED_BREW_COUNT,
        );

        assert!(
            mastered.quality_score >= recipe.minimum_quality,
            "a mastered formula scored {} against its own bar of {}",
            mastered.quality_score,
            recipe.minimum_quality
        );
        assert!(mastered.minimum_quality_met);
        assert!(
            mastered.quality_score > unmastered.quality_score,
            "mastery should be worth something with poor reagents too"
        );
    }
}
