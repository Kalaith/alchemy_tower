//! Serializable game definitions shared across systems.

use std::collections::HashMap;
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
    Shop,
    RuneWorkshop,
    ArchiveConsole,
    EndingFocus,
    QuestBoard,
    Planter,
    Habitat,
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpcScheduleEntry {
    pub time_window: String,
    pub area_id: String,
    pub position: [f32; 2],
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
pub struct JournalMilestoneEntry {
    pub id: String,
    pub title: String,
    pub text: String,
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameData {
    pub config: GameConfig,
    pub areas: Vec<AreaDefinition>,
    #[serde(default)]
    pub gathering_routes: Vec<GatheringRouteDefinition>,
    #[serde(default)]
    pub npcs: Vec<NpcDefinition>,
    #[serde(default)]
    pub quests: Vec<QuestDefinition>,
    pub items: Vec<ItemDefinition>,
    pub recipes: Vec<RecipeDefinition>,
    #[serde(default)]
    pub rune_recipes: Vec<RuneRecipeDefinition>,
    #[serde(default)]
    pub mutation_formulas: Vec<MutationFormulaDefinition>,
    pub stations: Vec<StationDefinition>,
    #[serde(skip)]
    area_index: HashMap<String, usize>,
    #[serde(skip)]
    item_index: HashMap<String, usize>,
    #[serde(skip)]
    route_index: HashMap<String, usize>,
    #[serde(skip)]
    npc_index: HashMap<String, usize>,
    #[serde(skip)]
    quest_index: HashMap<String, usize>,
    #[serde(skip)]
    mutation_formula_index: HashMap<String, Vec<usize>>,
}

impl GameData {
    pub fn build_indexes(&mut self) {
        self.area_index = self
            .areas
            .iter()
            .enumerate()
            .map(|(index, area)| (area.id.clone(), index))
            .collect();
        self.item_index = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| (item.id.clone(), index))
            .collect();
        self.route_index = self
            .gathering_routes
            .iter()
            .enumerate()
            .map(|(index, route)| (route.id.clone(), index))
            .collect();
        self.npc_index = self
            .npcs
            .iter()
            .enumerate()
            .map(|(index, npc)| (npc.id.clone(), index))
            .collect();
        self.quest_index = self
            .quests
            .iter()
            .enumerate()
            .map(|(index, quest)| (quest.id.clone(), index))
            .collect();
        let mut mutation_formula_index = HashMap::<String, Vec<usize>>::new();
        for (index, formula) in self.mutation_formulas.iter().enumerate() {
            mutation_formula_index
                .entry(formula.seed_item_id.clone())
                .or_default()
                .push(index);
        }
        self.mutation_formula_index = mutation_formula_index;
    }

    pub fn area(&self, area_id: &str) -> Option<&AreaDefinition> {
        self.area_index
            .get(area_id)
            .and_then(|index| self.areas.get(*index))
    }

    pub fn fallback() -> Self {
        let mut data: Self =
            serde_json::from_str(include_str!("../../assets/data/game_data.json"))
                .expect("embedded fallback game_data.json must remain valid");
        data.build_indexes();
        data
    }

    pub fn item(&self, item_id: &str) -> Option<&ItemDefinition> {
        self.item_index
            .get(item_id)
            .and_then(|index| self.items.get(*index))
    }

    pub fn item_name<'a>(&'a self, item_id: &'a str) -> &'a str {
        self.item(item_id)
            .map(|item| item.name.as_str())
            .unwrap_or(item_id)
    }

    pub fn route(&self, route_id: &str) -> Option<&GatheringRouteDefinition> {
        self.route_index
            .get(route_id)
            .and_then(|index| self.gathering_routes.get(*index))
    }

    pub fn npc(&self, npc_id: &str) -> Option<&NpcDefinition> {
        self.npc_index
            .get(npc_id)
            .and_then(|index| self.npcs.get(*index))
    }

    pub fn quest(&self, quest_id: &str) -> Option<&QuestDefinition> {
        self.quest_index
            .get(quest_id)
            .and_then(|index| self.quests.get(*index))
    }

    #[cfg(test)]
    pub fn recipe_for_output(&self, item_id: &str) -> Option<&RecipeDefinition> {
        self.recipes
            .iter()
            .find(|recipe| recipe.output_item_id == item_id)
    }

    pub fn mutation_formulas_for_seed(
        &self,
        seed_item_id: &str,
    ) -> Vec<&MutationFormulaDefinition> {
        self.mutation_formula_index
            .get(seed_item_id)
            .into_iter()
            .flatten()
            .filter_map(|index| self.mutation_formulas.get(*index))
            .collect()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InventoryEntry {
    pub item_id: String,
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecipeMasteryEntry {
    pub recipe_id: String,
    pub successful_brews: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CraftedItemProfileEntry {
    pub item_id: String,
    #[serde(default)]
    pub best_quality_score: u32,
    #[serde(default)]
    pub best_quality_band: String,
    #[serde(default)]
    pub inherited_traits: Vec<String>,
    #[serde(default)]
    pub effect_kinds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExperimentLogEntry {
    #[serde(default)]
    pub recipe_id: String,
    pub output_item_id: String,
    #[serde(default)]
    pub quality_score: u32,
    #[serde(default)]
    pub quality_band: String,
    #[serde(default)]
    pub stable: bool,
    #[serde(default)]
    pub catalyst_item_id: String,
    #[serde(default)]
    pub morph_output_item_id: String,
    #[serde(default)]
    pub day_index: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaveData {
    pub version: u32,
    pub current_area: String,
    pub player_position: [f32; 2],
    pub day_clock_seconds: f32,
    #[serde(default = "default_vitality")]
    pub vitality: f32,
    #[serde(default)]
    pub coins: u32,
    pub inventory: Vec<InventoryEntry>,
    pub gathered_nodes: Vec<String>,
    #[serde(default)]
    pub known_recipes: Vec<String>,
    #[serde(default)]
    pub day_index: u32,
    #[serde(default)]
    pub field_journal: Vec<FieldJournalEntry>,
    #[serde(default)]
    pub started_quests: Vec<String>,
    #[serde(default)]
    pub completed_quests: Vec<String>,
    #[serde(default)]
    pub recipe_mastery: Vec<RecipeMasteryEntry>,
    #[serde(default)]
    pub crafted_item_profiles: Vec<CraftedItemProfileEntry>,
    #[serde(default)]
    pub experiment_log: Vec<ExperimentLogEntry>,
    #[serde(default)]
    pub total_brews: u32,
    #[serde(default)]
    pub unlocked_warps: Vec<String>,
    #[serde(default)]
    pub planter_states: Vec<PlanterStateEntry>,
    #[serde(default)]
    pub journal_milestones: Vec<JournalMilestoneEntry>,
    #[serde(default)]
    pub relationships: Vec<RelationshipEntry>,
    #[serde(default)]
    pub habitat_states: Vec<HabitatStateEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationshipEntry {
    pub npc_id: String,
    pub value: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldJournalEntry {
    pub item_id: String,
    pub route_id: String,
    pub season: String,
    pub weather: String,
    pub time_window: String,
    pub note: String,
    #[serde(default)]
    pub best_quality: u32,
    #[serde(default)]
    pub best_quality_band: String,
    #[serde(default)]
    pub variant_name: String,
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

fn default_vitality() -> f32 {
    100.0
}
