pub(super) const SLOT_COUNT: usize = 3;
pub(super) const ALCHEMY_TIMINGS: [&str; 3] = ["steady", "early", "late"];

/// What the heat dial can actually be set to. A recipe or morph asking for
/// anything outside this range can never fire, however well authored it is.
pub(crate) const ALCHEMY_MIN_HEAT: i32 = 1;
pub(crate) const ALCHEMY_MAX_HEAT: i32 = 3;

#[derive(Clone, Debug)]
pub(super) struct AlchemySession {
    pub(super) index: usize,
    pub(super) heat: i32,
    pub(super) stirs: u32,
    pub(super) timing_index: usize,
    pub(super) slots: [Option<String>; SLOT_COUNT],
    pub(super) catalyst: Option<String>,
}

impl Default for AlchemySession {
    fn default() -> Self {
        Self {
            index: 0,
            heat: 2,
            stirs: 0,
            timing_index: 0,
            slots: [None, None, None],
            catalyst: None,
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct SavedAlchemySetup {
    pub(super) heat: i32,
    pub(super) stirs: u32,
    pub(super) timing_index: usize,
    pub(super) slots: [Option<String>; SLOT_COUNT],
    pub(super) catalyst: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ALCHEMY_MAX_HEAT, ALCHEMY_MIN_HEAT, ALCHEMY_TIMINGS};

    /// A recipe or morph is only content if the bench can actually be set to
    /// what it asks for. A heat the dial cannot reach, a timing that is not one
    /// of the three, or a catalyst tag no item carries all produce a branch that
    /// sits in the data and never once fires.
    #[test]
    fn every_recipe_and_morph_is_reachable_at_the_bench() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let available_tags = data
            .items
            .iter()
            .flat_map(|item| item.catalyst_tags.iter().cloned())
            .collect::<std::collections::HashSet<_>>();

        let mut unreachable = Vec::new();
        let mut check = |label: String, heat: i32, timing: &str, tag: &str| {
            if !(ALCHEMY_MIN_HEAT..=ALCHEMY_MAX_HEAT).contains(&heat) {
                unreachable.push(format!("{label}: heat {heat} is off the dial"));
            }
            if !timing.is_empty() && !ALCHEMY_TIMINGS.contains(&timing) {
                unreachable.push(format!("{label}: timing '{timing}' is not a setting"));
            }
            if !tag.is_empty() && !available_tags.contains(tag) {
                unreachable.push(format!("{label}: no catalyst carries the '{tag}' tag"));
            }
        };

        for recipe in &data.recipes {
            check(
                recipe.id.clone(),
                recipe.required_heat,
                &recipe.required_timing,
                &recipe.catalyst_tag,
            );
            for morph in &recipe.morph_targets {
                check(
                    format!("{} -> {}", recipe.id, morph.output_item_id),
                    morph.required_heat,
                    &morph.required_timing,
                    &morph.catalyst_tag,
                );
            }
        }

        assert!(
            unreachable.is_empty(),
            "recipes or morphs the bench can never satisfy:\n{unreachable:#?}"
        );
    }
}
