use super::{
    AlchemySession, ArchiveExperimentFilter, GameplayState, InventorySortMode, OverlayState,
    PlayerAvatar, ProgressionState, RuntimeState, TutorialState, WorldState,
};
use crate::data::GameData;
use macroquad::prelude::vec2;
use std::collections::{BTreeMap, HashSet};

impl GameplayState {
    pub fn new(data: &GameData) -> Self {
        let mut state = Self {
            world: WorldState {
                current_area_id: data.config.starting_area.clone(),
                player: PlayerAvatar {
                    position: vec2(
                        data.config.starting_position[0],
                        data.config.starting_position[1],
                    ),
                    facing: vec2(0.0, 1.0),
                    moving: false,
                },
                day_index: 0,
                day_clock_seconds: starting_day_time(data),
                day_length_seconds: data.config.day_length_seconds,
                gathered_nodes: HashSet::new(),
                available_nodes: HashSet::new(),
            },
            progression: ProgressionState {
                total_brews: 0,
                known_recipes: HashSet::new(),
                recipe_mastery: BTreeMap::new(),
                crafted_item_profiles: BTreeMap::new(),
                experiment_log: Vec::new(),
                unlocked_warps: HashSet::new(),
                planter_states: BTreeMap::new(),
                habitat_states: BTreeMap::new(),
                journal_milestones: initial_journal_milestones(),
                relationships: BTreeMap::new(),
                started_quests: HashSet::new(),
                completed_quests: HashSet::new(),
                herb_memories: BTreeMap::new(),
                potion_memories: BTreeMap::new(),
            },
            coins: 24,
            vitality: 100.0,
            inventory: BTreeMap::new(),
            runtime: RuntimeState {
                active_effects: Vec::new(),
                gather_toasts: Vec::new(),
                gather_feedbacks: Vec::new(),
                gather_pause_seconds: 0.0,
                camera_shake_seconds: 0.0,
                camera_shake_intensity: 0.0,
                sleep_flash_seconds: 0.0,
                npc_motion_states: BTreeMap::new(),
                status_text: String::new(),
                last_brew_setup: None,
                tutorial: TutorialState {
                    next_hint_delay_seconds: 1.5,
                    crow_intro_hint_shown: false,
                    save_hint_shown: false,
                    journal_hint_shown: false,
                    alchemy_hint_shown: false,
                    potion_hint_shown: false,
                    gather_hint_shown: false,
                    brew_goal_hint_shown: false,
                    mira_hint_shown: false,
                    rowan_hint_shown: false,
                    quest_hint_shown: false,
                    delivery_hint_shown: false,
                    route_hint_shown: false,
                },
                footstep_cooldown_seconds: 0.0,
                area_banner_area_id: data.config.starting_area.clone(),
                area_banner_label: data
                    .area(&data.config.starting_area)
                    .map(|area| area.name.clone())
                    .unwrap_or_default(),
                area_banner_seconds: 2.6,
            },
            ui: OverlayState {
                shop_buy_tab: true,
                inventory_sort_mode: InventorySortMode::Priority,
                archive_experiment_filter: ArchiveExperimentFilter::All,
                ..Default::default()
            },
            alchemy: AlchemySession::default(),
        };
        state.initialize_npc_motion_states(data);
        state.refresh_available_nodes(data);
        state
    }

}
