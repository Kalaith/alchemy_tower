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
    /// Sounds the game has decided to make but has not played yet.
    ///
    /// The moments worth hearing — a beat recorded, a request finished, a route
    /// opened, a day run out — are triggered deep in state code that has no
    /// `AudioAssets` and should not grow one. They queue here and the frame
    /// loop drains them, which is exactly how the visual feedbacks beside them
    /// already work.
    pub(super) pending_sounds: Vec<GameSound>,
}

/// A moment worth hearing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GameSound {
    JournalNote,
    WorkLanded,
    RouteRestored,
    CollapseHome,
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
            pending_sounds: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct TutorialState {
    /// A frame timer, which is why this one stays in runtime: it paces the
    /// hints within a session and means nothing across a save. Which hints have
    /// already been shown moved to `progression.shown_tutorial_hints`, because
    /// runtime is rebuilt on load and the opening three fire unconditionally.
    pub(super) next_hint_delay_seconds: f32,
}

impl Default for TutorialState {
    fn default() -> Self {
        Self {
            next_hint_delay_seconds: 1.5,
        }
    }
}
