//! Referential-integrity checks over the authored content: every id a piece of
//! content names must land on something real, and everything authored must be
//! reachable by some route the player actually has.
//!
//! Split out of `game_data.rs` when its test module alone passed 700 lines.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::data::load_embedded;
    use crate::data::GameData;

    #[test]
    fn recipe_ingredient_sets_are_unique_per_station() {
        let data = load_embedded().expect("embedded game data should load");
        let mut seen: HashMap<(String, Vec<(String, u32)>), String> = HashMap::new();

        for recipe in &data.recipes {
            let mut ingredients = recipe
                .ingredients
                .iter()
                .map(|ingredient| (ingredient.item_id.clone(), ingredient.amount))
                .collect::<Vec<_>>();
            ingredients.sort();
            let key = (recipe.station_id.clone(), ingredients);
            if let Some(existing) = seen.insert(key, recipe.id.clone()) {
                panic!(
                    "{} and {} share an ingredient set at station {}",
                    existing, recipe.id, recipe.station_id
                );
            }
        }
    }

    #[test]
    fn recipe_items_and_stations_resolve() {
        let data = load_embedded().expect("embedded game data should load");
        let mut missing = Vec::new();

        for recipe in &data.recipes {
            if !data.stations.iter().any(|s| s.id == recipe.station_id) {
                missing.push(format!("{} -> station {}", recipe.id, recipe.station_id));
            }
            let mut referenced = vec![
                recipe.output_item_id.clone(),
                recipe.unstable_output_item_id.clone(),
            ];
            referenced.extend(recipe.ingredients.iter().map(|i| i.item_id.clone()));
            referenced.extend(
                recipe
                    .morph_targets
                    .iter()
                    .map(|m| m.output_item_id.clone()),
            );
            for item_id in referenced.into_iter().filter(|id| !id.is_empty()) {
                if data.item(&item_id).is_none() {
                    missing.push(format!("{} -> item {}", recipe.id, item_id));
                }
            }
        }

        assert!(
            missing.is_empty(),
            "unresolved recipe references:\n{missing:#?}"
        );
    }

    #[test]
    fn gather_nodes_resolve_their_items_and_routes() {
        let data = load_embedded().expect("embedded game data should load");
        let mut missing = Vec::new();

        for area in &data.areas {
            for node in &area.gather_nodes {
                if data.item(&node.item_id).is_none() {
                    missing.push(format!("{} -> item {}", node.id, node.item_id));
                }
                if node.route_id.is_empty() {
                    continue;
                }
                let route = data.gathering_routes.iter().find(|r| r.id == node.route_id);
                match route {
                    None => missing.push(format!("{} -> route {}", node.id, node.route_id)),
                    Some(route) if route.area_id != area.id => missing.push(format!(
                        "{} uses route {} from area {}",
                        node.id, route.id, route.area_id
                    )),
                    Some(_) => {}
                }
            }
        }

        assert!(
            missing.is_empty(),
            "unresolved gather node references:\n{missing:#?}"
        );
    }

    /// A route with nothing on it is a name in the journal that leads to bare
    /// ground. Two of these survived several passes purely because the claim
    /// "every route has something on it" was being made from memory instead of
    /// counted.
    #[test]
    fn every_gathering_route_has_something_on_it() {
        let data = load_embedded().expect("embedded game data should load");
        let mut populated = std::collections::HashSet::new();
        for area in &data.areas {
            for node in &area.gather_nodes {
                populated.insert(node.route_id.clone());
            }
        }

        let bare = data
            .gathering_routes
            .iter()
            .filter(|route| !populated.contains(&route.id))
            .map(|route| format!("{} in {}", route.id, route.area_id))
            .collect::<Vec<_>>();

        assert!(bare.is_empty(), "routes with no gather nodes:\n{bare:#?}");
    }

    #[test]
    fn quests_reward_items_that_exist() {
        let data = load_embedded().expect("embedded game data should load");
        let missing = data
            .quests
            .iter()
            .filter(|quest| !quest.required_item_id.is_empty())
            .filter(|quest| data.item(&quest.required_item_id).is_none())
            .map(|quest| format!("{} -> item {}", quest.id, quest.required_item_id))
            .collect::<Vec<_>>();

        assert!(
            missing.is_empty(),
            "quests want unknown items:\n{missing:#?}"
        );
    }

    /// Every route by which an item can end up in the player's bag. A quest that
    /// asks for something outside this set is an errand that cannot be run.
    fn obtainable_item_ids(data: &GameData) -> std::collections::HashSet<String> {
        let mut obtainable = std::collections::HashSet::new();
        for area in &data.areas {
            obtainable.extend(area.gather_nodes.iter().map(|node| node.item_id.clone()));
        }
        for station in &data.stations {
            obtainable.extend(station.stock.iter().map(|stocked| stocked.item_id.clone()));
            if !station.habitat_output_item_id.is_empty() {
                obtainable.insert(station.habitat_output_item_id.clone());
            }
        }
        for recipe in &data.recipes {
            obtainable.insert(recipe.output_item_id.clone());
            obtainable.insert(recipe.unstable_output_item_id.clone());
            obtainable.extend(
                recipe
                    .morph_targets
                    .iter()
                    .map(|morph| morph.output_item_id.clone()),
            );
        }
        obtainable.extend(
            data.rune_recipes
                .iter()
                .map(|recipe| recipe.output_item_id.clone()),
        );
        // A townsperson handing you something is a way of getting it. Every
        // relationship gift used to double as shop stock or a gatherable, so
        // nothing noticed this was missing until one existed that a counter
        // will not sell.
        for npc in &data.npcs {
            for item_id in [&npc.friendship_reward_item_id, &npc.trusted_reward_item_id] {
                if !item_id.is_empty() {
                    obtainable.insert(item_id.clone());
                }
            }
        }
        // Salvage outputs are chosen in code when a mixture matches nothing, so
        // they never appear as any recipe's declared output.
        obtainable.extend(
            crate::alchemy::SALVAGE_OUTPUT_ITEM_IDS
                .iter()
                .map(|id| (*id).to_owned()),
        );
        obtainable
    }

    #[test]
    fn every_quest_asks_for_something_obtainable() {
        let data = load_embedded().expect("embedded game data should load");
        let obtainable = obtainable_item_ids(&data);

        let impossible = data
            .quests
            .iter()
            .filter(|quest| !quest.required_item_id.is_empty())
            .filter(|quest| !obtainable.contains(&quest.required_item_id))
            .map(|quest| format!("{} wants {}", quest.id, quest.required_item_id))
            .collect::<Vec<_>>();

        assert!(
            impossible.is_empty(),
            "quests asking for items nothing produces:\n{impossible:#?}"
        );
    }

    /// The same rule one step further out: an item nothing produces and nothing
    /// asks for is dead weight in the data.
    #[test]
    fn every_item_is_either_obtainable_or_an_ingredient_of_something() {
        let data = load_embedded().expect("embedded game data should load");
        let obtainable = obtainable_item_ids(&data);
        let mut used = std::collections::HashSet::new();
        for recipe in &data.recipes {
            used.extend(recipe.ingredients.iter().map(|i| i.item_id.clone()));
        }
        for recipe in &data.rune_recipes {
            used.insert(recipe.input_item_id.clone());
            used.insert(recipe.rune_item_id.clone());
        }
        for station in &data.stations {
            used.extend(station.planter_seed_ids.iter().cloned());
        }

        let stranded = data
            .items
            .iter()
            .filter(|item| !obtainable.contains(&item.id) && !used.contains(&item.id))
            .map(|item| item.id.clone())
            .collect::<Vec<_>>();

        assert!(
            stranded.is_empty(),
            "items nothing produces and nothing wants:\n{stranded:#?}"
        );
    }

    /// Every one of the game's recipes falls back to one of four salvage
    /// bottles when a brew destabilises, and free-form brewing produces them
    /// too — `soothing_tonic` alone catches twenty-one recipes' failures. They
    /// were worth 2 to 14 coins and **nothing in the world wanted any of them**,
    /// so the game's entire failure state paid out in items with no destination.
    ///
    /// A rune rework is the answer the game already had: it does not repair a
    /// spoiled brew, it decides what that brew's fault gets used for. This
    /// insists every salvage bottle keeps such a route.
    /// `quality_band_rank` matches the band name against the five UI strings
    /// and falls through to **0 — Crude — for anything it does not recognise**.
    /// So a request misspelling its band does not fail loudly; it silently
    /// becomes the easiest possible request, and a note demanding the finest
    /// work in the valley would be satisfied by the worst brew in the bag.
    ///
    /// Worth pinning now because this pass introduced `Masterwork`, a band no
    /// quest had ever asked for, and a typo in it would have been invisible.
    /// Not a quality bar — length does not make prose good, and this floor sits
    /// far below anything the game ships. It catches one specific failure: an
    /// item whose text nobody ever wrote properly.
    ///
    /// It checks the text the **journal will actually show**, which is not
    /// simply `item.description`. A `journal_herb_summary_<id>` or
    /// `journal_potion_recap_<id>` key in `ui_text.json` wins when present, and
    /// 23 items have one. So an item can carry a flat one-line description and
    /// read perfectly well in game, and — the direction that bites — a carefully
    /// written description can be shadowed by an override and never be seen.
    /// Measuring the field rather than the surface would report both wrongly.
    /// The same placeholder floor the items have, for the other prose the
    /// journal renders. Routes drifted the same way item descriptions did:
    /// the ones written early stayed at catalogue length while everything
    /// authored later ran three times longer, and two of them had gone stale
    /// as well — the observatory's route still described a room with nothing in
    /// it but a lens, six passes after it stopped being one.
    ///
    /// The floor is well below the shortest real description, so it only fires
    /// on a stub. The **ceiling** is the more interesting half: the pane draws
    /// the description with a wrapped block whose *start* is bounds-checked and
    /// whose height is not, so a long enough route runs down into the Tower
    /// Access panel underneath it. Measured off a capture — 197 characters
    /// wrapped to four lines in the 380px column, and there is room for about
    /// five before the collision. The ceiling is set at the longest description
    /// that already shipped and is known to render, not at the collision point.
    #[test]
    fn every_route_description_fits_the_pane_and_is_not_a_stub() {
        const PLACEHOLDER_FLOOR: usize = 100;
        const PANE_CEILING: usize = 215;

        let data = load_embedded().expect("embedded game data should load");
        let wrong = data
            .gathering_routes
            .iter()
            .filter_map(|route| {
                let len = route.description.chars().count();
                if len < PLACEHOLDER_FLOOR {
                    Some(format!("{}: {len} chars, a stub", route.id))
                } else if len > PANE_CEILING {
                    Some(format!("{}: {len} chars, overruns the pane", route.id))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert!(
            wrong.is_empty(),
            "route descriptions outside what the journal pane can show: {wrong:#?}"
        );
    }

    #[test]
    fn nothing_the_journal_shows_is_a_placeholder() {
        use crate::content::ui_copy_optional;
        use crate::data::ItemCategory;

        /// Well under the shortest real text, so this only fires on a stub.
        const PLACEHOLDER_FLOOR: usize = 60;

        let data = load_embedded().expect("embedded game data should load");
        let thin = data
            .items
            .iter()
            .filter_map(|item| {
                let key = if item.category == ItemCategory::Potion {
                    format!("journal_potion_recap_{}", item.id)
                } else {
                    format!("journal_herb_summary_{}", item.id)
                };
                let shown = ui_copy_optional(&key).unwrap_or(item.description.as_str());
                (shown.chars().count() < PLACEHOLDER_FLOOR)
                    .then(|| format!("{}: {} chars shown", item.id, shown.chars().count()))
            })
            .collect::<Vec<_>>();

        assert!(
            thin.is_empty(),
            "items the journal describes with a placeholder: {thin:#?}"
        );
    }

    #[test]
    fn every_quest_asks_for_a_quality_band_the_game_knows() {
        use crate::content::ui_copy;

        let known = [
            ui_copy("quality_band_crude"),
            ui_copy("quality_band_serviceable"),
            ui_copy("quality_band_fine"),
            ui_copy("quality_band_excellent"),
            ui_copy("quality_band_masterwork"),
        ];

        let data = load_embedded().expect("embedded game data should load");
        let unknown = data
            .quests
            .iter()
            .filter(|quest| !known.contains(&quest.minimum_quality_band.as_str()))
            .map(|quest| format!("{}: {:?}", quest.id, quest.minimum_quality_band))
            .collect::<Vec<_>>();

        assert!(
            unknown.is_empty(),
            "quests naming a band the game will silently read as Crude: {unknown:#?}"
        );
    }

    #[test]
    fn every_salvaged_brew_can_be_turned_into_something() {
        use crate::alchemy::SALVAGE_OUTPUT_ITEM_IDS;

        let data = load_embedded().expect("embedded game data should load");

        let reworkable = data
            .rune_recipes
            .iter()
            .map(|recipe| recipe.input_item_id.clone())
            .collect::<std::collections::HashSet<_>>();
        let wanted_by_quest = data
            .quests
            .iter()
            .map(|quest| quest.required_item_id.clone())
            .collect::<std::collections::HashSet<_>>();

        let dead_ends = SALVAGE_OUTPUT_ITEM_IDS
            .iter()
            .filter(|id| !reworkable.contains(**id) && !wanted_by_quest.contains(**id))
            .collect::<Vec<_>>();

        assert!(
            dead_ends.is_empty(),
            "failed brews that produce a bottle nothing in the valley wants: {dead_ends:?}"
        );
    }

    /// A room the player has to walk to should hold more than one thing. Two
    /// did not: the observatory, which the game *ends* in, and the archive,
    /// which holds the story's largest revelation — both a single node in a
    /// 960x720 floor. A room with one pickup in it is a corridor with a stop
    /// sign, and it reads as unfinished however good the writing on the walls.
    ///
    /// Two is a floor, not a target, and it is set at the leanest room that
    /// ships (the entry lab and the rune workshop, which are mostly benches and
    /// are meant to be).
    /// A warp's label is the sign on the door, and the area banner is the sign
    /// in the room. Nineteen of twenty-six matched exactly, so the intent was
    /// never in doubt — the other seven had simply drifted, and the same
    /// archive was reachable through a door marked "Archives" from one floor
    /// and "Tower Archives" from another. Nothing broke; the player was just
    /// told two different names for one room.
    /// A morph is the precision reward at the end of a recipe, and every one of
    /// them is gated on a catalyst tag. If the only thing carrying that tag
    /// comes from a single gather node, the whole branch inherits that node's
    /// weather: `kilnfire` gated **five** morphs behind one quarry node that
    /// appears twelve days in a hundred and cannot be bought.
    ///
    /// So each tag needs either a second source or a counter that sells it. The
    /// rule is deliberately about **routes, not rates** — counting days would
    /// mean picking a threshold, and a shop line is a real answer to scarcity
    /// even when the wild source stays rare.
    #[test]
    fn no_morph_branch_hangs_on_a_single_gather_node() {
        let data = load_embedded().expect("embedded game data should load");

        let wanted = data
            .recipes
            .iter()
            .flat_map(|recipe| recipe.morph_targets.iter())
            .filter(|morph| !morph.catalyst_tag.is_empty())
            .map(|morph| morph.catalyst_tag.clone())
            .collect::<std::collections::BTreeSet<_>>();

        let mut fragile = Vec::new();
        for tag in &wanted {
            let carriers = data
                .items
                .iter()
                .filter(|item| item.catalyst_tags.iter().any(|owned| owned == tag))
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>();
            let purchasable = data.stations.iter().any(|station| {
                station
                    .stock
                    .iter()
                    .any(|line| carriers.contains(&line.item_id.as_str()))
            });
            if carriers.len() < 2 && !purchasable {
                fragile.push(format!("{tag}: only {carriers:?}, and nowhere sells it"));
            }
        }

        assert!(
            fragile.is_empty(),
            "morph branches resting on one gather node: {fragile:#?}"
        );
    }

    #[test]
    fn every_door_is_signed_with_the_name_of_the_room_behind_it() {
        let data = load_embedded().expect("embedded game data should load");
        let names = data
            .areas
            .iter()
            .map(|area| (area.id.as_str(), area.name.as_str()))
            .collect::<std::collections::HashMap<_, _>>();

        let mismatched = data
            .areas
            .iter()
            .flat_map(|area| area.warps.iter().map(move |warp| (area, warp)))
            .filter_map(|(area, warp)| {
                let behind = names.get(warp.target_area.as_str())?;
                (warp.label != *behind).then(|| {
                    format!(
                        "{} in {}: labelled {:?}, room is {:?}",
                        warp.id, area.id, warp.label, behind
                    )
                })
            })
            .collect::<Vec<_>>();

        assert!(
            mismatched.is_empty(),
            "doors signed with a name the room does not use: {mismatched:#?}"
        );
    }

    #[test]
    fn no_room_is_worth_only_one_stop() {
        const MINIMUM_NODES: usize = 2;

        let data = load_embedded().expect("embedded game data should load");
        let sparse = data
            .areas
            .iter()
            .filter(|area| !area.gather_nodes.is_empty())
            .filter(|area| area.gather_nodes.len() < MINIMUM_NODES)
            .map(|area| format!("{}: {} node", area.id, area.gather_nodes.len()))
            .collect::<Vec<_>>();

        assert!(
            sparse.is_empty(),
            "rooms holding a single thing to find: {sparse:#?}"
        );
    }

    /// A plain ingredient earns its place by being wanted somewhere. Two herbs
    /// on the southern pass — `coldiron_lichen` and `rimeflower` — were authored
    /// with conditions, variants and prose, and then no recipe in the game
    /// asked for either, so the newest biome's whole harvest fed nothing. They
    /// were pickable, describable, and pointless.
    ///
    /// Creatures and catalysts are excluded deliberately: creatures feed
    /// habitats and catalysts are matched by `catalyst_tag` rather than by id,
    /// so neither shows up in a recipe's ingredient list even when in demand.
    #[test]
    fn every_gatherable_ingredient_is_wanted_by_some_recipe() {
        use crate::data::ItemCategory;

        let data = load_embedded().expect("embedded game data should load");
        let wanted = data
            .recipes
            .iter()
            .flat_map(|recipe| recipe.ingredients.iter().map(|entry| entry.item_id.clone()))
            .collect::<std::collections::HashSet<_>>();

        let idle = data
            .items
            .iter()
            .filter(|item| item.category == ItemCategory::Ingredient)
            .filter(|item| !wanted.contains(&item.id))
            .map(|item| item.id.clone())
            .collect::<std::collections::BTreeSet<_>>();

        assert!(
            idle.is_empty(),
            "ingredients that can be picked but that nothing brews with: {idle:?}"
        );
    }

    #[test]
    fn every_brewing_trait_has_a_bench_that_favours_it() {
        use crate::data::ItemCategory;

        let data = load_embedded().expect("embedded game data should load");
        let favoured = data
            .stations
            .iter()
            .filter(|station| station.kind == crate::data::StationKind::Alchemy)
            .flat_map(|station| station.room_bonus.favored_traits.iter().cloned())
            .collect::<std::collections::HashSet<_>>();

        let unrewarded = data
            .items
            .iter()
            .filter(|item| {
                matches!(
                    item.category,
                    ItemCategory::Ingredient | ItemCategory::Catalyst | ItemCategory::Creature
                )
            })
            .flat_map(|item| item.traits.iter().cloned())
            .filter(|item_trait| !favoured.contains(item_trait))
            .collect::<std::collections::BTreeSet<_>>();

        assert!(
            unrewarded.is_empty(),
            "traits no bench rewards, so nowhere is the right place to brew them: {unrewarded:?}"
        );
    }

    /// Imbuing has to leave a mark the rest of the game can read.
    ///
    /// The rune floor is the deepest verb in the tower and its outputs were
    /// indistinguishable, to every trait check in the game, from any other
    /// bottle: `spread`, `echo` and `delay` sat on the three runes and on
    /// nothing else, so a pattern was a name and a price and no property. Now
    /// each output carries the pattern its rune put into it, which is what lets
    /// a request ask for the echoed dose specifically and what a compound brew
    /// reads when one is folded in.
    ///
    /// The rule is per-rune rather than a hardcoded list, so a fifth rune is
    /// covered the day it is authored.
    #[test]
    fn an_imbued_bottle_carries_the_pattern_it_was_given() {
        let data = load_embedded().expect("embedded game data should load");
        let mut unmarked = Vec::new();

        for recipe in &data.rune_recipes {
            let (Some(rune), Some(output)) = (
                data.item(&recipe.rune_item_id),
                data.item(&recipe.output_item_id),
            ) else {
                continue; // resolution is `quest_chains_and_gates_resolve`'s job
            };
            // `arcane` is what every rune is; the other trait is what it does.
            let Some(pattern) = rune.traits.iter().find(|held| *held != "arcane") else {
                unmarked.push(format!("{} has no pattern trait at all", rune.id));
                continue;
            };
            if !output.traits.contains(pattern) {
                unmarked.push(format!(
                    "{}: {} leaves no {pattern} on {}",
                    recipe.id, rune.id, output.id
                ));
            }
        }

        assert!(
            unmarked.is_empty(),
            "rune patterns that leave no mark:
{unmarked:#?}"
        );
    }

    /// A pattern nothing asks for is a label. Each one has to be wanted by a
    /// request or rewarded by a bench, or the rune floor is making bottles that
    /// differ from ordinary ones only in price.
    #[test]
    fn every_rune_pattern_is_asked_for_by_something() {
        let data = load_embedded().expect("embedded game data should load");

        let mut wanted = std::collections::HashSet::new();
        for quest in &data.quests {
            wanted.insert(quest.required_trait.clone());
            wanted.extend(quest.required_traits.iter().cloned());
        }
        for recipe in &data.recipes {
            wanted.extend(recipe.preferred_traits.iter().cloned());
            wanted.extend(recipe.required_sequence.iter().cloned());
        }
        for station in &data.stations {
            wanted.extend(station.room_bonus.favored_traits.iter().cloned());
        }

        let ignored = data
            .items
            .iter()
            .filter(|item| item.category == crate::data::ItemCategory::Rune)
            .flat_map(|rune| rune.traits.iter())
            .filter(|held| *held != "arcane")
            .filter(|pattern| !wanted.contains(*pattern))
            .collect::<std::collections::BTreeSet<_>>();

        assert!(
            ignored.is_empty(),
            "rune patterns nothing wants: {ignored:?}"
        );
    }
}
