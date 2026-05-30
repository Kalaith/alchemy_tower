use super::{
    AreaRenderDefinition, GatherNodeRenderDefinition, RoomBonusDefinition, StationKind,
    StationRenderDefinition,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub starting_area: String,
    pub starting_position: [f32; 2],
    pub move_speed: f32,
    pub interaction_range: f32,
    pub day_length_seconds: f32,
    pub save_version: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RectDefinition {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl RectDefinition {
    pub fn contains_point(&self, point: macroquad::prelude::Vec2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.w
            && point.y >= self.y
            && point.y <= self.y + self.h
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JournalMilestoneEntry {
    pub id: String,
    pub title: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WarpDefinition {
    pub id: String,
    pub label: String,
    pub rect: RectDefinition,
    pub target_area: String,
    pub target_position: [f32; 2],
    #[serde(default)]
    pub required_total_brews: u32,
    #[serde(default)]
    pub required_coins: u32,
    #[serde(default)]
    pub required_item_id: String,
    #[serde(default)]
    pub required_item_amount: u32,
    #[serde(default)]
    pub required_journal_milestone: String,
    #[serde(default)]
    pub required_journal_hint: String,
    #[serde(default)]
    pub unlock_milestones: Vec<JournalMilestoneEntry>,
    #[serde(default)]
    pub locked_note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GatherNodeDefinition {
    pub id: String,
    pub name: String,
    pub item_id: String,
    pub color: [u8; 4],
    pub position: [f32; 2],
    pub radius: f32,
    #[serde(default)]
    pub route_id: String,
    #[serde(default)]
    pub seasons: Vec<String>,
    #[serde(default)]
    pub weathers: Vec<String>,
    #[serde(default)]
    pub time_windows: Vec<String>,
    #[serde(default = "default_spawn_chance")]
    pub spawn_chance: u32,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub render: GatherNodeRenderDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GatheringRouteDefinition {
    pub id: String,
    pub name: String,
    pub area_id: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpcDefinition {
    pub id: String,
    pub name: String,
    pub area_id: String,
    pub position: [f32; 2],
    pub interaction_radius: f32,
    pub color: [u8; 4],
    pub dialogue_start: String,
    pub dialogue_progress: String,
    pub dialogue_complete: String,
    #[serde(default)]
    pub quest_id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub schedule: Vec<NpcScheduleEntry>,
    #[serde(default)]
    pub phase1_dialogue: NpcPhase1DialogueDefinition,
    #[serde(default)]
    pub crow_phase1_dialogue: CrowPhase1DialogueDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpcScheduleEntry {
    pub time_window: String,
    pub area_id: String,
    pub position: [f32; 2],
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NpcPhase1DialogueDefinition {
    #[serde(default)]
    pub intro: String,
    #[serde(default)]
    pub pre_help_concern: String,
    #[serde(default)]
    pub active_request: String,
    #[serde(default)]
    pub post_help_relief: String,
    #[serde(default)]
    pub town_recovery_observation: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CrowPhase1DialogueDefinition {
    #[serde(default)]
    pub first_meeting: String,
    #[serde(default)]
    pub first_brew: String,
    #[serde(default)]
    pub first_quest_complete: String,
    #[serde(default)]
    pub first_tower_restoration: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub required_item_id: String,
    pub required_amount: u32,
    pub reward_coins: u32,
    pub giver_npc_id: String,
    #[serde(default)]
    pub minimum_quality_band: String,
    #[serde(default)]
    pub required_trait: String,
    #[serde(default)]
    pub required_traits: Vec<String>,
    #[serde(default)]
    pub minimum_trait_matches: u32,
    #[serde(default)]
    pub required_effect_kind: String,
    #[serde(default)]
    pub required_effect_kinds: Vec<String>,
    #[serde(default)]
    pub minimum_effect_matches: u32,
    #[serde(default)]
    pub prerequisite_quests: Vec<String>,
    #[serde(default)]
    pub required_unlocked_warp: String,
    #[serde(default)]
    pub minimum_total_brews: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationDefinition {
    pub id: String,
    pub name: String,
    pub kind: StationKind,
    pub area_id: String,
    pub position: [f32; 2],
    pub interaction_radius: f32,
    pub color: [u8; 4],
    #[serde(default)]
    pub stock: Vec<ShopStockDefinition>,
    #[serde(default)]
    pub room_bonus: RoomBonusDefinition,
    #[serde(default)]
    pub planter_harvest_days: u32,
    #[serde(default)]
    pub planter_seed_ids: Vec<String>,
    #[serde(default)]
    pub planter_yield_bonus: u32,
    #[serde(default)]
    pub required_completed_quest: String,
    #[serde(default)]
    pub required_total_brews: u32,
    #[serde(default)]
    pub required_journal_milestone: String,
    #[serde(default)]
    pub habitat_creature_ids: Vec<String>,
    #[serde(default)]
    pub habitat_output_item_id: String,
    #[serde(default)]
    pub habitat_harvest_days: u32,
    #[serde(default)]
    pub render: StationRenderDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShopStockDefinition {
    pub item_id: String,
    pub price: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AreaDefinition {
    pub id: String,
    pub name: String,
    pub size: [f32; 2],
    pub background: [u8; 4],
    pub accent: [u8; 4],
    pub blockers: Vec<RectDefinition>,
    pub warps: Vec<WarpDefinition>,
    pub gather_nodes: Vec<GatherNodeDefinition>,
    #[serde(default)]
    pub render: AreaRenderDefinition,
}

fn default_spawn_chance() -> u32 {
    100
}
