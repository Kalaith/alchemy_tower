//! Serializable game definitions shared across systems.

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    Ingredient,
    Catalyst,
    Potion,
    Rune,
    Creature,
}

impl ItemCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ingredient => "ingredient",
            Self::Catalyst => "catalyst",
            Self::Potion => "potion",
            Self::Rune => "rune",
            Self::Creature => "creature",
        }
    }
}

impl fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectKind {
    Glow,
    Speed,
    Misfire,
    Restore,
}

impl EffectKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Glow => "glow",
            Self::Speed => "speed",
            Self::Misfire => "misfire",
            Self::Restore => "restore",
        }
    }
}

impl fmt::Display for EffectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StationKind {
    Alchemy,
    RestBed,
    Shop,
    RuneWorkshop,
    ArchiveConsole,
    EndingFocus,
    QuestBoard,
    Planter,
    Habitat,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerVisualStyle {
    Shelf,
    House,
    #[default]
    Panel,
    Grass,
    Quarry,
    Forest,
    Reeds,
    Dunes,
    Rainforest,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AreaRenderDefinition {
    #[serde(default)]
    pub blocker_style: BlockerVisualStyle,
    #[serde(default)]
    pub blocker_primary: Option<[u8; 4]>,
    #[serde(default)]
    pub blocker_secondary: Option<[u8; 4]>,
    #[serde(default)]
    pub blocker_detail: Option<[u8; 4]>,
    #[serde(default)]
    pub blocker_alt: Option<[u8; 4]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationRenderDefinition {
    #[serde(default = "default_station_sprite_size")]
    pub sprite_size: [f32; 2],
    #[serde(default = "default_station_highlight_size_bonus")]
    pub highlight_size_bonus: [f32; 2],
    #[serde(default)]
    pub overlay_effect_id: String,
    #[serde(default = "default_zero_vec2")]
    pub overlay_effect_offset: [f32; 2],
    #[serde(default = "default_zero_vec2")]
    pub overlay_effect_size: [f32; 2],
}

impl Default for StationRenderDefinition {
    fn default() -> Self {
        Self {
            sprite_size: default_station_sprite_size(),
            highlight_size_bonus: default_station_highlight_size_bonus(),
            overlay_effect_id: String::new(),
            overlay_effect_offset: default_zero_vec2(),
            overlay_effect_size: default_zero_vec2(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GatherNodeRenderDefinition {
    #[serde(default = "default_gather_node_sprite_size")]
    pub sprite_size: [f32; 2],
    #[serde(default)]
    pub sprite_id: String,
}

impl Default for GatherNodeRenderDefinition {
    fn default() -> Self {
        Self {
            sprite_size: default_gather_node_sprite_size(),
            sprite_id: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ElementProfile {
    #[serde(default)]
    pub vita: i32,
    #[serde(default)]
    pub ember: i32,
    #[serde(default)]
    pub mist: i32,
    #[serde(default)]
    pub lux: i32,
}

impl ElementProfile {
    pub fn total(&self) -> i32 {
        self.vita + self.ember + self.mist + self.lux
    }

    pub fn add_assign(&mut self, other: &Self) {
        self.vita += other.vita;
        self.ember += other.ember;
        self.mist += other.mist;
        self.lux += other.lux;
    }

    pub fn meets(&self, required: &Self) -> bool {
        self.vita >= required.vita
            && self.ember >= required.ember
            && self.mist >= required.mist
            && self.lux >= required.lux
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WildVariantDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub required_conditions: Vec<String>,
    #[serde(default)]
    pub bonus_traits: Vec<String>,
    #[serde(default)]
    pub quality_bonus: u32,
    #[serde(default)]
    pub elements: ElementProfile,
    #[serde(default)]
    pub synthesis_weight_bonus: u32,
    #[serde(default)]
    pub synthesis_value_bonus: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MorphDefinition {
    pub output_item_id: String,
    #[serde(default)]
    pub minimum_quality: u32,
    #[serde(default)]
    pub catalyst_tag: String,
    #[serde(default = "default_heat")]
    pub required_heat: i32,
    #[serde(default)]
    pub required_stirs: u32,
    #[serde(default)]
    pub required_timing: String,
    #[serde(default)]
    pub required_sequence: Vec<String>,
    #[serde(default)]
    pub room_bonus_required: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoomBonusDefinition {
    #[serde(default)]
    pub quality_bonus: u32,
    #[serde(default)]
    pub favored_traits: Vec<String>,
    #[serde(default)]
    pub favored_categories: Vec<String>,
    #[serde(default)]
    pub description: String,
}

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
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub category: ItemCategory,
    pub base_value: u32,
    pub color: [u8; 4],
    pub description: String,
    #[serde(default = "default_item_quality")]
    pub quality: u32,
    #[serde(default = "default_item_rarity")]
    pub rarity: u8,
    #[serde(default)]
    pub elements: ElementProfile,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub source_conditions: Vec<String>,
    #[serde(default)]
    pub wild_variants: Vec<WildVariantDefinition>,
    #[serde(default = "default_synthesis_weight")]
    pub synthesis_weight: u32,
    #[serde(default)]
    pub synthesis_value: u32,
    #[serde(default)]
    pub catalyst_tags: Vec<String>,
    #[serde(default)]
    pub effects: Vec<EffectDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EffectDefinition {
    pub kind: EffectKind,
    pub magnitude: f32,
    pub duration_seconds: f32,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecipeIngredient {
    pub item_id: String,
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecipeDefinition {
    pub id: String,
    pub name: String,
    pub station_id: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub output_item_id: String,
    pub output_amount: u32,
    pub description: String,
    #[serde(default = "default_heat")]
    pub required_heat: i32,
    #[serde(default)]
    pub required_stirs: u32,
    #[serde(default = "default_unstable_output")]
    pub unstable_output_item_id: String,
    #[serde(default)]
    pub lore_note: String,
    #[serde(default)]
    pub minimum_quality: u32,
    #[serde(default)]
    pub preferred_traits: Vec<String>,
    #[serde(default)]
    pub guaranteed_traits: Vec<String>,
    #[serde(default)]
    pub minimum_elements: ElementProfile,
    #[serde(default)]
    pub catalyst_tag: String,
    #[serde(default)]
    pub catalyst_quality_bonus: u32,
    #[serde(default)]
    pub required_timing: String,
    #[serde(default)]
    pub required_sequence: Vec<String>,
    #[serde(default)]
    pub morph_targets: Vec<MorphDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuneRecipeDefinition {
    pub id: String,
    pub station_id: String,
    pub input_item_id: String,
    pub rune_item_id: String,
    pub output_item_id: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MutationFormulaDefinition {
    pub id: String,
    pub seed_item_id: String,
    #[serde(default)]
    pub required_effect_kind: String,
    #[serde(default)]
    pub yield_bonus: u32,
    #[serde(default)]
    pub growth_bonus_days: u32,
    #[serde(default)]
    pub mutation_note: String,
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
pub struct PlanterStateEntry {
    pub station_id: String,
    #[serde(default)]
    pub planted_item_id: String,
    #[serde(default)]
    pub planted_day: u32,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub tended_day: u32,
    #[serde(default)]
    pub growth_days: u32,
    #[serde(default)]
    pub mutation_formula_id: String,
    #[serde(default)]
    pub mutation_yield_bonus: u32,
    #[serde(default)]
    pub mutation_growth_bonus_days: u32,
    #[serde(default)]
    pub mutation_note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HabitatStateEntry {
    pub station_id: String,
    #[serde(default)]
    pub creature_item_id: String,
    #[serde(default)]
    pub placed_day: u32,
    #[serde(default)]
    pub last_harvest_day: u32,
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

fn default_heat() -> i32 {
    2
}

fn default_unstable_output() -> String {
    "murky_concoction".to_owned()
}

fn default_spawn_chance() -> u32 {
    100
}

fn default_item_quality() -> u32 {
    20
}

fn default_item_rarity() -> u8 {
    1
}

fn default_synthesis_weight() -> u32 {
    1
}

fn default_station_sprite_size() -> [f32; 2] {
    [96.0, 96.0]
}

fn default_station_highlight_size_bonus() -> [f32; 2] {
    [8.0, 8.0]
}

fn default_gather_node_sprite_size() -> [f32; 2] {
    [64.0, 64.0]
}

fn default_zero_vec2() -> [f32; 2] {
    [0.0, 0.0]
}
