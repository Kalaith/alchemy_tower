//! Where to go looking for a formula you have not worked out yet.
//!
//! Three of the game's sixty-two recipes are `starter_known`. The other
//! fifty-nine are learned by putting the exact right reagents in the exact
//! right pot — discovery is the design, and this does not change it. What the
//! game gave the player to work with was the problem: the herb journal, which
//! is the game's own memory of everything gathered, said of every one of those
//! fifty-nine only *"Used in formulae you have not yet discovered."* A count,
//! with no direction in it at all, against forty-six two-reagent formulae
//! (1,485 pairs across the fifty-four things that can go in a pot) and sixteen
//! three-reagent ones.
//!
//! The journal points now. It still names neither the formula nor the reagent —
//! it names **where the missing half comes from**: ground you could walk to, a
//! counter that stocks it, or the fact that it has to be brewed rather than
//! picked, which is also how the second-order tier announces that it exists.
//! That turns guessing into a decision about where to go, which is this game's
//! outer loop.

use super::GameplayState;
use crate::data::{AreaDefinition, GameData, RecipeDefinition};

#[path = "gameplay_formula_hint_text.rs"]
mod formula_hint_text;

/// Where a reagent comes from, in the terms a player would act on.
enum ReagentOrigin<'a> {
    /// Ground with a node on it.
    Ground(&'a str),
    /// A counter that stocks it.
    Counter(&'a str),
    /// Something the tower makes rather than the valley grows.
    Bench,
}

/// What the nearest unlogged formula still asks the player to do about it.
enum NearestUse<'a> {
    /// Everything else it wants is already in the journal or the bag, so the
    /// only thing left is the bench it wants working at — which for a player
    /// who has walked most of the valley is the whole of the useful answer.
    OnlyTheBench(&'a str),
    /// The first thing in it they have never seen.
    Missing(&'a str),
}

impl GameplayState {
    /// The clause that points at the nearest formula this reagent feeds and
    /// the player has not logged. `None` once every use is known.
    pub(super) fn undiscovered_formula_hint(
        &self,
        data: &GameData,
        item_id: &str,
    ) -> Option<String> {
        match self.nearest_unlogged_use(data, item_id)? {
            NearestUse::OnlyTheBench(station) => Some(formula_hint_text::only_the_bench(station)),
            NearestUse::Missing(unmet) => Some(match self.reagent_origin(data, unmet)? {
                ReagentOrigin::Ground(place) => formula_hint_text::out_of(place),
                ReagentOrigin::Counter(counter) => formula_hint_text::stocked_by(counter),
                ReagentOrigin::Bench => formula_hint_text::brewed_not_picked(),
            }),
        }
    }

    /// The unlogged formula using this reagent that asks least of the player,
    /// and the first thing in it they have never seen. Ordered by how much is
    /// still missing, then by how short the formula is, then by id so the
    /// journal does not reshuffle itself between frames.
    fn nearest_unlogged_use<'a>(
        &self,
        data: &'a GameData,
        item_id: &str,
    ) -> Option<NearestUse<'a>> {
        let mut best: Option<(usize, usize, &RecipeDefinition, Option<&str>)> = None;
        for recipe in &data.recipes {
            if self.recipe_is_known(&recipe.id) {
                continue;
            }
            if !recipe
                .ingredients
                .iter()
                .any(|ingredient| ingredient.item_id == item_id)
            {
                continue;
            }
            let mut unmet_count = 0usize;
            let mut first_unmet = None;
            for ingredient in &recipe.ingredients {
                if ingredient.item_id == item_id || self.has_met_reagent(&ingredient.item_id) {
                    continue;
                }
                unmet_count += 1;
                first_unmet.get_or_insert(ingredient.item_id.as_str());
            }
            let better = best.as_ref().is_none_or(|(count, slots, current, _)| {
                (unmet_count, recipe.ingredients.len(), recipe.id.as_str())
                    < (*count, *slots, current.id.as_str())
            });
            if better {
                best = Some((unmet_count, recipe.ingredients.len(), recipe, first_unmet));
            }
        }
        best.and_then(|(_, _, recipe, unmet)| match unmet {
            Some(item_id) => Some(NearestUse::Missing(item_id)),
            None => data
                .stations
                .iter()
                .find(|station| station.id == recipe.station_id)
                .map(|station| NearestUse::OnlyTheBench(&station.name)),
        })
    }

    /// Has this ever been in the player's hands or their journal? Held counts
    /// as met as well as remembered, because catalysts and runes keep no
    /// memory entry of their own.
    fn has_met_reagent(&self, item_id: &str) -> bool {
        self.progression.herb_memories.contains_key(item_id)
            || self.progression.potion_memories.contains_key(item_id)
            || self.inventory.get(item_id).copied().unwrap_or_default() > 0
    }

    /// Ground before counters before benches — the valley is the answer where
    /// the valley has one, and a thing that can be picked should send the
    /// player out rather than to a shelf.
    fn reagent_origin<'a>(&self, data: &'a GameData, item_id: &str) -> Option<ReagentOrigin<'a>> {
        // Ground the player can work beats ground they cannot. Twenty-one of
        // the valley's nodes wait on a finished quest or a treated thing, and a
        // reagent that grows in four places should not send somebody up a floor
        // that is still shut when it also grows in the plains. Season, weather
        // and hour are deliberately *not* consulted: the hint answers where,
        // and the conditions line above it already answers when.
        if let Some(area) = self
            .area_growing(data, item_id, true)
            .or_else(|| self.area_growing(data, item_id, false))
        {
            return Some(ReagentOrigin::Ground(&area.name));
        }
        if let Some(area) = data
            .stations
            .iter()
            .find(|station| station.habitat_output_item_id == item_id)
            .and_then(|station| data.area(&station.area_id))
        {
            return Some(ReagentOrigin::Ground(&area.name));
        }
        if let Some(station) = data
            .stations
            .iter()
            .find(|station| station.stock.iter().any(|stock| stock.item_id == item_id))
        {
            return Some(ReagentOrigin::Counter(&station.name));
        }
        let brewed = data.recipes.iter().any(|recipe| {
            recipe.output_item_id == item_id
                || recipe.unstable_output_item_id == item_id
                || recipe
                    .morph_targets
                    .iter()
                    .any(|morph| morph.output_item_id == item_id)
        }) || data
            .rune_recipes
            .iter()
            .any(|recipe| recipe.output_item_id == item_id);
        brewed.then_some(ReagentOrigin::Bench)
    }

    /// The first area carrying a node for this reagent. `open_only` skips
    /// ground still waiting on a quest or a journal beat.
    fn area_growing<'a>(
        &self,
        data: &'a GameData,
        item_id: &str,
        open_only: bool,
    ) -> Option<&'a AreaDefinition> {
        data.areas.iter().find(|area| {
            area.gather_nodes.iter().any(|node| {
                node.item_id == item_id
                    && (!open_only
                        || (self.story_gate_is_open(&node.required_completed_quest, true)
                            && self.story_gate_is_open(&node.required_journal_milestone, false)))
            })
        })
    }

    /// An empty gate is an open one; the rest is the same pair of checks
    /// `refresh_available_nodes` makes before a node is drawn at all.
    fn story_gate_is_open(&self, gate_id: &str, is_quest: bool) -> bool {
        if gate_id.is_empty() {
            return true;
        }
        if is_quest {
            self.progression.completed_quests.contains(gate_id)
        } else {
            self.has_journal_milestone(gate_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GameplayState, ReagentOrigin};

    /// A hint that sends the player to ground still waiting on a quest is worse
    /// than no hint, because they cannot tell the difference between "not here"
    /// and "not yet". Naming the first area in file order did exactly that:
    /// whisper moss grows in seven places and the terraces under the tower wall
    /// — which open at the end of Brin's arc — sorted ahead of the plains.
    #[test]
    fn a_reagent_is_pointed_at_ground_that_is_already_open() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);

        let mut shut = Vec::new();
        for item in &data.items {
            let open_somewhere = state.area_growing(&data, &item.id, true).is_some();
            if !open_somewhere {
                continue;
            }
            let Some(ReagentOrigin::Ground(named)) = state.reagent_origin(&data, &item.id) else {
                continue;
            };
            let area = data
                .areas
                .iter()
                .find(|area| area.name == named)
                .expect("the hint should name a real area");
            let workable = area.gather_nodes.iter().any(|node| {
                node.item_id == item.id
                    && state.story_gate_is_open(&node.required_completed_quest, true)
                    && state.story_gate_is_open(&node.required_journal_milestone, false)
            });
            if !workable {
                shut.push(format!("{} sent to {named}", item.id));
            }
        }

        assert!(
            shut.is_empty(),
            "reagents pointed at ground that is still shut: {shut:#?}"
        );
    }

    /// The rule this pass exists to make true: every reagent in the game, at a
    /// new game, points somewhere for at least one of the formulae it feeds.
    /// A reagent whose origin cannot be named is one the journal would send the
    /// player looking for with no direction, which is the state the whole shelf
    /// was in.
    #[test]
    fn every_undiscovered_formula_a_reagent_feeds_points_somewhere() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);

        let mut silent = Vec::new();
        for item in &data.items {
            let feeds = data.recipes.iter().any(|recipe| {
                recipe
                    .ingredients
                    .iter()
                    .any(|ingredient| ingredient.item_id == item.id)
            });
            if !feeds {
                continue;
            }
            if state.undiscovered_formula_hint(&data, &item.id).is_none() {
                silent.push(item.id.clone());
            }
        }

        assert!(
            silent.is_empty(),
            "reagents whose unlogged formulae point nowhere: {silent:#?}"
        );
    }

    /// The hint must not become the answer. It says where the missing half
    /// comes from and never what it is or what the formula is called.
    #[test]
    fn a_hint_names_neither_the_formula_nor_the_reagent_it_wants() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);

        for item in &data.items {
            let Some(hint) = state.undiscovered_formula_hint(&data, &item.id) else {
                continue;
            };
            for recipe in &data.recipes {
                assert!(
                    !hint.contains(&recipe.name),
                    "{}'s hint names the formula {}: {hint}",
                    item.id,
                    recipe.name
                );
            }
            for other in &data.items {
                if other.id == item.id || state.has_met_reagent(&other.id) {
                    continue;
                }
                assert!(
                    !hint.contains(&other.name),
                    "{}'s hint names {}: {hint}",
                    item.id,
                    other.name
                );
            }
        }
    }

    /// Once the walking is done the hint has to stop sending the player out
    /// and start naming the bench. This is not an edge case: a player who has
    /// been round the valley has met most of the shelf, so for most of the game
    /// this is the branch they read, and pointing them at ground they have
    /// already worked would be the old useless count in a longer sentence.
    #[test]
    fn a_formula_whose_reagents_are_all_met_names_the_bench_instead() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        // Forget the starter formulae, so the healing draught is unlogged and
        // this measures the hint rather than the seeding.
        state.progression.known_recipes.clear();

        let away = state
            .undiscovered_formula_hint(&data, "whisper_moss")
            .expect("whisper moss feeds unlogged formulae");

        for item in &data.items {
            state.inventory.insert(item.id.clone(), 1);
        }
        let home = state
            .undiscovered_formula_hint(&data, "whisper_moss")
            .expect("whisper moss still feeds unlogged formulae");

        assert_ne!(away, home, "the hint ignored what the player has met");
        // Found in the data rather than spelled out: the bench named has to be
        // one that really brews an unlogged whisper moss formula, so renaming a
        // station or moving a recipe cannot turn this into a test of nothing.
        let benches = data
            .recipes
            .iter()
            .filter(|recipe| {
                recipe
                    .ingredients
                    .iter()
                    .any(|ingredient| ingredient.item_id == "whisper_moss")
            })
            .filter_map(|recipe| {
                data.stations
                    .iter()
                    .find(|station| station.id == recipe.station_id)
            })
            .map(|station| station.name.as_str())
            .collect::<Vec<_>>();
        assert!(
            benches.iter().any(|bench| home.contains(bench)),
            "everything met should point at a bench that brews it: {home}"
        );
    }
}
