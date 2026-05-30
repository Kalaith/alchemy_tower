use super::gameplay_alchemy_types::AlchemySession;
use super::gameplay_overlay_types::OverlayState;
use super::gameplay_save_migrations::{restored_herb_memories, restored_journal_milestones};
use super::GameplayState;
use crate::content::{narrative_text, ui_format};
use crate::data::{GameData, SaveData};
use macroquad::prelude::vec2;

pub(super) fn apply_save_snapshot(
    state: &mut GameplayState,
    data: &GameData,
    save: SaveData,
) -> Result<(), String> {
    if save.version != data.config.save_version {
        return Err(ui_format(
            "gameplay_save_version_incompatible",
            &[
                ("found", &save.version.to_string()),
                ("expected", &data.config.save_version.to_string()),
            ],
        ));
    }
    if data.area(&save.current_area).is_none() {
        return Err(narrative_text().statuses.save_unknown_area.clone());
    }

    state.world.current_area_id = save.current_area;
    state.world.player.position = vec2(save.player_position[0], save.player_position[1]);
    state.world.player.facing = vec2(0.0, 1.0);
    state.world.player.moving = false;
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
    state.progression.herb_memories =
        restored_herb_memories(save.herb_memories, save.field_journal, state.world.day_index);
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
    state.world.available_nodes.clear();
    state.ui = OverlayState::new_gameplay();
    state.alchemy = AlchemySession::default();
    state.rebuild_memory_state(data);
    state.refresh_available_nodes(data);
    Ok(())
}
