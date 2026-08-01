use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::schema::{
    AreaDefinition, GameConfig, GatheringRouteDefinition, ItemDefinition,
    MutationFormulaDefinition, NpcDefinition, QuestDefinition, RecipeDefinition,
    RuneRecipeDefinition, StationDefinition,
};

#[path = "game_data_access.rs"]
mod game_data_access;
#[path = "game_data_indexes.rs"]
mod game_data_indexes;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GameData {
    pub(crate) config: GameConfig,
    pub(crate) areas: Vec<AreaDefinition>,
    #[serde(default)]
    pub(crate) gathering_routes: Vec<GatheringRouteDefinition>,
    #[serde(default)]
    pub(crate) npcs: Vec<NpcDefinition>,
    #[serde(default)]
    pub(crate) quests: Vec<QuestDefinition>,
    pub(crate) items: Vec<ItemDefinition>,
    pub(crate) recipes: Vec<RecipeDefinition>,
    #[serde(default)]
    pub(crate) rune_recipes: Vec<RuneRecipeDefinition>,
    #[serde(default)]
    pub(crate) mutation_formulas: Vec<MutationFormulaDefinition>,
    pub(crate) stations: Vec<StationDefinition>,
    #[serde(skip)]
    area_index: HashMap<String, usize>,
    #[serde(skip)]
    item_index: HashMap<String, usize>,
    #[serde(skip)]
    route_index: HashMap<String, usize>,
    #[serde(skip)]
    npc_index: HashMap<String, usize>,
    #[serde(skip)]
    quest_index: HashMap<String, usize>,
    #[serde(skip)]
    mutation_formula_index: HashMap<String, Vec<usize>>,
}

pub(crate) struct GameDataParts {
    pub(crate) config: GameConfig,
    pub(crate) areas: Vec<AreaDefinition>,
    pub(crate) gathering_routes: Vec<GatheringRouteDefinition>,
    pub(crate) npcs: Vec<NpcDefinition>,
    pub(crate) quests: Vec<QuestDefinition>,
    pub(crate) items: Vec<ItemDefinition>,
    pub(crate) recipes: Vec<RecipeDefinition>,
    pub(crate) rune_recipes: Vec<RuneRecipeDefinition>,
    pub(crate) mutation_formulas: Vec<MutationFormulaDefinition>,
    pub(crate) stations: Vec<StationDefinition>,
}

impl GameData {
    pub(crate) fn from_parts(parts: GameDataParts) -> Result<Self, String> {
        let mut data = Self {
            config: parts.config,
            areas: parts.areas,
            gathering_routes: parts.gathering_routes,
            npcs: parts.npcs,
            quests: parts.quests,
            items: parts.items,
            recipes: parts.recipes,
            rune_recipes: parts.rune_recipes,
            mutation_formulas: parts.mutation_formulas,
            stations: parts.stations,
            area_index: HashMap::new(),
            item_index: HashMap::new(),
            route_index: HashMap::new(),
            npc_index: HashMap::new(),
            quest_index: HashMap::new(),
            mutation_formula_index: HashMap::new(),
        };
        data.build_indexes()?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::data::load_embedded;

    /// The brew matcher keys on the exact ingredient multiset at a station, so
    /// two recipes sharing one would make the second unreachable.
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
    fn obtainable_item_ids(data: &super::GameData) -> std::collections::HashSet<String> {
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
    fn recordable_milestone_ids(data: &super::GameData) -> std::collections::HashSet<String> {
        use crate::content::narrative_text;

        let mut recordable = std::collections::HashSet::new();
        for milestone in narrative_text().milestones.all() {
            recordable.insert(milestone.id.clone());
        }
        for quest in &data.quests {
            recordable.extend(quest.completion_milestones.iter().map(|m| m.id.clone()));
        }
        for area in &data.areas {
            for warp in &area.warps {
                recordable.extend(warp.unlock_milestones.iter().map(|m| m.id.clone()));
            }
        }
        recordable
    }

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
    fn reachable_traits(
        data: &super::GameData,
        item_id: &str,
    ) -> std::collections::BTreeSet<String> {
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
    fn areas_open_at_the_start(data: &super::GameData) -> std::collections::HashSet<String> {
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

    /// A town reaction is prose gated on a condition. A quest or milestone id
    /// that does not exist gives a line that is written, shipped, and never
    /// spoken — the most expensive kind of typo in a content file.
    #[test]
    fn town_reactions_are_gated_on_real_beats() {
        use crate::content::narrative_text;

        let data = load_embedded().expect("embedded game data should load");
        let narrative = narrative_text();

        let mut known_milestones = std::collections::HashSet::new();
        for milestone in narrative.milestones.all() {
            known_milestones.insert(milestone.id.clone());
        }
        for quest in &data.quests {
            known_milestones.extend(quest.completion_milestones.iter().map(|m| m.id.clone()));
        }
        for area in &data.areas {
            for warp in &area.warps {
                known_milestones.extend(warp.unlock_milestones.iter().map(|m| m.id.clone()));
            }
        }

        let mut unreachable = Vec::new();
        for reaction in &narrative.reactions {
            if data.npc(&reaction.npc_id).is_none() {
                unreachable.push(format!("{}: no such townsperson", reaction.npc_id));
            }
            if !reaction.after_quest.is_empty() && data.quest(&reaction.after_quest).is_none() {
                unreachable.push(format!(
                    "{} waits on quest {}, which does not exist",
                    reaction.npc_id, reaction.after_quest
                ));
            }
            if !reaction.after_milestone.is_empty()
                && !known_milestones.contains(&reaction.after_milestone)
            {
                unreachable.push(format!(
                    "{} waits on milestone {}, which nothing records",
                    reaction.npc_id, reaction.after_milestone
                ));
            }
        }

        // The epilogue is gated the same way, and a beat that can never be
        // earned is the one piece of prose nobody would ever notice missing.
        for beat in &narrative.epilogue_beats {
            for milestone_id in &beat.after_milestones {
                if !known_milestones.contains(milestone_id) {
                    unreachable.push(format!(
                        "epilogue beat waits on milestone {milestone_id}, which nothing records"
                    ));
                }
            }
        }

        assert!(
            unreachable.is_empty(),
            "narrative gated on beats that never happen:\n{unreachable:#?}"
        );
    }
}
