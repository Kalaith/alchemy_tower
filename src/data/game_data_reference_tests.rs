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

    /// Every journal beat the game can ever record, from any source.
    /// A plain ingredient earns its place by being wanted somewhere. Two herbs
    /// on the southern pass — `coldiron_lichen` and `rimeflower` — were authored
    /// with conditions, variants and prose, and then no recipe in the game
    /// asked for either, so the newest biome's whole harvest fed nothing. They
    /// were pickable, describable, and pointless.
    ///
    /// Creatures and catalysts are excluded deliberately: creatures feed
    /// habitats and catalysts are matched by `catalyst_tag` rather than by id,
    /// so neither shows up in a recipe's ingredient list even when in demand.
    /// Two separate things this pins, both found the same way — by checking
    /// authored positions against the map rather than looking at a capture.
    ///
    /// Every townsperson must have somewhere to be in all four time windows.
    /// The whole cast had morning, day and evening and nothing else, and
    /// `active_schedule_index` falls back to the last entry, so from nine at
    /// night until six the next morning the entire valley stood frozen on its
    /// teatime mark — which stopped being invisible the moment night became
    /// worth going out in.
    ///
    /// And no mark may sit inside a blocker. Mayor Elric's day and evening
    /// spots were both inside the hall's footprint: reachable, since 48px is
    /// inside a 56px interaction radius, but rendering him through a wall.
    /// Blockers stop the player with a 14px body radius, so anything buried
    /// deeper inside one than its own reach is content you can see and never
    /// touch. `every_gather_node_can_actually_spawn` does not catch this — the
    /// node spawns perfectly, you simply cannot walk to it.
    ///
    /// Three were buried. The worst was `hollow_ashcap_01`, 88px inside a tree
    /// against a 44px reach, and the **only** source of ashcap in the game, so
    /// the two recipes calling for it could never be brewed. The greenhouse's
    /// north planter is inside a blocker too and is fine, because the blocker
    /// is the raised bed itself and 48px of reach clears it — which is why this
    /// measures reach rather than flagging overlap.
    #[test]
    fn everything_the_player_must_reach_can_be_stood_next_to() {
        /// Matches `PLAYER_RADIUS` in the movement code.
        const PLAYER_RADIUS: f32 = 14.0;

        let data = load_embedded().expect("embedded game data should load");
        let reach_bonus = data.config.interaction_range;
        let mut unreachable = Vec::new();

        // How close the player can get to a point sunk inside scenery.
        let closest_approach =
            |area: &crate::data::AreaDefinition, x: f32, y: f32| -> Option<f32> {
                area.blockers
                    .iter()
                    .filter(|b| x >= b.x && x <= b.x + b.w && y >= b.y && y <= b.y + b.h)
                    .map(|b| {
                        let depth = (x - b.x).min(b.x + b.w - x).min(y - b.y).min(b.y + b.h - y);
                        depth + PLAYER_RADIUS
                    })
                    .fold(None, |acc: Option<f32>, d| {
                        Some(acc.map_or(d, |a| a.max(d)))
                    })
            };

        for area in &data.areas {
            for node in &area.gather_nodes {
                if let Some(approach) = closest_approach(area, node.position[0], node.position[1]) {
                    let reach = node.radius + reach_bonus;
                    if approach > reach {
                        unreachable.push(format!(
                            "gather node {} in {}: closest {approach:.0}, reach {reach:.0}",
                            node.id, area.id
                        ));
                    }
                }
            }
        }
        for station in &data.stations {
            let Some(area) = data.area(&station.area_id) else {
                continue;
            };
            if let Some(approach) = closest_approach(area, station.position[0], station.position[1])
            {
                if approach > station.interaction_radius {
                    unreachable.push(format!(
                        "station {} in {}: closest {approach:.0}, reach {:.0}",
                        station.id, area.id, station.interaction_radius
                    ));
                }
            }
        }
        unreachable.sort();

        assert!(
            unreachable.is_empty(),
            "things the player can see but never walk up to:
{unreachable:#?}"
        );
    }

    #[test]
    fn every_townsperson_has_somewhere_to_be_at_every_hour() {
        const WINDOWS: [&str; 4] = ["morning", "day", "evening", "night"];
        // A body has width; standing flush against a wall reads as inside it.
        const CLEARANCE: f32 = 18.0;

        let data = load_embedded().expect("embedded game data should load");
        let mut complaints = Vec::new();

        for npc in &data.npcs {
            for window in WINDOWS {
                if !npc.schedule.iter().any(|entry| entry.time_window == window) {
                    complaints.push(format!("{}: nowhere to be at {window}", npc.id));
                }
            }

            for entry in &npc.schedule {
                let Some(area) = data.area(&entry.area_id) else {
                    complaints.push(format!(
                        "{} at {}: no such area {}",
                        npc.id, entry.time_window, entry.area_id
                    ));
                    continue;
                };
                let [x, y] = entry.position;
                if x < 0.0 || y < 0.0 || x > area.size[0] || y > area.size[1] {
                    complaints.push(format!(
                        "{} at {}: {x},{y} is outside {}",
                        npc.id, entry.time_window, area.id
                    ));
                }
                for blocker in &area.blockers {
                    if x >= blocker.x - CLEARANCE
                        && x <= blocker.x + blocker.w + CLEARANCE
                        && y >= blocker.y - CLEARANCE
                        && y <= blocker.y + blocker.h + CLEARANCE
                    {
                        complaints.push(format!(
                            "{} at {}: {x},{y} stands in scenery in {}",
                            npc.id, entry.time_window, area.id
                        ));
                    }
                }
            }
        }
        complaints.sort();

        assert!(
            complaints.is_empty(),
            "townsfolk standing somewhere they should not be:
{complaints:#?}"
        );
    }

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
}
