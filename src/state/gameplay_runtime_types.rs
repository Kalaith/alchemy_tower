use std::collections::BTreeMap;

use super::gameplay_alchemy_types::SavedAlchemySetup;
use super::gameplay_feedback_types::{ActiveEffect, GatherFeedback, GatherToast};
use super::gameplay_npc_types::NpcMotionTracker;
use crate::data::GameData;
use macroquad_toolkit::fx::ScreenShake;

#[derive(Clone, Debug)]
pub(super) struct RuntimeState {
    pub(super) active_effects: Vec<ActiveEffect>,
    pub(super) gather_toasts: Vec<GatherToast>,
    pub(super) gather_feedbacks: Vec<GatherFeedback>,
    pub(super) gather_pause_seconds: f32,
    pub(super) camera_shake: ScreenShake,
    pub(super) sleep_flash_seconds: f32,
    pub(super) npc_motion_states: BTreeMap<String, NpcMotionTracker>,
    pub(super) status_text: String,
    pub(super) last_brew_setup: Option<SavedAlchemySetup>,
    pub(super) tutorial: TutorialState,
    pub(super) footstep_cooldown_seconds: f32,
    pub(super) area_banner_area_id: String,
    pub(super) area_banner_label: String,
    pub(super) area_banner_seconds: f32,
}

impl RuntimeState {
    pub(super) fn new(data: &GameData) -> Self {
        Self {
            active_effects: Vec::new(),
            gather_toasts: Vec::new(),
            gather_feedbacks: Vec::new(),
            gather_pause_seconds: 0.0,
            camera_shake: ScreenShake::new(0.0),
            sleep_flash_seconds: 0.0,
            npc_motion_states: BTreeMap::new(),
            status_text: String::new(),
            last_brew_setup: None,
            tutorial: TutorialState::default(),
            footstep_cooldown_seconds: 0.0,
            area_banner_area_id: data.config.starting_area.clone(),
            area_banner_label: data
                .area(&data.config.starting_area)
                .map(|area| area.name.clone())
                .unwrap_or_default(),
            area_banner_seconds: 2.6,
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct TutorialState {
    pub(super) next_hint_delay_seconds: f32,
    pub(super) crow_intro_hint_shown: bool,
    pub(super) save_hint_shown: bool,
    pub(super) journal_hint_shown: bool,
    pub(super) alchemy_hint_shown: bool,
    pub(super) potion_hint_shown: bool,
    pub(super) gather_hint_shown: bool,
    pub(super) brew_goal_hint_shown: bool,
    pub(super) mira_hint_shown: bool,
    pub(super) rowan_hint_shown: bool,
    pub(super) quest_hint_shown: bool,
    pub(super) delivery_hint_shown: bool,
    pub(super) route_hint_shown: bool,
}

impl Default for TutorialState {
    fn default() -> Self {
        Self {
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
        }
    }
}
