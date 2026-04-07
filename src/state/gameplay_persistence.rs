use super::*;

pub(super) struct GameplayStateLoader;

impl GameplayStateLoader {
    pub(super) fn save_slot(state: &GameplayState, data: &GameData) -> Result<(), String> {
        SaveRepository::save(&Self::snapshot(state, data))
    }

    pub(super) fn load_slot(state: &mut GameplayState, data: &GameData) -> Result<(), String> {
        let save = SaveRepository::load()?;
        Self::apply_snapshot(state, data, save)
    }

    fn snapshot(state: &GameplayState, data: &GameData) -> SaveData {
        // Runtime-only UI/session fields intentionally stay transient across saves.
        SaveData {
            version: data.config.save_version,
            current_area: state.world.current_area_id.clone(),
            player_position: [state.world.player.position.x, state.world.player.position.y],
            day_clock_seconds: state.world.day_clock_seconds,
            vitality: state.vitality,
            total_brews: state.progression.total_brews,
            coins: state.coins,
            inventory: state
                .inventory
                .iter()
                .map(|(item_id, amount)| InventoryEntry {
                    item_id: item_id.clone(),
                    amount: *amount,
                })
                .collect(),
            gathered_nodes: state.world.gathered_nodes.iter().cloned().collect(),
            known_recipes: state.progression.known_recipes.iter().cloned().collect(),
            day_index: state.world.day_index,
            field_journal: state.progression.field_journal.values().cloned().collect(),
            started_quests: state.progression.started_quests.iter().cloned().collect(),
            completed_quests: state.progression.completed_quests.iter().cloned().collect(),
            recipe_mastery: state
                .progression
                .recipe_mastery
                .iter()
                .map(|(recipe_id, successful_brews)| RecipeMasteryEntry {
                    recipe_id: recipe_id.clone(),
                    successful_brews: *successful_brews,
                })
                .collect(),
            crafted_item_profiles: state.progression.crafted_item_profiles.values().cloned().collect(),
            experiment_log: state.progression.experiment_log.clone(),
            unlocked_warps: state.progression.unlocked_warps.iter().cloned().collect(),
            planter_states: state.progression.planter_states.values().cloned().collect(),
            habitat_states: state.progression.habitat_states.values().cloned().collect(),
            journal_milestones: state.progression.journal_milestones.clone(),
            relationships: state
                .progression
                .relationships
                .iter()
                .map(|(npc_id, value)| RelationshipEntry {
                    npc_id: npc_id.clone(),
                    value: *value,
                })
                .collect(),
        }
    }

    fn apply_snapshot(
        state: &mut GameplayState,
        data: &GameData,
        save: SaveData,
    ) -> Result<(), String> {
        if save.version != data.config.save_version {
            return Err(format!(
                "Save version {} is incompatible with expected version {}.",
                save.version, data.config.save_version
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
        state.progression.field_journal = save
            .field_journal
            .into_iter()
            .map(|entry| (entry.item_id.clone(), entry))
            .collect();
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
        state.progression.journal_milestones = if save.journal_milestones.is_empty() {
            initial_journal_milestones()
        } else {
            save.journal_milestones
        };
        state.progression.relationships = save
            .relationships
            .into_iter()
            .map(|entry| (entry.npc_id, entry.value))
            .collect();
        state.world.available_nodes.clear();
        state.ui = OverlayState {
            shop_buy_tab: true,
            ..Default::default()
        };
        state.alchemy = AlchemySession::default();
        state.refresh_available_nodes(data);
        Ok(())
    }
}


