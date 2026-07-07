use super::{AreaRenderDefinition, GatherNodeRenderDefinition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GameConfig {
    pub(crate) starting_area: String,
    pub(crate) starting_position: [f32; 2],
    pub(crate) move_speed: f32,
    pub(crate) interaction_range: f32,
    pub(crate) day_length_seconds: f32,
    pub(crate) save_version: u32,
    #[serde(default)]
    pub(crate) archive_required_completed_quests: Vec<String>,
    #[serde(default)]
    pub(crate) archive_required_journal_milestones: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RectDefinition {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl RectDefinition {
    pub(crate) fn contains_xy(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct JournalMilestoneEntry {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct WarpDefinition {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) rect: RectDefinition,
    pub(crate) target_area: String,
    pub(crate) target_position: [f32; 2],
    #[serde(default)]
    pub(crate) required_total_brews: u32,
    #[serde(default)]
    pub(crate) required_coins: u32,
    #[serde(default)]
    pub(crate) required_item_id: String,
    #[serde(default)]
    pub(crate) required_item_amount: u32,
    #[serde(default)]
    pub(crate) required_journal_milestone: String,
    /// Recipe that must be brought to the "mastered" mastery stage before this
    /// warp opens. A skill gate rather than a grind gate — see
    /// `alchemy::MASTERED_BREW_COUNT`.
    #[serde(default)]
    pub(crate) required_mastered_recipe: String,
    #[serde(default)]
    pub(crate) required_journal_hint: String,
    #[serde(default)]
    pub(crate) unlock_milestones: Vec<JournalMilestoneEntry>,
    #[serde(default)]
    pub(crate) locked_note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GatherNodeDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) item_id: String,
    pub(crate) color: [u8; 4],
    pub(crate) position: [f32; 2],
    pub(crate) radius: f32,
    #[serde(default)]
    pub(crate) route_id: String,
    #[serde(default)]
    pub(crate) seasons: Vec<String>,
    #[serde(default)]
    pub(crate) weathers: Vec<String>,
    #[serde(default)]
    pub(crate) time_windows: Vec<String>,
    #[serde(default = "default_spawn_chance")]
    pub(crate) spawn_chance: u32,
    #[serde(default)]
    pub(crate) note: String,
    #[serde(default)]
    pub(crate) render: GatherNodeRenderDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GatheringRouteDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) area_id: String,
    pub(crate) description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct QuestDefinition {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) required_item_id: String,
    pub(crate) required_amount: u32,
    pub(crate) reward_coins: u32,
    pub(crate) giver_npc_id: String,
    #[serde(default)]
    pub(crate) minimum_quality_band: String,
    #[serde(default)]
    pub(crate) required_trait: String,
    #[serde(default)]
    pub(crate) required_traits: Vec<String>,
    #[serde(default)]
    pub(crate) minimum_trait_matches: u32,
    #[serde(default)]
    pub(crate) required_effect_kind: String,
    #[serde(default)]
    pub(crate) required_effect_kinds: Vec<String>,
    #[serde(default)]
    pub(crate) minimum_effect_matches: u32,
    #[serde(default)]
    pub(crate) prerequisite_quests: Vec<String>,
    #[serde(default)]
    pub(crate) required_unlocked_warp: String,
    #[serde(default)]
    pub(crate) minimum_total_brews: u32,
    #[serde(default)]
    pub(crate) completion_milestones: Vec<JournalMilestoneEntry>,
    /// Board requests only: when true, the request returns to the board after a
    /// cooldown instead of being permanently completed, giving the mid-game a
    /// recurring reason to brew and deliver.
    #[serde(default)]
    pub(crate) repeatable: bool,
    /// Days after a repeatable request is delivered before it is offered again
    /// (clamped to at least 1 day at delivery time).
    #[serde(default)]
    pub(crate) repeat_cooldown_days: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AreaDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) size: [f32; 2],
    pub(crate) background: [u8; 4],
    pub(crate) accent: [u8; 4],
    #[serde(default = "default_footstep_sound_set")]
    pub(crate) footstep_sound_set: String,
    pub(crate) blockers: Vec<RectDefinition>,
    pub(crate) warps: Vec<WarpDefinition>,
    pub(crate) gather_nodes: Vec<GatherNodeDefinition>,
    #[serde(default)]
    pub(crate) render: AreaRenderDefinition,
}

fn default_spawn_chance() -> u32 {
    100
}

fn default_footstep_sound_set() -> String {
    "dirt_path".to_owned()
}
