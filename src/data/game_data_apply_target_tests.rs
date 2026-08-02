//! Checks over the applied-alchemy layer: that everything a brew can be poured
//! on could actually be treated, and that treating things opens something.
//!
//! Split out of `game_data_progression_tests.rs`, which was at 837 lines when
//! apply targets landed. Those checks ask whether the game can be *finished*;
//! these ask whether its stated premise is on the critical path.

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;

    /// A target asks for a kind of brew and, sometimes, a grade. Both are
    /// strings in a content file, so both can be wrong in ways nothing else
    /// would catch: an effect kind no potion produces makes a target that can
    /// never be treated, and since gates now wait on targets, that is a wall
    /// with no door rather than a missed flourish.
    #[test]
    fn everything_a_brew_can_be_poured_on_can_actually_be_treated() {
        let data = load_embedded().expect("embedded game data should load");
        let mut targets = 0usize;
        let mut impossible = Vec::new();

        for area in &data.areas {
            for target in &area.apply_targets {
                targets += 1;
                let candidates = data
                    .items
                    .iter()
                    .filter(|item| item.category == crate::data::ItemCategory::Potion)
                    .filter(|item| {
                        item.effects
                            .iter()
                            .any(|effect| effect.kind.to_string() == target.required_effect_kind)
                    })
                    .collect::<Vec<_>>();
                if candidates.is_empty() {
                    impossible.push(format!(
                        "{} wants a {} brew, and nothing in the game does that",
                        target.id, target.required_effect_kind
                    ));
                    continue;
                }
                // Something has to be able to reach the grade it asks for.
                if !target.minimum_quality_band.is_empty()
                    && !candidates
                        .iter()
                        .any(|item| data.recipes.iter().any(|r| r.output_item_id == item.id))
                {
                    impossible.push(format!(
                        "{} wants {} of a {} brew, and no recipe makes one",
                        target.id, target.minimum_quality_band, target.required_effect_kind
                    ));
                }
            }
        }

        assert!(
            targets > 0,
            "nothing in the world can have a brew used on it"
        );
        assert!(
            impossible.is_empty(),
            "targets no brew could ever treat:
{impossible:#?}"
        );
    }

    /// The premise on the critical path. Applying a brew to the world was
    /// supposed to open things, not merely decorate them — if no gate anywhere
    /// waits on a treated target, the whole mechanic is optional scenery.
    #[test]
    fn something_in_the_world_opens_only_by_treating_it() {
        let data = load_embedded().expect("embedded game data should load");
        let target_beats = data
            .areas
            .iter()
            .flat_map(|area| area.apply_targets.iter())
            .flat_map(|target| target.completion_milestones.iter())
            .map(|milestone| milestone.id.clone())
            .collect::<std::collections::HashSet<_>>();

        let gated = data
            .areas
            .iter()
            .flat_map(|area| area.warps.iter())
            .any(|warp| target_beats.contains(&warp.required_journal_milestone))
            || data
                .stations
                .iter()
                .any(|station| target_beats.contains(&station.required_journal_milestone));

        assert!(
            gated,
            "no route or facility waits on a brew being poured on anything"
        );
    }

    /// The ending was a wall. Every one of the game's requests, nodes, warps and
    /// flourishes was reachable *before* the observatory, so the moment a player
    /// finished the thing the whole game builds towards, the valley had nine
    /// sentences of last words and then never changed again — in a game whose
    /// own scope note says a finished product is twenty to twenty-five hours.
    ///
    /// The ending beat is read off the narrative spine rather than spelled here,
    /// so renaming it cannot quietly turn this into a test of nothing.
    #[test]
    fn something_in_the_game_happens_after_the_ending() {
        let data = load_embedded().expect("embedded game data should load");
        let ending = crate::content::narrative_text()
            .milestones
            .observatory_ending
            .id
            .clone();

        let requests = data
            .quests
            .iter()
            .filter(|quest| quest.required_journal_milestone == ending)
            .count();
        let commissions = data
            .quests
            .iter()
            .filter(|quest| quest.required_journal_milestone == ending && quest.coin_cost > 0)
            .count();

        assert!(
            requests >= 3,
            "only {requests} request(s) wait on the ending; the valley stops the day it is finished"
        );
        assert!(
            commissions >= 1,
            "nothing after the ending costs anything, so a finished campaign's coins have nowhere left to go"
        );
    }

    /// A flourish waits on a quest or a beat, both of which are strings in an
    /// area file. One that names something that does not exist is a piece of
    /// the world that never appears, and nothing on screen would say why.
    #[test]
    fn every_flourish_waits_on_something_real() {
        use crate::data::game_data_narrative_tests::tests::recordable_milestone_ids;

        let data = load_embedded().expect("embedded game data should load");
        let recordable = recordable_milestone_ids(&data);
        let mut flourishes = 0usize;
        let mut dangling = Vec::new();

        for area in &data.areas {
            for flourish in &area.flourishes {
                flourishes += 1;
                for quest_id in &flourish.after_any_completed_quest {
                    if data.quest(quest_id).is_none() {
                        dangling.push(format!("{} waits on quest {quest_id}", flourish.id));
                    }
                }
                for milestone_id in &flourish.after_any_journal_milestone {
                    if !recordable.contains(milestone_id) {
                        dangling.push(format!("{} waits on beat {milestone_id}", flourish.id));
                    }
                }
                if flourish.shapes.is_empty() {
                    dangling.push(format!("{} draws nothing at all", flourish.id));
                }
            }
        }

        assert!(flourishes > 0, "the world never changes for anything");
        assert!(
            dangling.is_empty(),
            "flourishes waiting on things that never happen:
{dangling:#?}"
        );
    }

    /// The point of moving these into data was coverage. Twelve story chains
    /// finish in this game and the world used to acknowledge four of them,
    /// across two areas, because each one was a `match` arm somebody had to
    /// write. This is a floor, not a target.
    #[test]
    fn the_world_changes_in_more_than_a_couple_of_places() {
        let data = load_embedded().expect("embedded game data should load");
        let areas = data
            .areas
            .iter()
            .filter(|area| !area.flourishes.is_empty())
            .count();
        let total = data
            .areas
            .iter()
            .map(|area| area.flourishes.len())
            .sum::<usize>();

        assert!(
            areas >= 3 && total >= 6,
            "only {total} flourishes across {areas} areas; the world barely notices what you do"
        );
    }

    /// The floor above counts flourishes and is satisfied by putting them all in
    /// one room, which is exactly what happened: nine of the first fourteen were
    /// in the town square, and the tower — the building this game is *about*
    /// reopening — changed in two of its six rooms. The entry lab, where the
    /// player starts every day and brews for the first several hours, changed
    /// for nothing at all.
    ///
    /// A room the player works in is derived from the stations, not listed here,
    /// so a new bench on a new floor is covered the day it is placed.
    #[test]
    fn every_room_the_player_works_in_changes_for_something() {
        let data = load_embedded().expect("embedded game data should load");

        let working_rooms = data
            .stations
            .iter()
            .map(|station| station.area_id.clone())
            .collect::<std::collections::BTreeSet<_>>();

        let mut unchanging = working_rooms
            .iter()
            .filter(|area_id| {
                data.areas
                    .iter()
                    .find(|area| &&area.id == area_id)
                    .is_none_or(|area| area.flourishes.is_empty())
            })
            .cloned()
            .collect::<Vec<_>>();

        unchanging.sort();
        assert!(
            unchanging.is_empty(),
            "rooms the player works in that never change: {unchanging:?}"
        );
    }

    /// Every beat a milestone-writer records should lead somewhere. Treating a
    /// thing, or funding a commission, is expensive — bottles at a grade, and
    /// for a commission thousands of coins — and a payoff that exists only as a
    /// journal entry reads as a receipt rather than a change.
    ///
    /// "Somewhere" is deliberately broad: a route, a facility, ground that
    /// starts growing, or a visible change in the world all count. What does
    /// not count is nothing.
    #[test]
    fn treating_and_funding_things_leads_somewhere() {
        let data = load_embedded().expect("embedded game data should load");

        let mut consumed = std::collections::HashSet::new();
        for area in &data.areas {
            for warp in &area.warps {
                consumed.insert(warp.required_journal_milestone.clone());
            }
            for node in &area.gather_nodes {
                consumed.insert(node.required_journal_milestone.clone());
            }
            for flourish in &area.flourishes {
                consumed.extend(flourish.after_any_journal_milestone.iter().cloned());
            }
        }
        for station in &data.stations {
            consumed.insert(station.required_journal_milestone.clone());
        }

        let mut orphaned = Vec::new();

        for area in &data.areas {
            for target in &area.apply_targets {
                if !target
                    .completion_milestones
                    .iter()
                    .any(|milestone| consumed.contains(&milestone.id))
                {
                    orphaned.push(format!("treating {} opens nothing", target.id));
                }
            }
        }
        for quest in data.quests.iter().filter(|quest| quest.coin_cost > 0) {
            if !quest
                .completion_milestones
                .iter()
                .any(|milestone| consumed.contains(&milestone.id))
            {
                orphaned.push(format!("funding {} changes nothing", quest.id));
            }
        }

        orphaned.sort();
        assert!(
            orphaned.is_empty(),
            "work whose only payoff is a journal entry:
{orphaned:#?}"
        );
    }

    /// Second-order brewing: a bench that takes finished bottles as reagents.
    /// It is the only structural sink the deep benches' outputs have — nothing
    /// asks for a benchlight solution, so the way it stops being vendor trash
    /// is for something else to need one.
    ///
    /// The rule this pins is the one a content author will trip over: a recipe
    /// may only call for a potion at a bench that accepts potions. The entry
    /// cauldron does not, and a recipe there naming a bottle would be
    /// unfillable with nothing on screen to explain it.
    #[test]
    fn a_recipe_only_asks_for_a_bottle_at_a_bench_that_takes_bottles() {
        use crate::data::ItemCategory;

        let data = load_embedded().expect("embedded game data should load");
        let mut second_order = 0usize;
        let mut misplaced = Vec::new();

        for recipe in &data.recipes {
            let bottles = recipe
                .ingredients
                .iter()
                .filter(|ingredient| {
                    data.item(&ingredient.item_id)
                        .is_some_and(|item| item.category == ItemCategory::Potion)
                })
                .count();
            if bottles == 0 {
                continue;
            }
            second_order += 1;
            let takes_bottles = data
                .stations
                .iter()
                .find(|station| station.id == recipe.station_id)
                .is_some_and(|station| station.accepts_potions);
            if !takes_bottles {
                misplaced.push(format!(
                    "{} wants {bottles} finished bottle(s) at {}, which will not take one",
                    recipe.id, recipe.station_id
                ));
            }
        }

        assert!(
            second_order > 0,
            "no recipe consumes a finished bottle; the deep benches still make vendor trash"
        );
        assert!(
            misplaced.is_empty(),
            "recipes asking for bottles at a bench that refuses them:
{misplaced:#?}"
        );
    }

    /// The tier was built to give the deep benches' outputs a destination, so
    /// it must not become the new worst offender. A compound bottle costs two
    /// finished brews and a reagent to make; if nothing then asks for it, the
    /// whole chain terminates in vendor trash one layer further up than before.
    ///
    /// "Asks for it" is deliberately broad — a request, a repeatable order, a
    /// commission, a rune pattern, or another recipe using it as a reagent. A
    /// morph target does not count: that is another way to *make* the thing,
    /// not a reason to have one.
    #[test]
    fn the_late_tier_does_not_make_its_own_vendor_trash() {
        use crate::data::ItemCategory;

        let data = load_embedded().expect("embedded game data should load");

        let mut wanted = std::collections::HashSet::new();
        for quest in &data.quests {
            wanted.insert(quest.required_item_id.clone());
        }
        for recipe in &data.recipes {
            for ingredient in &recipe.ingredients {
                wanted.insert(ingredient.item_id.clone());
            }
        }
        for rune in &data.rune_recipes {
            wanted.insert(rune.input_item_id.clone());
        }

        let mut unwanted = Vec::new();
        for recipe in &data.recipes {
            let second_order = recipe.ingredients.iter().any(|ingredient| {
                data.item(&ingredient.item_id)
                    .is_some_and(|item| item.category == ItemCategory::Potion)
            });
            if second_order && !wanted.contains(&recipe.output_item_id) {
                unwanted.push(format!(
                    "{} makes {}, which nothing asks for",
                    recipe.id, recipe.output_item_id
                ));
            }
        }

        unwanted.sort();
        assert!(
            unwanted.is_empty(),
            "compound bottles with nowhere to go:
{unwanted:#?}"
        );
    }

    /// A morph branch is the hardest thing the brewing system asks for: the
    /// quality bar, the exact heat and stir count, the timing word, sometimes a
    /// named catalyst, a reagent order and the room bonus, all at once. Thirteen
    /// of the twenty-nine bottles only a branch can make were wanted by nothing
    /// — so the reward for the deepest verb in the tower was a thing to sell.
    ///
    /// "Wanted" is the same broad definition the compound tier uses: a request,
    /// a repeatable order, a commission, a rune pattern, or a reagent slot. A
    /// second morph reaching the same bottle deliberately does not count; that
    /// is another way to make it, not a reason to have one.
    #[test]
    fn a_morph_branch_pays_out_in_something_somebody_wants() {
        let data = load_embedded().expect("embedded game data should load");

        let mut wanted = std::collections::HashSet::new();
        for quest in &data.quests {
            wanted.insert(quest.required_item_id.clone());
        }
        for recipe in &data.recipes {
            for ingredient in &recipe.ingredients {
                wanted.insert(ingredient.item_id.clone());
            }
        }
        for rune in &data.rune_recipes {
            wanted.insert(rune.input_item_id.clone());
        }

        // A bottle an ordinary recipe also makes is somebody else's problem —
        // this guard is about what the *branch* is worth reaching for.
        let plainly_brewable = data
            .recipes
            .iter()
            .map(|recipe| recipe.output_item_id.clone())
            .collect::<std::collections::HashSet<_>>();

        let mut unwanted = data
            .recipes
            .iter()
            .flat_map(|recipe| {
                recipe
                    .morph_targets
                    .iter()
                    .map(move |morph| (recipe, &morph.output_item_id))
            })
            .filter(|(_, output)| !wanted.contains(*output) && !plainly_brewable.contains(*output))
            .map(|(recipe, output)| format!("{} branches into {output}, unasked for", recipe.id))
            .collect::<Vec<_>>();

        unwanted.sort();
        unwanted.dedup();
        assert!(
            unwanted.is_empty(),
            "precision brewing that pays out in vendor trash:
{unwanted:#?}"
        );
    }

    /// The late tier exists to deepen decisions, not to lengthen a list. A
    /// second-order recipe should use the parts of the lattice the mid-game
    /// barely touches: three reagents, a full sequence, and a branch.
    #[test]
    fn the_late_tier_is_deeper_than_the_middle_of_the_game() {
        use crate::data::ItemCategory;

        let data = load_embedded().expect("embedded game data should load");
        let mut shallow = Vec::new();

        for recipe in &data.recipes {
            let second_order = recipe.ingredients.iter().any(|ingredient| {
                data.item(&ingredient.item_id)
                    .is_some_and(|item| item.category == ItemCategory::Potion)
            });
            if !second_order {
                continue;
            }
            if recipe.ingredients.len() < 3
                || recipe.required_sequence.len() < 3
                || recipe.morph_targets.len() < 2
            {
                shallow.push(format!(
                    "{}: {} reagents, {}-step sequence, {} branches",
                    recipe.id,
                    recipe.ingredients.len(),
                    recipe.required_sequence.len(),
                    recipe.morph_targets.len()
                ));
            }
        }

        assert!(
            shallow.is_empty(),
            "late-tier recipes that are flat variants in disguise:
{shallow:#?}"
        );
    }

    /// The apply-potion verb is the game's answer to "a brew is for pouring on
    /// something, not only for drinking or handing over", and it shipped with
    /// exactly the three examples the TODO listed — two restore, one misfire.
    /// A whole effect kind with nothing to pour it on means a player who brews
    /// lights has no reason to carry one anywhere but a dark node.
    ///
    /// Effect kinds come from the potions that exist, so this cannot go stale
    /// against a fifth kind being authored.
    #[test]
    fn every_effect_a_bottle_can_carry_has_something_to_pour_it_on() {
        use crate::data::ItemCategory;

        let data = load_embedded().expect("embedded game data should load");
        let poured = data
            .areas
            .iter()
            .flat_map(|area| area.apply_targets.iter())
            .map(|target| target.required_effect_kind.clone())
            .collect::<std::collections::HashSet<_>>();

        let unpourable = data
            .items
            .iter()
            .filter(|item| item.category == ItemCategory::Potion)
            .flat_map(|item| item.effects.iter())
            .map(|effect| effect.kind.as_str())
            .filter(|kind| !poured.contains(*kind))
            .collect::<std::collections::BTreeSet<_>>();

        assert!(
            unpourable.is_empty(),
            "effect kinds with nothing in the world to use them on: {unpourable:?}"
        );
    }
}
