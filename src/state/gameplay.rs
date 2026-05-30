//! Core exploration and alchemy state.

use std::collections::{BTreeMap, HashSet};

use macroquad::prelude::*;

use crate::alchemy::{quality_band, resolve_brew};
use crate::content::{input_bindings, narrative_text};
use crate::data::{
    AreaDefinition, CraftedItemProfileEntry, EffectDefinition, EffectKind, ExperimentLogEntry,
    FieldJournalEntry, GameData, HabitatStateEntry, HerbMemoryEntry, InventoryEntry, ItemCategory,
    JournalMilestoneEntry, NpcDefinition, PlanterStateEntry, PotionMemoryEntry, QuestDefinition,
    RecipeMasteryEntry, RelationshipEntry, SaveData, StationDefinition, StationKind,
    WarpDefinition,
};
use crate::save::SaveRepository;

#[path = "gameplay_alchemy_inventory.rs"]
mod gameplay_alchemy_inventory;
#[path = "gameplay_alchemy_preview.rs"]
mod gameplay_alchemy_preview;
#[path = "gameplay_alchemy_setup.rs"]
mod gameplay_alchemy_setup;
#[path = "gameplay_brew_records.rs"]
mod gameplay_brew_records;
#[path = "gameplay_gathering.rs"]
mod gameplay_gathering;
#[path = "gameplay_facilities.rs"]
mod gameplay_facilities;
#[path = "gameplay_habitats.rs"]
mod gameplay_habitats;
#[path = "gameplay_planters.rs"]
mod gameplay_planters;
#[path = "gameplay_warps.rs"]
mod gameplay_warps;
#[path = "gameplay_player.rs"]
mod gameplay_player;
#[path = "gameplay_feedback.rs"]
mod gameplay_feedback;
#[path = "gameplay_alchemy_input.rs"]
mod gameplay_alchemy_input;
#[path = "gameplay_archive_input.rs"]
mod gameplay_archive_input;
#[path = "gameplay_potion_input.rs"]
mod gameplay_potion_input;
#[path = "gameplay_rune_input.rs"]
mod gameplay_rune_input;
#[path = "gameplay_shop_input.rs"]
mod gameplay_shop_input;
#[path = "gameplay_init.rs"]
mod gameplay_init;
#[path = "gameplay_loop.rs"]
mod gameplay_loop;
#[path = "gameplay_camera.rs"]
mod gameplay_camera;
#[path = "gameplay_inventory.rs"]
mod gameplay_inventory;
#[path = "gameplay_inventory_memory.rs"]
mod gameplay_inventory_memory;
#[path = "gameplay_inventory_views.rs"]
mod gameplay_inventory_views;
#[path = "gameplay_inventory_transactions.rs"]
mod gameplay_inventory_transactions;
#[path = "gameplay_npc.rs"]
mod gameplay_npc;
#[path = "gameplay_npc_dialogue.rs"]
mod gameplay_npc_dialogue;
#[path = "gameplay_npc_motion.rs"]
mod gameplay_npc_motion;
#[path = "gameplay_npc_pathing.rs"]
mod gameplay_npc_pathing;
#[path = "gameplay_npc_routes.rs"]
mod gameplay_npc_routes;
#[path = "../ui/overlays.rs"]
mod gameplay_overlays;
#[path = "gameplay_persistence.rs"]
mod gameplay_persistence;
#[path = "gameplay_progression.rs"]
mod gameplay_progression;
#[path = "gameplay_disassembly.rs"]
mod gameplay_disassembly;
#[path = "gameplay_duplication.rs"]
mod gameplay_duplication;
#[path = "gameplay_planter_mutation.rs"]
mod gameplay_planter_mutation;
#[path = "gameplay_dialogue.rs"]
mod gameplay_dialogue;
#[path = "gameplay_quest_board.rs"]
mod gameplay_quest_board;
#[path = "gameplay_quests.rs"]
mod gameplay_quests;
#[path = "gameplay_quest_requirements.rs"]
mod gameplay_quest_requirements;
#[path = "gameplay_render.rs"]
mod gameplay_render;
#[path = "gameplay_render_markers.rs"]
mod gameplay_render_markers;
#[path = "gameplay_services.rs"]
mod gameplay_services;
#[path = "gameplay_effects.rs"]
mod gameplay_effects;
#[path = "gameplay_world.rs"]
mod gameplay_world;
#[path = "gameplay_world_labels.rs"]
mod gameplay_world_labels;
#[path = "gameplay_support.rs"]
mod gameplay_support;
#[path = "gameplay_tutorial.rs"]
mod gameplay_tutorial;
#[path = "gameplay_time.rs"]
mod gameplay_time;
#[path = "../ui/hud.rs"]
mod ui_hud;
#[path = "../ui/world_prompts.rs"]
mod ui_world_prompts;

use self::gameplay_support::{
    initial_journal_milestones, planter_stage_label, quality_band_rank, rgba, starting_day_time,
};

const PLAYER_RADIUS: f32 = 14.0;
const CAMERA_PADDING: f32 = 160.0;
const SLOT_COUNT: usize = 3;
const ALCHEMY_TIMINGS: [&str; 3] = ["steady", "early", "late"];
const ARCHIVE_TABS: [&str; 6] = [
    "timeline",
    "experiments",
    "mastery",
    "morphs",
    "disassembly",
    "duplication",
];

#[derive(Clone, Debug)]
struct PlayerAvatar {
    position: Vec2,
    facing: Vec2,
    moving: bool,
}

#[derive(Clone, Debug)]
struct ActiveEffect {
    kind: EffectKind,
    magnitude: f32,
    remaining_seconds: f32,
    description: String,
}

#[derive(Clone, Debug)]
struct GatherToast {
    remaining_seconds: f32,
}

#[derive(Clone, Debug)]
struct GatherFeedback {
    position: Vec2,
    remaining_seconds: f32,
    color: Color,
    emphasis: bool,
    burst_scale: f32,
}

#[derive(Clone, Debug, Default)]
struct FieldDiscoveryFeedback {
    new_note: bool,
    improved_quality: bool,
    variant_discovered: bool,
}

#[derive(Clone, Debug)]
struct NpcRuntimeState {
    area_id: String,
    position: Vec2,
    direction: Vec2,
    moving: bool,
    target_area_id: Option<String>,
}

#[derive(Clone, Debug)]
struct TravelSegment {
    area_id: String,
    start: Vec2,
    end: Vec2,
}

#[derive(Clone, Debug)]
struct NpcMotionTracker {
    area_id: String,
    position: Vec2,
    direction: Vec2,
    moving: bool,
    target_area_id: Option<String>,
    schedule_index: Option<usize>,
    target_schedule_index: Option<usize>,
    route_segments: Vec<TravelSegment>,
    route_segment_index: usize,
}

#[derive(Clone, Debug, Default)]
struct OverlayState {
    journal_tab: usize,
    shop_buy_tab: bool,
    shop_index: usize,
    rune_index: usize,
    archive_tab: usize,
    archive_index: usize,
    archive_experiment_filter: ArchiveExperimentFilter,
    current: Option<OverlayScreen>,
    inventory_sort_mode: InventorySortMode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum OverlayScreen {
    Journal,
    QuestBoard,
    Shop,
    Rune,
    Archive,
    Ending,
    Dialogue(String),
    Alchemy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InventorySortMode {
    Priority,
    Type,
    Name,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArchiveExperimentFilter {
    All,
    Stable,
    Unstable,
}

#[derive(Clone, Debug)]
struct AlchemySession {
    index: usize,
    heat: i32,
    stirs: u32,
    timing_index: usize,
    slots: [Option<String>; SLOT_COUNT],
    catalyst: Option<String>,
}

#[derive(Clone, Debug)]
struct SavedAlchemySetup {
    heat: i32,
    stirs: u32,
    timing_index: usize,
    slots: [Option<String>; SLOT_COUNT],
    catalyst: Option<String>,
}

impl Default for AlchemySession {
    fn default() -> Self {
        Self {
            index: 0,
            heat: 2,
            stirs: 0,
            timing_index: 0,
            slots: [None, None, None],
            catalyst: None,
        }
    }
}

impl InventorySortMode {
    fn next(self) -> Self {
        match self {
            Self::Priority => Self::Type,
            Self::Type => Self::Name,
            Self::Name => Self::Priority,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Priority => "priority",
            Self::Type => "type",
            Self::Name => "name",
        }
    }
}

impl Default for InventorySortMode {
    fn default() -> Self {
        Self::Priority
    }
}

impl Default for ArchiveExperimentFilter {
    fn default() -> Self {
        Self::All
    }
}

impl ArchiveExperimentFilter {
    fn next(self) -> Self {
        match self {
            Self::All => Self::Stable,
            Self::Stable => Self::Unstable,
            Self::Unstable => Self::All,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Stable => "stable",
            Self::Unstable => "unstable",
        }
    }
}

#[derive(Clone, Debug)]
struct WorldState {
    current_area_id: String,
    player: PlayerAvatar,
    day_index: u32,
    day_clock_seconds: f32,
    day_length_seconds: f32,
    gathered_nodes: HashSet<String>,
    available_nodes: HashSet<String>,
}

#[derive(Clone, Debug)]
struct ProgressionState {
    total_brews: u32,
    known_recipes: HashSet<String>,
    recipe_mastery: BTreeMap<String, u32>,
    crafted_item_profiles: BTreeMap<String, CraftedItemProfileEntry>,
    experiment_log: Vec<ExperimentLogEntry>,
    unlocked_warps: HashSet<String>,
    planter_states: BTreeMap<String, PlanterStateEntry>,
    habitat_states: BTreeMap<String, HabitatStateEntry>,
    journal_milestones: Vec<JournalMilestoneEntry>,
    relationships: BTreeMap<String, i32>,
    started_quests: HashSet<String>,
    completed_quests: HashSet<String>,
    herb_memories: BTreeMap<String, HerbMemoryEntry>,
    potion_memories: BTreeMap<String, PotionMemoryEntry>,
}

#[derive(Clone, Debug)]
struct RuntimeState {
    active_effects: Vec<ActiveEffect>,
    gather_toasts: Vec<GatherToast>,
    gather_feedbacks: Vec<GatherFeedback>,
    gather_pause_seconds: f32,
    camera_shake_seconds: f32,
    camera_shake_intensity: f32,
    sleep_flash_seconds: f32,
    npc_motion_states: BTreeMap<String, NpcMotionTracker>,
    status_text: String,
    last_brew_setup: Option<SavedAlchemySetup>,
    tutorial: TutorialState,
    footstep_cooldown_seconds: f32,
    area_banner_area_id: String,
    area_banner_label: String,
    area_banner_seconds: f32,
}

#[derive(Clone, Debug)]
struct TutorialState {
    next_hint_delay_seconds: f32,
    crow_intro_hint_shown: bool,
    save_hint_shown: bool,
    journal_hint_shown: bool,
    alchemy_hint_shown: bool,
    potion_hint_shown: bool,
    gather_hint_shown: bool,
    brew_goal_hint_shown: bool,
    mira_hint_shown: bool,
    rowan_hint_shown: bool,
    quest_hint_shown: bool,
    delivery_hint_shown: bool,
    route_hint_shown: bool,
}

#[derive(Clone, Debug)]
pub struct GameplayState {
    world: WorldState,
    progression: ProgressionState,
    coins: u32,
    vitality: f32,
    inventory: BTreeMap<String, u32>,
    runtime: RuntimeState,
    ui: OverlayState,
    alchemy: AlchemySession,
}

impl GameplayState {
    fn overlay(&self) -> Option<&OverlayScreen> {
        self.ui.current.as_ref()
    }

    fn set_overlay(&mut self, overlay: OverlayScreen) {
        self.ui.current = Some(overlay);
    }

    fn clear_overlay(&mut self) {
        self.ui.current = None;
    }

    fn dialogue_npc_id(&self) -> Option<&str> {
        match self.overlay() {
            Some(OverlayScreen::Dialogue(npc_id)) => Some(npc_id.as_str()),
            _ => None,
        }
    }
}
