//! Core exploration and alchemy state.

use std::collections::{BTreeMap, HashSet};

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use crate::alchemy::{quality_band, resolve_brew};
use crate::art::{
    draw_blocker_prop, draw_character_frame, draw_gather_node_marker, draw_priority_marker,
    draw_station_marker, draw_texture_centered, ArtAssets,
};
use crate::audio::AudioAssets;
use crate::content::{input_bindings, narrative_text, ui_copy, ui_format, ui_text};
use crate::data::{
    AreaDefinition, CraftedItemProfileEntry, EffectDefinition, EffectKind, ExperimentLogEntry,
    FieldJournalEntry, GameData, HabitatStateEntry, InventoryEntry, ItemCategory,
    JournalMilestoneEntry, NpcDefinition, PlanterStateEntry, QuestDefinition,
    RecipeMasteryEntry, RelationshipEntry, SaveData, StationDefinition, StationKind,
    WarpDefinition,
};
use crate::save::SaveRepository;
use crate::state::StateTransition;
use crate::ui::draw_panel;

#[path = "gameplay_gathering.rs"]
mod gameplay_gathering;
#[path = "gameplay_input.rs"]
mod gameplay_input;
#[path = "gameplay_inventory.rs"]
mod gameplay_inventory;
#[path = "gameplay_npc.rs"]
mod gameplay_npc;
#[path = "../ui/overlays.rs"]
mod gameplay_overlays;
#[path = "gameplay_persistence.rs"]
mod gameplay_persistence;
#[path = "gameplay_progression.rs"]
mod gameplay_progression;
#[path = "gameplay_quests.rs"]
mod gameplay_quests;
#[path = "gameplay_support.rs"]
mod gameplay_support;
#[path = "../ui/hud.rs"]
mod ui_hud;
#[path = "../ui/world_prompts.rs"]
mod ui_world_prompts;

use self::gameplay_support::{
    clock_text, effect_name, initial_journal_milestones, planter_stage_label, quality_band_rank,
    rgba, starting_day_time,
};

const PLAYER_RADIUS: f32 = 14.0;
const CAMERA_PADDING: f32 = 160.0;
const SLOT_COUNT: usize = 3;
const ALCHEMY_TIMINGS: [&str; 3] = ["steady", "early", "late"];
const ARCHIVE_TABS: [&str; 6] = [
    "Timeline",
    "Experiments",
    "Mastery",
    "Morphs",
    "Disassembly",
    "Duplication",
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
    journal_open: bool,
    journal_tab: usize,
    quest_board_open: bool,
    shop_open: bool,
    shop_buy_tab: bool,
    shop_index: usize,
    rune_open: bool,
    rune_index: usize,
    archive_open: bool,
    archive_tab: usize,
    archive_index: usize,
    archive_experiment_filter: ArchiveExperimentFilter,
    ending_open: bool,
    dialogue_open: bool,
    current_npc_id: Option<String>,
    inventory_sort_mode: InventorySortMode,
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
    open: bool,
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
            open: false,
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
    field_journal: BTreeMap<String, FieldJournalEntry>,
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
                field_journal: BTreeMap::new(),
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
                status_text: "Gather ingredients and experiment at the tower cauldron.".to_owned(),
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

    pub fn update(&mut self, data: &GameData, audio: &AudioAssets) -> Option<StateTransition> {
        if is_key_pressed(KeyCode::Escape) {
            if self.alchemy.open {
                self.alchemy.open = false;
                self.runtime.status_text = ui_text().statuses.closed_alchemy.clone();
                return None;
            }
            if self.ui.shop_open {
                self.ui.shop_open = false;
                self.runtime.status_text = ui_text().statuses.closed_shop.clone();
                return None;
            }
            if self.ui.rune_open {
                self.ui.rune_open = false;
                self.runtime.status_text = ui_text().statuses.closed_rune.clone();
                return None;
            }
            if self.ui.archive_open {
                self.ui.archive_open = false;
                self.runtime.status_text = ui_text().statuses.closed_archive.clone();
                return None;
            }
            if self.ui.ending_open {
                self.ui.ending_open = false;
                self.runtime.status_text = ui_format("gameplay_observatory_back", &[]);
                return None;
            }
            if self.ui.dialogue_open {
                self.ui.dialogue_open = false;
                self.ui.current_npc_id = None;
                self.runtime.status_text = ui_format("gameplay_conversation_ended", &[]);
                return None;
            }
            if self.ui.journal_open {
                self.ui.journal_open = false;
                self.runtime.status_text = ui_text().statuses.closed_journal.clone();
                return None;
            }
            if self.ui.quest_board_open {
                self.ui.quest_board_open = false;
                self.runtime.status_text = ui_text().statuses.closed_quest_board.clone();
                return None;
            }
            return Some(StateTransition::Pause);
        }

        let frame_time = get_frame_time();
        self.world.day_clock_seconds += frame_time;
        while self.world.day_clock_seconds >= data.config.day_length_seconds {
            self.world.day_clock_seconds -= data.config.day_length_seconds;
            self.advance_to_next_day(data, true);
        }
        self.handle_sleep_pressure(data);
        self.update_active_effects(frame_time);
        self.update_gather_feedback(frame_time);
        self.update_npc_motion(data, frame_time);
        self.update_tutorial_hints(data, frame_time);

        if self.ui.dialogue_open {
            self.handle_dialogue_inputs(data);
        } else if self.ui.shop_open {
            self.handle_shop_inputs(data);
        } else if self.ui.rune_open {
            self.handle_rune_inputs(data);
        } else if self.ui.archive_open {
            self.handle_archive_inputs(data);
        } else if self.ui.ending_open {
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) {
                self.ui.ending_open = false;
            }
        } else if self.ui.quest_board_open {
            self.handle_quest_board_inputs(data);
        } else if self.ui.journal_open {
            let journal_tab_count = self.journal_tabs().len();
            self.ui.journal_tab = self.ui.journal_tab.min(journal_tab_count.saturating_sub(1));
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse = mouse_position().into();
                if self.journal_close_rect().contains(mouse) {
                    self.ui.journal_open = false;
                    self.runtime.status_text = ui_text().statuses.closed_journal.clone();
                    return None;
                }
                for index in 0..journal_tab_count {
                    if self
                        .journal_tab_rect(index, journal_tab_count)
                        .contains(mouse)
                    {
                        self.ui.journal_tab = index;
                        break;
                    }
                }
            }
            if is_key_pressed(KeyCode::Left) {
                self.ui.journal_tab = self.ui.journal_tab.saturating_sub(1);
            }
            if is_key_pressed(KeyCode::Right) {
                self.ui.journal_tab =
                    (self.ui.journal_tab + 1).min(journal_tab_count.saturating_sub(1));
            }
            if is_key_pressed(KeyCode::J) {
                self.ui.journal_open = false;
                self.runtime.status_text = ui_text().statuses.closed_journal.clone();
            }
        } else if self.alchemy.open {
            self.handle_alchemy_inputs(data, audio);
        } else {
            if is_key_pressed(KeyCode::J) {
                self.ui.journal_open = true;
                self.ui.journal_tab = 0;
                self.runtime.status_text = ui_text().statuses.open_journal.clone();
            }
            if is_key_pressed(KeyCode::V) {
                self.cycle_inventory_sort_mode();
            }
            if self.runtime.gather_pause_seconds <= 0.0 {
                self.update_movement(data, frame_time);
                self.update_footstep_audio(audio, frame_time);
                self.handle_potion_inputs(data);
                self.handle_interactions(data, audio);
            }
        }

        if is_key_pressed(KeyCode::F5) {
            self.save_progress(data);
        }
        if is_key_pressed(KeyCode::F9) {
            self.load_progress(data);
        }

        None
    }

    pub fn draw(&self, data: &GameData, art: &ArtAssets) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            draw_text(ui_copy("gameplay_missing_area"), 40.0, 80.0, 32.0, RED);
            return;
        };

        let offset = self.camera_offset(area);
        self.draw_area(area, offset, data, art);
        self.draw_player(offset, art);
        self.draw_hud(area, data, art);
        if self.ui.dialogue_open {
            self.draw_dialogue_overlay(data);
        } else if self.ui.shop_open {
            self.draw_shop_overlay(data);
        } else if self.ui.rune_open {
            self.draw_rune_overlay(data);
        } else if self.ui.archive_open {
            self.draw_archive_overlay(data);
        } else if self.ui.ending_open {
            self.draw_ending_overlay();
        } else if self.ui.quest_board_open {
            self.draw_quest_board_overlay(data);
        } else if self.ui.journal_open {
            self.draw_field_journal(data, art);
        } else if !self.alchemy.open {
            self.draw_prompt(area, offset, data);
        } else {
            self.draw_alchemy_overlay(data, art);
        }
        self.draw_sleep_flash_overlay();
    }

    fn update_active_effects(&mut self, frame_time: f32) {
        for effect in &mut self.runtime.active_effects {
            effect.remaining_seconds -= frame_time;
        }
        self.runtime.active_effects
            .retain(|effect| effect.remaining_seconds > 0.0);
    }

    fn update_gather_feedback(&mut self, frame_time: f32) {
        self.runtime.gather_pause_seconds = (self.runtime.gather_pause_seconds - frame_time).max(0.0);
        self.runtime.camera_shake_seconds = (self.runtime.camera_shake_seconds - frame_time).max(0.0);
        self.runtime.sleep_flash_seconds = (self.runtime.sleep_flash_seconds - frame_time).max(0.0);
        self.runtime.footstep_cooldown_seconds =
            (self.runtime.footstep_cooldown_seconds - frame_time).max(0.0);
        if self.runtime.camera_shake_seconds <= 0.0 {
            self.runtime.camera_shake_intensity = 0.0;
        }
        for toast in &mut self.runtime.gather_toasts {
            toast.remaining_seconds -= frame_time;
        }
        self.runtime.gather_toasts
            .retain(|toast| toast.remaining_seconds > 0.0);
        for feedback in &mut self.runtime.gather_feedbacks {
            feedback.remaining_seconds -= frame_time;
        }
        self.runtime.gather_feedbacks
            .retain(|feedback| feedback.remaining_seconds > 0.0);
    }

    fn update_movement(&mut self, data: &GameData, frame_time: f32) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };
        let mut direction = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }
        if direction.length_squared() == 0.0 {
            self.world.player.moving = false;
            return;
        }

        let normalized = direction.normalize();
        let speed = data.config.move_speed * self.move_speed_multiplier();
        let previous = self.world.player.position;
        let candidate = self.world.player.position + normalized * speed * frame_time;
        self.world.player.position = self.resolve_area_collision(area, candidate);
        self.world.player.facing = normalized;
        self.world.player.moving = self.world.player.position.distance_squared(previous) > 0.01;
    }

    fn update_footstep_audio(&mut self, audio: &AudioAssets, frame_time: f32) {
        if !self.world.player.moving {
            self.runtime.footstep_cooldown_seconds = self.runtime.footstep_cooldown_seconds.min(0.06);
            return;
        }
        let step_interval = (0.34 / self.move_speed_multiplier()).clamp(0.16, 0.34);
        if self.runtime.footstep_cooldown_seconds > 0.0 {
            self.runtime.footstep_cooldown_seconds =
                (self.runtime.footstep_cooldown_seconds - frame_time).max(0.0);
            return;
        }
        audio.play_footstep_for_area(&self.world.current_area_id);
        self.runtime.footstep_cooldown_seconds = step_interval;
    }

    fn handle_interactions(&mut self, data: &GameData, audio: &AudioAssets) {
        if let Some(station) = self.nearby_station(data) {
            if station.kind == StationKind::Alchemy
                && (is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::E))
            {
                self.alchemy.open = true;
                self.alchemy.index = 0;
                self.runtime.status_text = ui_text().statuses.open_alchemy.clone();
                audio.play_alchemy_open();
                return;
            }
        }

        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };
        if !is_key_pressed(KeyCode::E) {
            return;
        }

        if let Some(npc) = self.nearby_npc(data) {
            self.ui.dialogue_open = true;
            self.ui.current_npc_id = Some(npc.id.clone());
            self.runtime.status_text = ui_format("gameplay_talking_to", &[("name", &npc.name)]);
            return;
        }

        if let Some(station) = self.nearby_station(data) {
            if station.kind == StationKind::Shop {
                self.ui.shop_open = true;
                self.ui.shop_buy_tab = true;
                self.ui.shop_index = 0;
                self.runtime.status_text = ui_format("gameplay_opened_station", &[("name", &station.name)]);
                return;
            }
            if station.kind == StationKind::RuneWorkshop {
                self.ui.rune_open = true;
                self.ui.rune_index = 0;
                self.runtime.status_text = ui_format("gameplay_opened_station", &[("name", &station.name)]);
                return;
            }
            if station.kind == StationKind::ArchiveConsole {
                self.ui.archive_open = true;
                self.ui.archive_tab = 0;
                self.ui.archive_index = 0;
                self.runtime.status_text = ui_format("gameplay_opened_station", &[("name", &station.name)]);
                return;
            }
            if station.kind == StationKind::EndingFocus {
                if self.has_journal_milestone("archive_revelation") {
                    self.ui.ending_open = true;
                    self.push_journal_milestone(
                        "observatory_ending",
                        "Observatory Ending",
                        "At the tower's highest lens, the missing wizard's final path resolved into a choice to leave the tower alive rather than master it forever.",
                    );
                    self.runtime.status_text = ui_format("gameplay_observatory_aligned", &[]);
                } else {
                    self.runtime.status_text =
                        "The observatory remains dark. The archives are not complete.".to_owned();
                }
                return;
            }
            if station.kind == StationKind::QuestBoard {
                self.ui.quest_board_open = true;
                self.ui.shop_index = 0;
                self.runtime.status_text = ui_text().statuses.reading_quest_board.clone();
                return;
            }
            if station.kind == StationKind::RestBed {
                self.sleep_until(data, 7.0 * 60.0, false);
                return;
            }
            if station.kind == StationKind::Planter {
                self.interact_with_planter(data, station);
                return;
            }
            if station.kind == StationKind::Habitat {
                self.interact_with_habitat(data, station);
                return;
            }
        }

        if let Some(warp) = area
            .warps
            .iter()
            .find(|warp| warp.rect.contains_point(self.world.player.position))
        {
            if !self.warp_is_unlocked(warp) {
                if self.can_unlock_warp(warp) {
                    self.pay_warp_costs(warp);
                    self.progression.unlocked_warps.insert(warp.id.clone());
                    for milestone in &warp.unlock_milestones {
                        self.push_journal_milestone(
                            &milestone.id,
                            &milestone.title,
                            &milestone.text,
                        );
                    }
                    self.push_event_toast_with_icon(
                        ui_format("gameplay_route_restored", &[("label", &warp.label)]),
                        Color::from_rgba(188, 255, 220, 255),
                        "route_restored",
                    );
                    self.trigger_world_feedback(
                        vec2(
                            warp.rect.x + warp.rect.w * 0.5,
                            warp.rect.y + warp.rect.h * 0.5,
                        ),
                        Color::from_rgba(188, 255, 220, 255),
                        true,
                        2.0,
                    );
                    self.trigger_camera_shake(0.18, 4.8);
                    self.runtime.status_text = ui_format("gameplay_repaired_access", &[("label", &warp.label)]);
                } else {
                    self.runtime.status_text = self.warp_lock_text(data, warp);
                    return;
                }
            }
            self.world.current_area_id = warp.target_area.clone();
            self.world.player.position = vec2(warp.target_position[0], warp.target_position[1]);
            self.world.player.moving = false;
            self.refresh_available_nodes(data);
            self.runtime.status_text = ui_format("gameplay_entered", &[("label", &warp.label)]);
            self.trigger_world_feedback(
                self.world.player.position,
                Color::from_rgba(255, 245, 160, 255),
                false,
                1.2,
            );
            return;
        }

        if let Some(node) = area.gather_nodes.iter().find(|node| {
            !self.world.gathered_nodes.contains(&node.id)
                && self
                    .world
                    .player
                    .position
                    .distance(vec2(node.position[0], node.position[1]))
                    <= node.radius + data.config.interaction_range
        }) {
            if self.node_is_available(node) {
                self.world.gathered_nodes.insert(node.id.clone());
                *self.inventory.entry(node.item_id.clone()).or_insert(0) += 1;
                let discovery = self.record_field_discovery(data, node);
                self.trigger_gather_feedback(data, node, &discovery);
                self.runtime.status_text = self.gather_status_text(data, node, &discovery);
                audio.play_gather_pickup();
            } else {
                self.runtime.status_text = self.gather_attempt_status(data, node);
            }
        }
    }

    pub fn save_progress(&mut self, data: &GameData) {
        self.runtime.status_text = match gameplay_persistence::GameplayStateLoader::save_slot(self, data) {
            Ok(()) => ui_format("gameplay_saved_progress", &[]),
            Err(error) => ui_format("gameplay_save_failed", &[("error", &error)]),
        };
    }

    pub fn load_progress(&mut self, data: &GameData) {
        self.runtime.status_text = match gameplay_persistence::GameplayStateLoader::load_slot(self, data) {
            Ok(()) => ui_format("gameplay_loaded_progress", &[]),
            Err(error) => ui_format("gameplay_load_failed", &[("error", &error)]),
        };
    }

    pub fn pause_status_text(&self) -> &str {
        &self.runtime.status_text
    }

    fn can_reconstruct_archive(&self) -> bool {
        self.progression.completed_quests.contains("star_elixir_for_ione")
            && self.progression.completed_quests.contains("containment_for_lyra")
            && self.has_journal_milestone("greenhouse_repaired")
            && self.has_journal_milestone("containment_repaired")
            && self.has_journal_milestone("rune_workshop_restored")
    }

    fn available_rune_recipes<'a>(
        &self,
        data: &'a GameData,
        station: &StationDefinition,
    ) -> Vec<&'a crate::data::RuneRecipeDefinition> {
        data.rune_recipes
            .iter()
            .filter(|recipe| recipe.station_id == station.id)
            .filter(|recipe| {
                self.inventory
                    .get(&recipe.input_item_id)
                    .copied()
                    .unwrap_or_default()
                    > 0
                    && self
                        .inventory
                        .get(&recipe.rune_item_id)
                        .copied()
                        .unwrap_or_default()
                        > 0
            })
            .collect()
    }

    fn apply_rune_recipe(&mut self, data: &GameData, recipe: &crate::data::RuneRecipeDefinition) {
        for item_id in [&recipe.input_item_id, &recipe.rune_item_id] {
            if let Some(amount) = self.inventory.get_mut(item_id) {
                *amount = amount.saturating_sub(1);
            }
        }
        self.inventory.retain(|_, amount| *amount > 0);
        *self
            .inventory
            .entry(recipe.output_item_id.clone())
            .or_insert(0) += 1;
        self.push_journal_milestone(
            "first_rune_imbuing",
            "First Rune Imbuing",
            "The workshop accepted a finished potion and changed its behavior after the fact. The tower's later methods were more modular than the entry lab ever suggested.",
        );
        self.runtime.status_text = format!(
            "Imbued {} with {} to create {}.",
            data.item_name(&recipe.input_item_id),
            data.item_name(&recipe.rune_item_id),
            data.item_name(&recipe.output_item_id)
        );
    }

    fn draw_area(&self, area: &AreaDefinition, offset: Vec2, data: &GameData, art: &ArtAssets) {
        if let Some(texture) = art.background(&area.id) {
            draw_texture_ex(
                texture,
                offset.x,
                offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(area.size[0], area.size[1])),
                    ..Default::default()
                },
            );
        } else {
            draw_rectangle(offset.x, offset.y, area.size[0], area.size[1], rgba(area.background));
        }
        self.draw_environment_overlay(area, offset);
        self.draw_phase1_story_flourishes(area, offset);
        for (index, blocker) in area.blockers.iter().enumerate() {
            draw_blocker_prop(area, blocker, index, offset);
        }
        for warp in &area.warps {
            let center = vec2(
                offset.x + warp.rect.x + warp.rect.w * 0.5,
                offset.y + warp.rect.y + warp.rect.h * 0.5,
            );
            let unlock_ready = !self.warp_is_unlocked(warp) && self.can_unlock_warp(warp);
            if unlock_ready {
                let pulse = ((get_time() as f32 * 3.0) + warp.rect.x * 0.01).sin() * 0.5 + 0.5;
                if let Some(texture) = art.effect("warp_glow_effect") {
                    draw_texture_centered(
                        texture,
                        center,
                        vec2(74.0 + pulse * 12.0, 74.0 + pulse * 12.0),
                        Color::new(1.0, 1.0, 1.0, 0.55 + pulse * 0.2),
                    );
                }
                draw_rectangle(
                    offset.x + warp.rect.x,
                    offset.y + warp.rect.y,
                    warp.rect.w,
                    warp.rect.h,
                    Color::new(188.0 / 255.0, 255.0 / 255.0, 220.0 / 255.0, 0.10 + pulse * 0.08),
                );
                draw_circle_lines(
                    center.x,
                    center.y,
                    20.0 + pulse * 8.0,
                    2.0,
                    Color::from_rgba(188, 255, 220, 220),
                );
            }
            draw_rectangle_lines(
                offset.x + warp.rect.x,
                offset.y + warp.rect.y,
                warp.rect.w,
                warp.rect.h,
                3.0,
                if unlock_ready {
                    Color::from_rgba(188, 255, 220, 255)
                } else {
                    Color::from_rgba(255, 245, 160, 255)
                },
            );
        }
        for station in self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.area_id == area.id)
        {
            let center = vec2(offset.x + station.position[0], offset.y + station.position[1]);
            let player_distance = self
                .world
                .player
                .position
                .distance(vec2(station.position[0], station.position[1]));
            let nearby = player_distance <= station.interaction_radius + 60.0;
            let priority = self.station_world_label(data, station);
            draw_station_marker(station, center, priority.is_some(), art);
            if nearby || priority.is_some() {
                draw_text(
                    &station.name,
                    center.x - 42.0,
                    center.y + 46.0,
                    18.0,
                    dark::TEXT_BRIGHT,
                );
            }
            if let Some((label, color)) = priority {
                draw_text(&label, center.x - 42.0, center.y - 34.0, 18.0, color);
            }
        }
        for npc in self.visible_npcs(data) {
            let runtime = self.npc_runtime_state(data, npc);
            if runtime.area_id != area.id {
                continue;
            }
            let pos = self.npc_draw_position(npc, &runtime);
            let center = vec2(offset.x + pos.x, offset.y + pos.y);
            let priority = self.npc_world_label(data, npc);
            if let Some(texture) = art.character(&npc.id) {
                let facing = if runtime.direction.length_squared() > 0.0 {
                    runtime.direction
                } else {
                    vec2(0.0, 1.0)
                };
                draw_character_frame(texture, center, facing, runtime.moving, 1.0);
            } else {
                draw_circle(center.x, center.y, 18.0, rgba(npc.color));
            }
            if priority.is_some()
                || self.world.player.position.distance(pos) <= npc.interaction_radius + 54.0
            {
                draw_text(&npc.name, center.x - 34.0, center.y - 28.0, 18.0, dark::TEXT_BRIGHT);
            }
            if let Some((label, color)) = priority {
                draw_priority_marker(center, color);
                draw_text(&label, center.x - 34.0, center.y - 50.0, 18.0, color);
            }
        }
        for node in &area.gather_nodes {
            if self.world.gathered_nodes.contains(&node.id) {
                continue;
            }
            let available = self.node_is_available(node);
            if !available {
                continue;
            }
            let color = rgba(node.color);
            let center = vec2(offset.x + node.position[0], offset.y + node.position[1]);
            draw_gather_node_marker(
                node,
                data.item(&node.item_id).map(|item| item.category),
                center,
                color,
                available,
                art,
            );
        }
    }

    fn draw_environment_overlay(&self, area: &AreaDefinition, offset: Vec2) {
        let time_tint = match self.current_time_window() {
            "morning" => Color::from_rgba(255, 220, 170, 24),
            "day" => Color::from_rgba(255, 255, 255, 0),
            "evening" => Color::from_rgba(255, 184, 120, 38),
            _ => Color::from_rgba(72, 92, 150, 72),
        };
        if time_tint.a > 0.0 {
            draw_rectangle(offset.x, offset.y, area.size[0], area.size[1], time_tint);
        }

        match self.current_weather() {
            "mist" => {
                draw_rectangle(
                    offset.x,
                    offset.y,
                    area.size[0],
                    area.size[1],
                    Color::from_rgba(220, 228, 240, 28),
                );
                for index in 0..10 {
                    let drift = ((get_time() as f32 * 0.4) + index as f32 * 0.6).sin() * 18.0;
                    let x = offset.x + 80.0 + index as f32 * 110.0 + drift;
                    let y = offset.y + 60.0 + (index % 4) as f32 * 120.0;
                    draw_circle(x, y, 42.0 + (index % 3) as f32 * 12.0, Color::from_rgba(240, 244, 248, 20));
                }
            }
            "rain" => {
                draw_rectangle(
                    offset.x,
                    offset.y,
                    area.size[0],
                    area.size[1],
                    Color::from_rgba(90, 126, 168, 26),
                );
                for index in 0..28 {
                    let wave = ((get_time() as f32 * 2.8) + index as f32 * 0.4).fract();
                    let x = offset.x + (index as f32 * 48.0).rem_euclid(area.size[0]);
                    let y = offset.y + wave * area.size[1];
                    draw_line(x, y, x - 8.0, y + 16.0, 2.0, Color::from_rgba(200, 224, 255, 120));
                }
            }
            "windy" => {
                for index in 0..16 {
                    let wave = ((get_time() as f32 * 1.4) + index as f32 * 0.33).fract();
                    let x = offset.x + wave * area.size[0];
                    let y = offset.y + 30.0 + index as f32 * 34.0;
                    draw_line(x - 10.0, y, x + 22.0, y - 6.0, 2.0, Color::from_rgba(232, 232, 210, 64));
                }
            }
            _ => {}
        }
    }

    fn draw_phase1_story_flourishes(&self, area: &AreaDefinition, offset: Vec2) {
        match area.id.as_str() {
            "town_square" => {
                if self.progression.completed_quests.contains("healing_for_mira") {
                    let shelf = Color::from_rgba(122, 88, 66, 255);
                    let bottle = Color::from_rgba(176, 226, 255, 255);
                    draw_rectangle(offset.x + 684.0, offset.y + 670.0, 72.0, 18.0, shelf);
                    draw_rectangle(offset.x + 694.0, offset.y + 652.0, 10.0, 18.0, bottle);
                    draw_rectangle(offset.x + 714.0, offset.y + 646.0, 12.0, 24.0, Color::from_rgba(255, 214, 132, 255));
                    draw_rectangle(offset.x + 736.0, offset.y + 654.0, 10.0, 16.0, bottle);
                }
                if self.progression.completed_quests.contains("glow_for_rowan") {
                    for (x, y) in [(536.0, 540.0), (610.0, 470.0), (696.0, 404.0)] {
                        let pulse = ((get_time() as f32 * 2.2) + x * 0.01).sin() * 0.5 + 0.5;
                        draw_circle(
                            offset.x + x,
                            offset.y + y,
                            10.0 + pulse * 2.5,
                            Color::from_rgba(255, 228, 150, 120),
                        );
                        draw_circle(
                            offset.x + x,
                            offset.y + y,
                            5.0,
                            Color::from_rgba(255, 244, 188, 255),
                        );
                    }
                }
                if self.has_journal_milestone("greenhouse_repaired")
                    || self.progression.completed_quests.contains("cultivation_for_brin")
                {
                    for (x, y, color) in [
                        (598.0, 760.0, Color::from_rgba(126, 220, 158, 255)),
                        (640.0, 744.0, Color::from_rgba(239, 205, 90, 255)),
                        (676.0, 764.0, Color::from_rgba(188, 255, 220, 255)),
                    ] {
                        draw_circle(offset.x + x, offset.y + y, 8.0, color);
                        draw_line(
                            offset.x + x,
                            offset.y + y + 8.0,
                            offset.x + x,
                            offset.y + y + 18.0,
                            2.0,
                            Color::from_rgba(88, 152, 102, 255),
                        );
                    }
                }
            }
            "greenhouse_floor" => {
                if self.progression.completed_quests.contains("cultivation_for_brin") {
                    for (x, y) in [(690.0, 190.0), (742.0, 174.0), (794.0, 190.0)] {
                        draw_circle(
                            offset.x + x,
                            offset.y + y,
                            10.0,
                            Color::from_rgba(126, 220, 158, 255),
                        );
                        draw_circle(
                            offset.x + x + 10.0,
                            offset.y + y - 4.0,
                            7.0,
                            Color::from_rgba(239, 205, 90, 255),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_player(&self, offset: Vec2, art: &ArtAssets) {
        let center = offset + self.world.player.position;
        if self.effect_active(EffectKind::Glow) {
            draw_circle(
                center.x,
                center.y,
                PLAYER_RADIUS + 18.0,
                Color::from_rgba(215, 202, 255, 70),
            );
        }
        if let Some(texture) = art.player() {
            draw_character_frame(texture, center, self.world.player.facing, self.world.player.moving, 1.0);
        } else {
            draw_circle(
                center.x,
                center.y,
                PLAYER_RADIUS,
                Color::from_rgba(133, 204, 255, 255),
            );
            draw_circle_lines(center.x, center.y, PLAYER_RADIUS, 2.0, WHITE);
            draw_circle(center.x + 5.0, center.y - 4.0, 2.5, WHITE);
        }
    }

    fn draw_sleep_flash_overlay(&self) {
        if self.runtime.sleep_flash_seconds <= 0.0 {
            return;
        }
        let t = (self.runtime.sleep_flash_seconds / 1.2).clamp(0.0, 1.0);
        let pulse = ((get_time() as f32 * 16.0).sin() * 0.5 + 0.5) * t;
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(180, 22, 18, (100.0 + pulse * 110.0) as u8),
        );
        draw_panel(
            screen_width() * 0.5 - 260.0,
            screen_height() * 0.5 - 64.0,
            520.0,
            128.0,
            ui_copy("gameplay_sleep_flash_title"),
        );
        draw_text(
            ui_copy("gameplay_fainted_home"),
            screen_width() * 0.5 - 220.0,
            screen_height() * 0.5 + 10.0,
            28.0,
            Color::from_rgba(255, 236, 216, 255),
        );
    }

    fn current_season(&self) -> &'static str {
        match (self.world.day_index / 5) % 4 {
            0 => "spring",
            1 => "summer",
            2 => "autumn",
            _ => "winter",
        }
    }

    fn current_weather(&self) -> &'static str {
        match self.world.day_index % 4 {
            0 => "clear",
            1 => "mist",
            2 => "rain",
            _ => "windy",
        }
    }

    fn node_daily_roll(&self, node_id: &str) -> u32 {
        let mut value = self.world.day_index.wrapping_mul(31);
        for byte in node_id.as_bytes() {
            value = value.wrapping_mul(33).wrapping_add(*byte as u32);
        }
        (value % 100) + 1
    }

    fn refresh_available_nodes(&mut self, data: &GameData) {
        self.world.available_nodes.clear();
        let Some(area) = data.area(&self.world.current_area_id) else {
            return;
        };

        for node in &area.gather_nodes {
            let season_ok = node.seasons.is_empty()
                || node
                    .seasons
                    .iter()
                    .any(|season| season == self.current_season());
            let weather_ok = node.weathers.is_empty()
                || node
                    .weathers
                    .iter()
                    .any(|weather| weather == self.current_weather());
            let time_ok = node.time_windows.is_empty()
                || node
                    .time_windows
                    .iter()
                    .any(|time| time == self.current_time_window());
            let daily_roll = self.node_daily_roll(&node.id);

            if season_ok && weather_ok && time_ok && daily_roll <= node.spawn_chance {
                self.world.available_nodes.insert(node.id.clone());
            }
        }
    }

    fn advance_planters(&mut self, data: &GameData) {
        for state in self.progression.planter_states.values_mut() {
            if state.planted_item_id.is_empty() || state.ready {
                continue;
            }
            let days = data
                .stations
                .iter()
                .find(|station| station.id == state.station_id)
                .map(|station| {
                    station
                        .planter_harvest_days
                        .max(1)
                        .saturating_sub(state.mutation_growth_bonus_days)
                        .max(1)
                })
                .unwrap_or(2);
            state.growth_days = self.world.day_index.saturating_sub(state.planted_day);
            if state.growth_days >= days {
                state.ready = true;
            }
        }
    }

    fn push_journal_milestone(&mut self, id: &str, title: &str, text: &str) {
        if self.progression.journal_milestones.iter().any(|entry| entry.id == id) {
            return;
        }
        self.progression.journal_milestones.push(JournalMilestoneEntry {
            id: id.to_owned(),
            title: title.to_owned(),
            text: text.to_owned(),
        });
        self.push_event_toast_with_icon(
            ui_format("gameplay_new_journal_note", &[("title", title)]),
            Color::from_rgba(176, 226, 255, 255),
            "journal_note",
        );
        self.trigger_world_feedback(
            self.world.player.position,
            Color::from_rgba(176, 226, 255, 255),
            true,
            1.6,
        );
    }

    fn has_journal_milestone(&self, id: &str) -> bool {
        self.progression.journal_milestones.iter().any(|entry| entry.id == id)
    }

    fn nearby_station<'a>(&self, data: &'a GameData) -> Option<&'a StationDefinition> {
        self.visible_stations(data).into_iter().find(|station| {
            station.area_id == self.world.current_area_id
                && self
                    .world
                    .player
                    .position
                    .distance(vec2(station.position[0], station.position[1]))
                    <= station.interaction_radius
        })
    }

    fn visible_stations<'a>(&self, data: &'a GameData) -> Vec<&'a StationDefinition> {
        data.stations
            .iter()
            .filter(|station| {
                station.required_completed_quest.is_empty()
                    || self
                        .progression
                        .completed_quests
                        .contains(&station.required_completed_quest)
            })
            .filter(|station| self.progression.total_brews >= station.required_total_brews)
            .filter(|station| {
                station.required_journal_milestone.is_empty()
                    || self.has_journal_milestone(&station.required_journal_milestone)
            })
            .collect()
    }

    fn warp_is_unlocked(&self, warp: &WarpDefinition) -> bool {
        self.progression.unlocked_warps.contains(&warp.id)
            || (warp.required_total_brews == 0
                && warp.required_coins == 0
                && warp.required_item_id.is_empty()
                && warp.required_journal_milestone.is_empty())
    }

    fn can_unlock_warp(&self, warp: &WarpDefinition) -> bool {
        self.progression.total_brews >= warp.required_total_brews
            && self.coins >= warp.required_coins
            && (warp.required_item_id.is_empty()
                || self
                    .inventory
                    .get(&warp.required_item_id)
                    .copied()
                    .unwrap_or_default()
                    >= warp.required_item_amount)
            && (warp.required_journal_milestone.is_empty()
                || self.has_journal_milestone(&warp.required_journal_milestone))
    }

    fn pay_warp_costs(&mut self, warp: &WarpDefinition) {
        self.coins = self.coins.saturating_sub(warp.required_coins);
        if !warp.required_item_id.is_empty() {
            if let Some(amount) = self.inventory.get_mut(&warp.required_item_id) {
                *amount = amount.saturating_sub(warp.required_item_amount);
            }
            self.inventory.retain(|_, amount| *amount > 0);
        }
    }

    fn warp_requirement_summary(&self, data: &GameData, warp: &WarpDefinition) -> String {
        let mut parts = Vec::new();
        if self.progression.total_brews < warp.required_total_brews {
            parts.push(format!(
                "{} more brews",
                warp.required_total_brews.saturating_sub(self.progression.total_brews)
            ));
        }
        if self.coins < warp.required_coins {
            parts.push(format!(
                "{} more coins",
                warp.required_coins.saturating_sub(self.coins)
            ));
        }
        if !warp.required_item_id.is_empty() {
            let owned = self
                .inventory
                .get(&warp.required_item_id)
                .copied()
                .unwrap_or_default();
            if owned < warp.required_item_amount {
                parts.push(format!(
                    "{} x{}",
                    data.item_name(&warp.required_item_id),
                    warp.required_item_amount.saturating_sub(owned)
                ));
            }
        }
        if !warp.required_journal_milestone.is_empty()
            && !self.has_journal_milestone(&warp.required_journal_milestone)
        {
            parts.push(if warp.required_journal_hint.is_empty() {
                "recover the required archive entry".to_owned()
            } else {
                warp.required_journal_hint.clone()
            });
        }
        if parts.is_empty() {
            warp.locked_note.clone()
        } else {
            parts.join(", ")
        }
    }

    fn warp_lock_text(&self, data: &GameData, warp: &WarpDefinition) -> String {
        let requirement_summary = self.warp_requirement_summary(data, warp);
        if requirement_summary == warp.locked_note {
            requirement_summary
        } else {
            format!("{} needs {}.", warp.label, requirement_summary)
        }
    }

    fn interact_with_planter(&mut self, data: &GameData, station: &StationDefinition) {
        let existing_state = self.progression.planter_states.get(&station.id).cloned();
        let candidate = existing_state
            .as_ref()
            .filter(|state| state.planted_item_id.is_empty())
            .and_then(|_| self.planter_seed_choice(data, station));
        let mutation_candidate = existing_state.as_ref().and_then(|state| {
            (!state.planted_item_id.is_empty() && !state.ready && state.mutation_formula_id.is_empty())
                .then(|| self.planter_mutation_candidate(data, &state.planted_item_id))
                .flatten()
        });
        let mut state = self
            .progression
            .planter_states
            .remove(&station.id)
            .unwrap_or(PlanterStateEntry {
                station_id: station.id.clone(),
                planted_item_id: String::new(),
                planted_day: self.world.day_index,
                ready: false,
                tended_day: 0,
                growth_days: 0,
                mutation_formula_id: String::new(),
                mutation_yield_bonus: 0,
                mutation_growth_bonus_days: 0,
                mutation_note: String::new(),
            });
        if state.ready && !state.planted_item_id.is_empty() {
            let harvest_amount = 2 + station.planter_yield_bonus + state.mutation_yield_bonus;
            let mutation_note = state.mutation_note.clone();
            *self
                .inventory
                .entry(state.planted_item_id.clone())
                .or_insert(0) += harvest_amount;
            self.runtime.status_text = if mutation_note.is_empty() {
                format!(
                    "Harvested {} x{} from {}.",
                    data.item_name(&state.planted_item_id),
                    harvest_amount,
                    station.name
                )
            } else {
                format!(
                    "Harvested {} x{} from {}. Mutation: {}.",
                    data.item_name(&state.planted_item_id),
                    harvest_amount,
                    station.name,
                    mutation_note
                )
            };
            state.planted_item_id.clear();
            state.ready = false;
            state.growth_days = 0;
            state.tended_day = 0;
            state.mutation_formula_id.clear();
            state.mutation_yield_bonus = 0;
            state.mutation_growth_bonus_days = 0;
            state.mutation_note.clear();
            self.progression.planter_states.insert(station.id.clone(), state);
            return;
        }
        if !state.planted_item_id.is_empty() {
            if state.tended_day != self.world.day_index {
                state.tended_day = self.world.day_index;
                state.growth_days += 1;
                let mutation_text =
                    self.apply_planter_mutation(data, &mut state, mutation_candidate.as_ref());
                let growth_target = station
                    .planter_harvest_days
                    .max(1)
                    .saturating_sub(state.mutation_growth_bonus_days)
                    .max(1);
                if state.growth_days >= growth_target {
                    state.ready = true;
                    self.runtime.status_text = if let Some(text) = mutation_text {
                        format!(
                            "A careful tending pushes {} to ripeness. {}",
                            station.name, text
                        )
                    } else {
                        ui_format("gameplay_tending_ripeness", &[("station", &station.name)])
                    };
                } else {
                    self.runtime.status_text = if let Some(text) = mutation_text {
                        format!(
                            "Tended {}. Growth stage: {}. {}",
                            station.name,
                            planter_stage_label(state.growth_days, growth_target),
                            text
                        )
                    } else {
                        format!(
                            "Tended {}. Growth stage: {}.",
                            station.name,
                            planter_stage_label(state.growth_days, growth_target)
                        )
                    };
                }
            } else {
                let growth_target = station
                    .planter_harvest_days
                    .max(1)
                    .saturating_sub(state.mutation_growth_bonus_days)
                    .max(1);
                let days_left = growth_target.saturating_sub(state.growth_days);
                self.runtime.status_text = format!(
                    "{} is {}. {} day(s) left.",
                    station.name,
                    planter_stage_label(state.growth_days, growth_target),
                    days_left
                );
            }
            self.progression.planter_states.insert(station.id.clone(), state);
            return;
        }

        let Some(item_id) = candidate else {
            self.runtime.status_text = if station.planter_seed_ids.is_empty() {
                "Planters need a rarer ingredient specimen to cultivate.".to_owned()
            } else {
                format!(
                    "{} accepts: {}.",
                    station.name,
                    station
                        .planter_seed_ids
                        .iter()
                        .map(|item_id| data.item_name(item_id))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            self.progression.planter_states.insert(station.id.clone(), state);
            return;
        };
        if let Some(amount) = self.inventory.get_mut(&item_id) {
            *amount -= 1;
        }
        self.inventory.retain(|_, amount| *amount > 0);
        state.planted_item_id = item_id.clone();
        state.planted_day = self.world.day_index;
        state.ready = false;
        state.tended_day = 0;
        state.growth_days = 0;
        state.mutation_formula_id.clear();
        state.mutation_yield_bonus = 0;
        state.mutation_growth_bonus_days = 0;
        state.mutation_note.clear();
        self.runtime.status_text =
            ui_format("gameplay_planted", &[("item", data.item_name(&item_id)), ("station", &station.name)]);
        self.progression.planter_states.insert(station.id.clone(), state);
    }

    fn planter_seed_choice(&self, data: &GameData, station: &StationDefinition) -> Option<String> {
        self.inventory
            .iter()
            .find(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| {
                            item.category == ItemCategory::Ingredient
                                && item.rarity >= 2
                                && (station.planter_seed_ids.is_empty()
                                    || station.planter_seed_ids.iter().any(|seed| seed == *item_id))
                        })
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
    }

    fn interact_with_habitat(&mut self, data: &GameData, station: &StationDefinition) {
        let candidate = self
            .inventory
            .iter()
            .find(|(item_id, amount)| {
                **amount > 0
                    && station
                        .habitat_creature_ids
                        .iter()
                        .any(|creature_id| creature_id == *item_id)
            })
            .map(|(item_id, _)| item_id.clone());
        let state = self
            .progression
            .habitat_states
            .entry(station.id.clone())
            .or_insert(HabitatStateEntry {
                station_id: station.id.clone(),
                creature_item_id: String::new(),
                placed_day: self.world.day_index,
                last_harvest_day: self.world.day_index,
            });

        if state.creature_item_id.is_empty() {
            let Some(creature_id) = candidate else {
                self.runtime.status_text = format!(
                    "{} accepts {}.",
                    station.name,
                    station
                        .habitat_creature_ids
                        .iter()
                        .map(|item_id| data.item_name(item_id))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                return;
            };
            if let Some(amount) = self.inventory.get_mut(&creature_id) {
                *amount -= 1;
            }
            self.inventory.retain(|_, amount| *amount > 0);
            state.creature_item_id = creature_id.clone();
            state.placed_day = self.world.day_index;
            state.last_harvest_day = self.world.day_index;
            self.push_journal_milestone(
                "containment_started",
                "Containment Floor",
                "The creature habitats are stable enough to hold gentle tower specimens. Managed collection can begin.",
            );
            self.runtime.status_text = format!(
                "Settled {} into {}.",
                data.item_name(&creature_id),
                station.name
            );
            return;
        }

        let ready_day = state
            .last_harvest_day
            .saturating_add(station.habitat_harvest_days.max(1));
        if self.world.day_index >= ready_day {
            let amount = 1 + u32::from(self.progression.total_brews >= 20);
            *self
                .inventory
                .entry(station.habitat_output_item_id.clone())
                .or_insert(0) += amount;
            state.last_harvest_day = self.world.day_index;
            self.runtime.status_text = format!(
                "Collected {} x{} from {}.",
                data.item_name(&station.habitat_output_item_id),
                amount,
                station.name
            );
        } else {
            let days_left = ready_day.saturating_sub(self.world.day_index);
            self.runtime.status_text = format!(
                "{} is calm. {} day(s) until more {}.",
                data.item_name(&state.creature_item_id),
                days_left,
                data.item_name(&station.habitat_output_item_id)
            );
        }
    }

    fn apply_effect(&mut self, effect: &EffectDefinition) {
        match effect.kind {
            EffectKind::Restore => self.vitality = (self.vitality + effect.magnitude).min(100.0),
            EffectKind::Speed | EffectKind::Glow | EffectKind::Misfire => {
                if let Some(existing) = self
                    .runtime
                    .active_effects
                    .iter_mut()
                    .find(|active| active.kind == effect.kind)
                {
                    existing.magnitude = existing.magnitude.max(effect.magnitude);
                    existing.remaining_seconds =
                        existing.remaining_seconds.max(effect.duration_seconds);
                    existing.description = effect.description.clone();
                } else {
                    self.runtime.active_effects.push(ActiveEffect {
                        kind: effect.kind,
                        magnitude: effect.magnitude,
                        remaining_seconds: effect.duration_seconds,
                        description: effect.description.clone(),
                    });
                }
            }
        }
    }

    fn move_speed_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        for effect in &self.runtime.active_effects {
            if effect.kind == EffectKind::Speed {
                multiplier += effect.magnitude;
            } else if effect.kind == EffectKind::Misfire {
                multiplier -= 0.25 * effect.magnitude;
            }
        }
        multiplier.max(0.55)
    }

    fn effect_active(&self, kind: EffectKind) -> bool {
        self.runtime.active_effects.iter().any(|effect| effect.kind == kind)
    }
}

