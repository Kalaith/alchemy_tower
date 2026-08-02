use super::gameplay_alchemy_types::AlchemySession;
use super::gameplay_overlay_types::OverlayState;
use super::gameplay_save_migrations::{restored_herb_memories, restored_journal_milestones};
use super::GameplayState;
use crate::data::{GameData, SaveData};

#[path = "gameplay_save_restore_text.rs"]
mod save_restore_text;

pub(super) fn apply_save_snapshot(
    state: &mut GameplayState,
    data: &GameData,
    save: SaveData,
) -> Result<(), String> {
    if save.version != data.config.save_version {
        return Err(save_restore_text::incompatible_version(
            save.version,
            data.config.save_version,
        ));
    }
    if data.area(&save.current_area).is_none() {
        return Err(save_restore_text::unknown_area());
    }

    state.world.current_area_id = save.current_area;
    state.set_player_position(save.player_position);
    state.set_player_facing([0.0, 1.0]);
    state.stop_player_motion();
    state.world.day_index = save.day_index;
    state.world.day_clock_seconds = save.day_clock_seconds;
    state.world.day_length_seconds = data.config.day_length_seconds;
    state.vitality = save.vitality.clamp(0.0, 100.0);
    state.progression.total_brews = save.total_brews;
    state.coins = save.coins;
    state.inventory = save
        .inventory
        .into_iter()
        .map(|entry| (entry.item_id, entry.amount))
        .collect();
    state.world.gathered_nodes = save.gathered_nodes.into_iter().collect();
    state.progression.known_recipes = save.known_recipes.into_iter().collect();
    state.progression.herb_memories = restored_herb_memories(
        save.herb_memories,
        save.field_journal,
        state.world.day_index,
    );
    state.progression.started_quests = save.started_quests.into_iter().collect();
    state.progression.completed_quests = save.completed_quests.into_iter().collect();
    state.progression.recipe_mastery = save
        .recipe_mastery
        .into_iter()
        .map(|entry| (entry.recipe_id, entry.successful_brews))
        .collect();
    state.progression.crafted_item_profiles = save
        .crafted_item_profiles
        .into_iter()
        .map(|entry| (entry.item_id.clone(), entry))
        .collect();
    state.progression.experiment_log = save.experiment_log;
    state.progression.potion_memories = save
        .potion_memories
        .into_iter()
        .map(|entry| (entry.item_id.clone(), entry))
        .collect();
    state.progression.unlocked_warps = save.unlocked_warps.into_iter().collect();
    state.progression.planter_states = save
        .planter_states
        .into_iter()
        .map(|entry| (entry.station_id.clone(), entry))
        .collect();
    state.progression.habitat_states = save
        .habitat_states
        .into_iter()
        .map(|entry| (entry.station_id.clone(), entry))
        .collect();
    state.progression.journal_milestones = restored_journal_milestones(save.journal_milestones);
    state.progression.relationships = save
        .relationships
        .into_iter()
        .map(|entry| (entry.npc_id, entry.value))
        .collect();
    state.progression.board_quest_cooldowns = save
        .board_quest_cooldowns
        .into_iter()
        .map(|entry| (entry.quest_id, entry.available_day))
        .collect();
    state.progression.variant_stock.clear();
    for entry in save.variant_stock {
        if entry.count == 0 {
            continue;
        }
        state
            .progression
            .variant_stock
            .entry(entry.item_id)
            .or_default()
            .insert(entry.variant_id, entry.count);
    }
    state.progression.bottle_stock.clear();
    for entry in save.bottle_stock {
        if entry.count == 0 {
            continue;
        }
        state
            .progression
            .bottle_stock
            .entry(entry.item_id.clone())
            .or_default()
            .push(entry);
    }
    for batches in state.progression.bottle_stock.values_mut() {
        batches.sort_by_key(|batch| batch.quality_score);
    }
    state.progression.spoken_reactions = save.spoken_reactions.into_iter().collect();
    state.progression.salvage_familiarity = save
        .salvage_familiarity
        .into_iter()
        .map(|entry| (entry.signature, entry.attempts))
        .collect();
    state.progression.treated_targets = save.treated_targets.into_iter().collect();
    state.progression.shown_tutorial_hints = save.shown_tutorial_hints.into_iter().collect();
    state.world.available_nodes.clear();
    state.ui = OverlayState::new_gameplay();
    state.alchemy = AlchemySession::default();
    state.rebuild_memory_state(data);
    state.refresh_available_nodes(data);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::gameplay_save_snapshot::build_save_snapshot;
    use super::super::GameplayState;
    use crate::data::{
        CraftedItemProfileEntry, ExperimentLogEntry, HabitatStateEntry, HerbMemoryEntry,
        JournalMilestoneEntry, PlanterStateEntry, PotionMemoryEntry,
    };

    /// Twenty-five passes have added fields to the progression state, and the
    /// save system had no test of any kind. A field that exists in both the
    /// state and the save file but is not actually carried by snapshot/restore
    /// loses a player's progress silently, which is the worst way to lose it.
    #[test]
    fn a_save_round_trip_keeps_everything_the_game_tracks() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        // Make every tracked field distinctive, so a dropped one cannot pass by
        // coincidentally matching a fresh game's default.
        state.coins = 1234;
        state.vitality = 61.5;
        state.inventory.insert("whisper_moss".to_owned(), 7);
        state.inventory.insert("sunleaf".to_owned(), 3);
        state.world.day_index = 19;
        state.world.day_clock_seconds = 421.0;
        state.world.current_area_id = "north_plains".to_owned();
        state.world.player.position = macroquad::prelude::vec2(321.0, 654.0);
        state
            .world
            .gathered_nodes
            .insert("plains_sunleaf_01".to_owned());

        let p = &mut state.progression;
        p.total_brews = 42;
        p.known_recipes.insert("healing_draught_recipe".to_owned());
        p.recipe_mastery.insert("glow_potion_recipe".to_owned(), 5);
        p.unlocked_warps.insert("entry_to_plains".to_owned());
        p.started_quests.insert("glow_for_rowan".to_owned());
        p.completed_quests.insert("healing_for_mira".to_owned());
        p.relationships.insert("mira_apothecary".to_owned(), 9);
        p.board_quest_cooldowns
            .insert("board_lantern_supply".to_owned(), 23);
        p.bottle_stock.insert(
            "healing_draught".to_owned(),
            vec![crate::data::BottleBatchEntry {
                item_id: "healing_draught".to_owned(),
                quality_score: 71,
                quality_band: "Excellent".to_owned(),
                traits: vec!["restorative".to_owned()],
                count: 1,
            }],
        );
        p.spoken_reactions.insert("deadbeefdeadbeef".to_owned());
        p.shown_tutorial_hints
            .insert("tutorial_crow_intro".to_owned());
        p.salvage_familiarity
            .insert("entry_cauldron|sunleaf+whisper_moss".to_owned(), 2);
        p.variant_stock.insert(
            "whisper_moss".to_owned(),
            [("whisper_moss_dew".to_owned(), 2u32)]
                .into_iter()
                .collect(),
        );
        p.journal_milestones.push(JournalMilestoneEntry {
            id: "test_beat".to_owned(),
            title: "Test Beat".to_owned(),
            text: "A beat recorded before saving.".to_owned(),
        });
        p.crafted_item_profiles.insert(
            "healing_draught".to_owned(),
            CraftedItemProfileEntry {
                item_id: "healing_draught".to_owned(),
                best_quality_score: 77,
                best_quality_band: "Excellent".to_owned(),
                inherited_traits: vec!["restorative".to_owned()],
                effect_kinds: vec!["restore".to_owned()],
            },
        );
        p.experiment_log.push(ExperimentLogEntry {
            recipe_id: "healing_draught_recipe".to_owned(),
            output_item_id: "healing_draught".to_owned(),
            quality_score: 71,
            quality_band: "Excellent".to_owned(),
            stable: true,
            catalyst_item_id: String::new(),
            morph_output_item_id: String::new(),
            day_index: 12,
        });
        p.planter_states.insert(
            "greenhouse_planter_west".to_owned(),
            PlanterStateEntry {
                station_id: "greenhouse_planter_west".to_owned(),
                planted_item_id: "ember_root".to_owned(),
                planted_day: 4,
                tended_day: 5,
                tended_days: 1,
                growth_days: 2,
                ready: false,
                mutation_formula_id: String::new(),
                mutation_note: String::new(),
                mutation_yield_bonus: 0,
                mutation_growth_bonus_days: 0,
            },
        );
        p.habitat_states.insert(
            "containment_habitat_moth".to_owned(),
            HabitatStateEntry {
                station_id: "containment_habitat_moth".to_owned(),
                creature_item_id: "glow_moth".to_owned(),
                last_harvest_day: 8,
            },
        );
        p.herb_memories.insert(
            "whisper_moss".to_owned(),
            HerbMemoryEntry {
                item_id: "whisper_moss".to_owned(),
                first_seen_day: 1,
                first_seen_route_id: "plains_crossing".to_owned(),
                seen: true,
                learned: true,
                learned_day: 3,
                learned_route_id: "plains_crossing".to_owned(),
                note: "kept".to_owned(),
                best_quality: 33,
                best_quality_band: "Fine".to_owned(),
                variant_name: String::new(),
            },
        );
        p.potion_memories.insert(
            "healing_draught".to_owned(),
            PotionMemoryEntry {
                item_id: "healing_draught".to_owned(),
                seen: true,
                learned: true,
                first_seen_day: 2,
                learned_day: 3,
                last_recipe_id: "healing_draught_recipe".to_owned(),
                best_quality_score: 71,
                best_quality_band: "Excellent".to_owned(),
                successful_brews: 4,
            },
        );

        let snapshot = build_save_snapshot(&state, &data);
        let mut restored = GameplayState::new(&data);
        super::apply_save_snapshot(&mut restored, &data, snapshot).expect("the save should load");

        assert_eq!(restored.coins, state.coins, "coins");
        assert_eq!(restored.vitality, state.vitality, "vitality");
        assert_eq!(restored.inventory, state.inventory, "inventory");
        assert_eq!(restored.world.day_index, state.world.day_index, "day");
        assert_eq!(
            restored.world.current_area_id, state.world.current_area_id,
            "area"
        );
        assert_eq!(
            restored.world.gathered_nodes, state.world.gathered_nodes,
            "gathered nodes"
        );

        let (a, b) = (&restored.progression, &state.progression);
        assert_eq!(a.total_brews, b.total_brews, "total brews");
        assert_eq!(a.known_recipes, b.known_recipes, "known recipes");
        assert_eq!(a.recipe_mastery, b.recipe_mastery, "recipe mastery");
        assert_eq!(a.unlocked_warps, b.unlocked_warps, "unlocked warps");
        assert_eq!(a.started_quests, b.started_quests, "started quests");
        assert_eq!(a.completed_quests, b.completed_quests, "completed quests");
        assert_eq!(a.relationships, b.relationships, "relationships");
        assert_eq!(
            a.board_quest_cooldowns, b.board_quest_cooldowns,
            "board cooldowns"
        );
        assert_eq!(
            a.crafted_item_profiles.len(),
            b.crafted_item_profiles.len(),
            "crafted profiles"
        );
        assert_eq!(
            a.experiment_log.len(),
            b.experiment_log.len(),
            "experiments"
        );
        // Which held units were gathered as variants lives only in the save;
        // losing it silently downgrades the player's stock back to plain.
        assert_eq!(a.variant_stock, b.variant_stock, "variant stock");
        // Which lines a townsperson has already said lives only in the save;
        // losing it makes the whole town repeat their opening remarks.
        assert_eq!(a.spoken_reactions, b.spoken_reactions, "spoken reactions");
        // And the same for the opening hints, which fire on no condition at
        // all: losing this reintroduces the crow to a player forty hours in.
        assert_eq!(
            a.shown_tutorial_hints, b.shown_tutorial_hints,
            "tutorial hints already shown"
        );
        // Losing this makes every off-book mixture the player had half worked
        // out start again from a blind guess.
        assert_eq!(
            a.salvage_familiarity, b.salvage_familiarity,
            "salvage familiarity"
        );
        // What each bottle is worth lives only in the save; losing it would put
        // the quality gates back to reading a best-ever record.
        assert_eq!(a.bottle_stock, b.bottle_stock, "bottle stock");
        assert_eq!(a.planter_states.len(), b.planter_states.len(), "planters");
        // Not just the count: a bed's growth is part elapsed time and part days
        // tended, and the tended half exists only in the save. Losing it would
        // look exactly like the midnight overwrite this field was added to fix.
        let restored_bed = a
            .planter_states
            .get("greenhouse_planter_west")
            .expect("the saved planter should survive");
        let saved_bed = b
            .planter_states
            .get("greenhouse_planter_west")
            .expect("fixture planter");
        assert_eq!(
            (
                &restored_bed.planted_item_id,
                restored_bed.planted_day,
                restored_bed.tended_day,
                restored_bed.tended_days,
                restored_bed.growth_days,
            ),
            (
                &saved_bed.planted_item_id,
                saved_bed.planted_day,
                saved_bed.tended_day,
                saved_bed.tended_days,
                saved_bed.growth_days,
            ),
            "planter growth state"
        );
        assert_eq!(a.habitat_states.len(), b.habitat_states.len(), "habitats");
        // Restoring deliberately *rebuilds* potion memories from the inventory,
        // the experiment log, known recipes and crafted profiles, so a loaded
        // save can hold more of them than the one that was written. Assert the
        // saved entry survives intact rather than that the counts match.
        let restored_draught = a
            .potion_memories
            .get("healing_draught")
            .expect("the saved potion memory should survive");
        let saved_draught = b
            .potion_memories
            .get("healing_draught")
            .expect("fixture potion memory");
        assert_eq!(
            restored_draught.successful_brews, saved_draught.successful_brews,
            "potion memory brew count"
        );
        assert_eq!(
            restored_draught.best_quality_band, saved_draught.best_quality_band,
            "potion memory best band"
        );
        assert!(
            a.potion_memories.len() >= b.potion_memories.len(),
            "restoring should never drop a potion memory"
        );
        assert!(
            a.journal_milestones.iter().any(|m| m.id == "test_beat"),
            "journal milestones"
        );
        assert!(
            a.herb_memories.contains_key("whisper_moss"),
            "herb memories"
        );
    }
}
