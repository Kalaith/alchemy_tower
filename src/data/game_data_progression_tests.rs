//! Playability checks over the authored content: that a new game can be
//! finished, that every quest spec can actually be brewed, and that nothing
//! waits on a flag the game never sets.
//!
//! Split out of `game_data.rs` when its test module alone passed 700 lines.

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;
    use crate::data::GameData;

    use crate::data::game_data_narrative_tests::tests::recordable_milestone_ids;

    /// The archive reconstruction is the game's ending. A requirement pointing at
    /// a quest or a journal beat that nothing can produce does not make the
    /// ending hard, it makes it impossible, and nothing else would catch it.
    #[test]
    fn the_ending_can_still_be_reached() {
        let data = load_embedded().expect("embedded game data should load");
        let recordable = recordable_milestone_ids(&data);
        let mut blocked = Vec::new();

        for quest_id in &data.config.archive_required_completed_quests {
            if data.quest(quest_id).is_none() {
                blocked.push(format!(
                    "ending needs quest {quest_id}, which does not exist"
                ));
            }
        }
        for milestone_id in &data.config.archive_required_journal_milestones {
            if !recordable.contains(milestone_id) {
                blocked.push(format!(
                    "ending needs milestone {milestone_id}, which nothing records"
                ));
            }
        }

        assert!(
            blocked.is_empty(),
            "the ending is unreachable:\n{blocked:#?}"
        );
    }

    /// A brew keeps about two traits, chosen by the recipe's guaranteed and
    /// preferred lists and by what its reagents have most of. A request naming
    /// traits the delivered bottle can never carry looks perfectly reasonable in
    /// the data and can never be handed in, so ask the real rule rather than
    /// guessing at it.
    #[test]
    fn every_quest_spec_can_actually_be_brewed() {
        let data = load_embedded().expect("embedded game data should load");
        let mut impossible = Vec::new();

        for quest in &data.quests {
            let required_traits = quest
                .required_traits
                .iter()
                .chain((!quest.required_trait.is_empty()).then_some(&quest.required_trait))
                .collect::<std::collections::BTreeSet<_>>();
            let required_effects = quest
                .required_effect_kinds
                .iter()
                .chain(
                    (!quest.required_effect_kind.is_empty()).then_some(&quest.required_effect_kind),
                )
                .collect::<std::collections::BTreeSet<_>>();
            if required_traits.is_empty() && required_effects.is_empty() {
                continue;
            }

            let trait_target = if quest.minimum_trait_matches == 0 {
                required_traits.len()
            } else {
                (quest.minimum_trait_matches as usize).min(required_traits.len())
            };
            let effect_target = if quest.minimum_effect_matches == 0 {
                required_effects.len()
            } else {
                (quest.minimum_effect_matches as usize).min(required_effects.len())
            };

            if let Some(item) = data.item(&quest.required_item_id) {
                let carried = item
                    .effects
                    .iter()
                    .map(|effect| effect.kind.as_str())
                    .collect::<std::collections::BTreeSet<_>>();
                let hits = required_effects
                    .iter()
                    .filter(|kind| carried.contains(kind.as_str()))
                    .count();
                if hits < effect_target {
                    impossible.push(format!(
                        "{}: {} only does {:?}, short of {:?}",
                        quest.id, quest.required_item_id, carried, required_effects
                    ));
                }
            }

            if trait_target == 0 {
                continue;
            }
            let reachable = reachable_traits(&data, &quest.required_item_id);
            let hits = required_traits
                .iter()
                .filter(|wanted| reachable.contains(wanted.as_str()))
                .count();
            if hits < trait_target {
                impossible.push(format!(
                    "{}: {} can carry {:?}, so it cannot meet {} of {:?}",
                    quest.id, quest.required_item_id, reachable, trait_target, required_traits
                ));
            }
        }

        assert!(
            impossible.is_empty(),
            "quest specs no brew can satisfy:\n{impossible:#?}"
        );
    }

    /// What a plain brew of this item ends up carrying, taken from the real
    /// inheritance rule. A catalyst can add more, so this is the honest floor.
    fn reachable_traits(data: &GameData, item_id: &str) -> std::collections::BTreeSet<String> {
        let mut reachable = std::collections::BTreeSet::new();
        for recipe in &data.recipes {
            let produces = recipe.output_item_id == item_id
                || recipe
                    .morph_targets
                    .iter()
                    .any(|morph| morph.output_item_id == item_id);
            if !produces {
                continue;
            }
            let ingredients = recipe
                .ingredients
                .iter()
                .filter_map(|ingredient| data.item(&ingredient.item_id))
                .collect::<Vec<_>>();
            reachable.extend(crate::alchemy::inherited_traits(recipe, &ingredients, None));
        }
        reachable
    }

    /// Coins stopped being a decision once the content grew: quests pay out
    /// thousands, the four floor gates cost 250 between them, and for a long
    /// while the dearest thing any counter sold was 38. This is a floor on
    /// ambition, not a balance model — it fails if nothing in the game is worth
    /// deliberately saving for.
    #[test]
    fn there_is_something_worth_saving_for() {
        const WORTH_SAVING_FOR: u32 = 150;

        let data = load_embedded().expect("embedded game data should load");
        let dearest = data
            .stations
            .iter()
            .flat_map(|station| station.stock.iter())
            .map(|stocked| stocked.price)
            .max()
            .unwrap_or(0);
        let one_off_income = data
            .quests
            .iter()
            .filter(|quest| !quest.repeatable)
            .map(|quest| quest.reward_coins)
            .sum::<u32>();

        assert!(
            dearest >= WORTH_SAVING_FOR,
            "the dearest purchasable item costs {dearest} against {one_off_income} coins of              one-off quest income; nothing in the shops is worth saving for"
        );
    }

    /// Areas a new game can walk to before satisfying any gate at all.
    fn areas_open_at_the_start(data: &GameData) -> std::collections::HashSet<String> {
        let mut open = std::collections::HashSet::new();
        let mut frontier = vec![data.config.starting_area.clone()];
        while let Some(area_id) = frontier.pop() {
            if !open.insert(area_id.clone()) {
                continue;
            }
            let Some(area) = data.area(&area_id) else {
                continue;
            };
            for warp in &area.warps {
                let ungated = warp.required_total_brews == 0
                    && warp.required_coins == 0
                    && warp.required_item_id.is_empty()
                    && warp.required_mastered_recipe.is_empty()
                    && warp.required_journal_milestone.is_empty();
                if ungated {
                    frontier.push(warp.target_area.clone());
                }
            }
        }
        open
    }

    /// The first request a player can accept must be completable with what a
    /// new game can actually reach. Twenty-three passes of content moved the
    /// world around underneath the opening — the tower entry's only gatherables
    /// ended up behind a late-game quest — and nothing would have said so.
    #[test]
    fn the_opening_can_be_completed_from_a_new_game() {
        let data = load_embedded().expect("embedded game data should load");
        let open_areas = areas_open_at_the_start(&data);

        let mut available = std::collections::HashSet::new();
        for area_id in &open_areas {
            let Some(area) = data.area(area_id) else {
                continue;
            };
            for node in &area.gather_nodes {
                if node.required_completed_quest.is_empty() {
                    available.insert(node.item_id.clone());
                }
            }
            for station in data.stations.iter().filter(|s| &s.area_id == area_id) {
                if station.required_completed_quest.is_empty()
                    && station.required_journal_milestone.is_empty()
                {
                    available.extend(station.stock.iter().map(|s| s.item_id.clone()));
                }
            }
        }

        let opening_quests = data
            .quests
            .iter()
            .filter(|quest| {
                quest.prerequisite_quests.is_empty()
                    && quest.required_unlocked_warp.is_empty()
                    && quest.minimum_total_brews == 0
                    && !quest.required_item_id.is_empty()
            })
            .collect::<Vec<_>>();
        assert!(
            !opening_quests.is_empty(),
            "no quest is available at the very start"
        );

        let mut blocked = Vec::new();
        for quest in opening_quests {
            let brewable = data.recipes.iter().any(|recipe| {
                recipe.output_item_id == quest.required_item_id
                    && recipe
                        .ingredients
                        .iter()
                        .all(|ingredient| available.contains(&ingredient.item_id))
                    && data
                        .stations
                        .iter()
                        .any(|s| s.id == recipe.station_id && open_areas.contains(&s.area_id))
            });
            if !brewable {
                blocked.push(format!(
                    "{} wants {}, which cannot be brewed from what a new game can reach",
                    quest.id, quest.required_item_id
                ));
            }
        }

        assert!(
            blocked.is_empty(),
            "the opening is not completable:
{blocked:#?}
reachable areas: {open_areas:?}"
        );
    }

    /// Delivering a repeatable request puts it on a cooldown instead of into
    /// `completed_quests` — that is the whole point of repeatable. So anything
    /// listing one as a prerequisite waits on a flag that is never set, and is
    /// locked forever. Thirteen repeatable requests exist and nothing else would
    /// notice a fourteenth being pointed at.
    #[test]
    fn nothing_waits_on_a_repeatable_quest() {
        let data = load_embedded().expect("embedded game data should load");
        let unreachable = data
            .quests
            .iter()
            .flat_map(|quest| {
                quest
                    .prerequisite_quests
                    .iter()
                    .map(move |prerequisite| (quest, prerequisite))
            })
            .filter(|(_, prerequisite)| {
                data.quest(prerequisite)
                    .map(|earlier| earlier.repeatable)
                    .unwrap_or(false)
            })
            .map(|(quest, prerequisite)| {
                format!("{} waits on {prerequisite}, which repeats", quest.id)
            })
            .collect::<Vec<_>>();

        assert!(
            unreachable.is_empty(),
            "quests gated behind a request that never completes:
{unreachable:#?}"
        );
    }

    /// A room bonus is the game's answer to "where should this be brewed", and
    /// it can only answer for traits some bench actually favours. Heat was the
    /// hole: `warm`, `volatile` and `vigor` ran through eight recipes and no
    /// bench rewarded any of them, so a third of the trait vocabulary had no
    /// room worth walking to. Rune traits are excluded — runes rework finished
    /// potions and are never brew reagents.
    #[test]
    fn quest_chains_and_gates_resolve() {
        let data = load_embedded().expect("embedded game data should load");
        let mut missing = Vec::new();

        for npc in &data.npcs {
            for quest_id in npc.quest_chain() {
                match data.quest(quest_id) {
                    None => missing.push(format!("{} -> quest {}", npc.id, quest_id)),
                    Some(quest) if quest.giver_npc_id != npc.id => missing.push(format!(
                        "{} carries {} but it is given by {}",
                        npc.id, quest.id, quest.giver_npc_id
                    )),
                    Some(_) => {}
                }
            }
        }

        for quest in &data.quests {
            for prerequisite in &quest.prerequisite_quests {
                if data.quest(prerequisite).is_none() {
                    missing.push(format!("{} -> prerequisite {}", quest.id, prerequisite));
                }
            }
        }

        for area in &data.areas {
            for node in &area.gather_nodes {
                if !node.required_completed_quest.is_empty()
                    && data.quest(&node.required_completed_quest).is_none()
                {
                    missing.push(format!(
                        "{} -> gate quest {}",
                        node.id, node.required_completed_quest
                    ));
                }
            }
        }

        for recipe in &data.rune_recipes {
            if !data.stations.iter().any(|s| s.id == recipe.station_id) {
                missing.push(format!("{} -> station {}", recipe.id, recipe.station_id));
            }
            for (label, item_id) in [
                ("input", &recipe.input_item_id),
                ("rune", &recipe.rune_item_id),
                ("output", &recipe.output_item_id),
            ] {
                match data.item(item_id) {
                    None => missing.push(format!("{} -> {label} {item_id}", recipe.id)),
                    // Handing the bench something that is not a rune would leave
                    // a pattern that can be listed but never assembled.
                    Some(item)
                        if label == "rune" && item.category != crate::data::ItemCategory::Rune =>
                    {
                        missing.push(format!("{} -> {item_id} is not a rune", recipe.id))
                    }
                    Some(_) => {}
                }
            }
        }

        for station in &data.stations {
            if !station.required_completed_quest.is_empty()
                && data.quest(&station.required_completed_quest).is_none()
            {
                missing.push(format!(
                    "{} -> gate quest {}",
                    station.id, station.required_completed_quest
                ));
            }
            for creature_id in &station.habitat_creature_ids {
                if data.item(creature_id).is_none() {
                    missing.push(format!("{} -> creature {}", station.id, creature_id));
                }
            }
            if !station.habitat_output_item_id.is_empty()
                && data.item(&station.habitat_output_item_id).is_none()
            {
                missing.push(format!(
                    "{} -> harvest {}",
                    station.id, station.habitat_output_item_id
                ));
            }
            for stocked in &station.stock {
                if data.item(&stocked.item_id).is_none() {
                    missing.push(format!("{} -> stock {}", station.id, stocked.item_id));
                }
            }
        }

        assert!(
            missing.is_empty(),
            "unresolved quest references:\n{missing:#?}"
        );
    }

    /// Quest gates on stations and gather nodes have been checked since the
    /// first arc landed. Journal-milestone gates never have — and four things
    /// already use one: the reading bench, the channel forge, the astral lens,
    /// and the warp into the archive itself. A milestone id that nothing
    /// records does not fail to load and does not read as broken; the bench is
    /// simply never available, in a game where a bench can be a whole floor's
    /// reason to exist.
    ///
    /// The cloud frame is the fifth, and it is gated on a *recipe discovery*
    /// beat rather than a quest, which is a source this file's milestone set
    /// had not been taught about at all.
    #[test]
    fn every_milestone_gate_points_at_something_that_happens() {
        let data = load_embedded().expect("embedded game data should load");
        let recordable = recordable_milestone_ids(&data);

        let mut gates = 0usize;
        let mut dangling = Vec::new();
        let mut check = |what: String, milestone: &str, gates: &mut usize| {
            if milestone.is_empty() {
                return;
            }
            *gates += 1;
            if !recordable.contains(milestone) {
                dangling.push(format!(
                    "{what} waits on {milestone}, which nothing records"
                ));
            }
        };

        for station in &data.stations {
            check(
                format!("station {}", station.id),
                &station.required_journal_milestone,
                &mut gates,
            );
        }
        for area in &data.areas {
            for warp in &area.warps {
                check(
                    format!("warp {}", warp.id),
                    &warp.required_journal_milestone,
                    &mut gates,
                );
            }
        }

        assert!(gates > 0, "nothing in the game is gated on a journal beat");
        assert!(
            dangling.is_empty(),
            "gates that can never open:\n{dangling:#?}"
        );
    }

    /// Imbuing spends a rune and a finished bottle, and three of the four runes
    /// cost coin while the fourth can only be prised off ward frames that have
    /// already let go. A pattern whose output is worth less than what went into
    /// it is therefore a trap, and the drafts list advertises every pattern
    /// equally — the player has no way to tell before spending the rune.
    ///
    /// All seventeen already honour this, including the four salvage reworks,
    /// which take a spoiled brew and are the largest upgrades in the layer.
    /// Base value is the honest proxy here: it is what the game itself uses to
    /// price a bottle.
    #[test]
    fn imbuing_is_never_a_downgrade() {
        let data = load_embedded().expect("embedded game data should load");
        let mut traps = Vec::new();

        for recipe in &data.rune_recipes {
            let (Some(input), Some(output)) = (
                data.item(&recipe.input_item_id),
                data.item(&recipe.output_item_id),
            ) else {
                continue; // resolution is `quest_chains_and_gates_resolve`'s job
            };
            if output.base_value <= input.base_value {
                traps.push(format!(
                    "{}: {} at {}c becomes {} at {}c",
                    recipe.id, input.id, input.base_value, output.id, output.base_value
                ));
            }
        }

        assert!(
            !data.rune_recipes.is_empty(),
            "no rune patterns loaded at all"
        );
        assert!(
            traps.is_empty(),
            "rune patterns that cost more than they give:\n{traps:#?}"
        );
    }

    /// A habitat only does anything once a creature is put in it, so a habitat
    /// whose creature exists nowhere in the valley is furniture: it can be
    /// built, gated, drawn and walked past, and never stocked. That has already
    /// shipped once — the bloomwing habitat went in a whole pass before there
    /// was any way to meet a bloomwing — and it was found by hand rather than
    /// by the suite, which is the argument for spending a test on it.
    ///
    /// Buying counts. The valley's counters sell things nobody will say where
    /// they got, and a habitat stocked from a shop is stocked.
    #[test]
    fn every_habitat_creature_can_be_met() {
        let data = load_embedded().expect("embedded game data should load");
        let mut sources = std::collections::HashSet::new();
        for area in &data.areas {
            for node in &area.gather_nodes {
                sources.insert(node.item_id.as_str());
            }
        }
        for station in &data.stations {
            for stocked in &station.stock {
                sources.insert(stocked.item_id.as_str());
            }
        }

        let mut housed = 0usize;
        let mut orphans = Vec::new();
        for station in &data.stations {
            for creature_id in &station.habitat_creature_ids {
                housed += 1;
                if !sources.contains(creature_id.as_str()) {
                    orphans.push(format!(
                        "{} houses {creature_id}, which cannot be gathered or bought anywhere",
                        station.id
                    ));
                }
            }
        }

        // Zero habitats would pass the loop above without asserting anything.
        assert!(housed > 0, "no habitat houses anything at all");
        assert!(
            orphans.is_empty(),
            "habitats waiting on creatures that cannot be met:\n{orphans:#?}"
        );
    }
}
