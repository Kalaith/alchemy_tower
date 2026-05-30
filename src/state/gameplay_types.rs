use std::collections::BTreeMap;

use super::gameplay_alchemy_types::AlchemySession;
use super::gameplay_overlay_types::OverlayState;
use super::gameplay_progression_types::ProgressionState;
use super::gameplay_runtime_types::RuntimeState;
use super::gameplay_world_types::WorldState;

#[derive(Clone, Debug)]
pub(crate) struct GameplayState {
    pub(super) world: WorldState,
    pub(super) progression: ProgressionState,
    pub(super) coins: u32,
    pub(super) vitality: f32,
    pub(super) inventory: BTreeMap<String, u32>,
    pub(super) runtime: RuntimeState,
    pub(super) ui: OverlayState,
    pub(super) alchemy: AlchemySession,
}
