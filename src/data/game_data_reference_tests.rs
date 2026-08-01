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
