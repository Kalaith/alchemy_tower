//! Core exploration and alchemy state.

use std::collections::{BTreeMap, HashSet};

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use crate::alchemy::{quality_band, resolve_brew};
use crate::content::{input_bindings, narrative_text, ui_text};
use crate::data::{
    AreaDefinition, CraftedItemProfileEntry, EffectDefinition, EffectKind, ExperimentLogEntry,
    FieldJournalEntry, GameData, HabitatStateEntry, InventoryEntry, ItemCategory,
    JournalMilestoneEntry, NpcDefinition, PlanterStateEntry, QuestDefinition,
    RecipeMasteryEntry, RelationshipEntry, SaveData, StationDefinition, StationKind,
    WarpDefinition,
};
use crate::save::SaveRepository;
use crate::state::StateTransition;
use crate::ui::{draw_interaction_prompt, draw_panel};

#[path = "gameplay_gathering.rs"]
mod gameplay_gathering;
#[path = "gameplay_input.rs"]
mod gameplay_input;
#[path = "gameplay_inventory.rs"]
mod gameplay_inventory;
#[path = "gameplay_npc.rs"]
mod gameplay_npc;
#[path = "gameplay_overlays.rs"]
mod gameplay_overlays;
#[path = "gameplay_persistence.rs"]
mod gameplay_persistence;
#[path = "gameplay_progression.rs"]
mod gameplay_progression;
#[path = "gameplay_quests.rs"]
mod gameplay_quests;
#[path = "gameplay_support.rs"]
mod gameplay_support;

use self::gameplay_support::{
    clock_text, draw_wrapped_text, effect_name, initial_journal_milestones, planter_stage_label,
    quality_band_rank, rgba, starting_day_time,
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
    text: String,
    remaining_seconds: f32,
    color: Color,
}

#[derive(Clone, Debug)]
struct GatherFeedback {
    position: Vec2,
    remaining_seconds: f32,
    color: Color,
    emphasis: bool,
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
    ending_open: bool,
    dialogue_open: bool,
    current_npc_id: Option<String>,
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

#[derive(Clone, Debug)]
struct WorldState {
    current_area_id: String,
    player: PlayerAvatar,
    day_index: u32,
    day_clock_seconds: f32,
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
    npc_motion_states: BTreeMap<String, NpcMotionTracker>,
    status_text: String,
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
                },
                day_index: 0,
                day_clock_seconds: starting_day_time(data),
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
                npc_motion_states: BTreeMap::new(),
                status_text: "Gather ingredients and experiment at the tower cauldron.".to_owned(),
            },
            ui: OverlayState {
                shop_buy_tab: true,
                ..Default::default()
            },
            alchemy: AlchemySession::default(),
        };
        state.initialize_npc_motion_states(data);
        state.refresh_available_nodes(data);
        state
    }

    pub fn update(&mut self, data: &GameData) -> Option<StateTransition> {
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
                self.runtime.status_text = "Stepped back from the observatory lens.".to_owned();
                return None;
            }
            if self.ui.dialogue_open {
                self.ui.dialogue_open = false;
                self.ui.current_npc_id = None;
                self.runtime.status_text = "Ended conversation.".to_owned();
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
        self.world.day_clock_seconds += frame_time * 60.0;
        while self.world.day_clock_seconds >= data.config.day_length_seconds {
            self.world.day_clock_seconds -= data.config.day_length_seconds;
            self.world.day_index += 1;
            self.world.gathered_nodes.clear();
            self.advance_planters(data);
            self.refresh_available_nodes(data);
            self.runtime.status_text = format!(
                "A new day begins: {} in {}.",
                self.current_weather(),
                self.current_season()
            );
        }
        self.update_active_effects(frame_time);
        self.update_gather_feedback(frame_time);
        self.update_npc_motion(data, frame_time);

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
            self.handle_alchemy_inputs(data);
        } else {
            if is_key_pressed(KeyCode::J) {
                self.ui.journal_open = true;
                self.ui.journal_tab = 0;
                self.runtime.status_text = ui_text().statuses.open_journal.clone();
            }
            if self.runtime.gather_pause_seconds <= 0.0 {
                self.update_movement(data, frame_time);
                self.handle_potion_inputs(data);
                self.handle_interactions(data);
            }
        }

        if is_key_pressed(KeyCode::F5) {
            self.runtime.status_text =
                match gameplay_persistence::GameplayStateLoader::save_slot(self, data) {
                    Ok(()) => "Saved progress.".to_owned(),
                    Err(error) => format!("Save failed: {error}"),
                };
        }
        if is_key_pressed(KeyCode::F9) {
            self.runtime.status_text =
                match gameplay_persistence::GameplayStateLoader::load_slot(self, data) {
                    Ok(()) => "Loaded progress.".to_owned(),
                    Err(error) => format!("Load failed: {error}"),
                };
        }

        None
    }

    pub fn draw(&self, data: &GameData) {
        let Some(area) = data.area(&self.world.current_area_id) else {
            draw_text("Missing area data", 40.0, 80.0, 32.0, RED);
            return;
        };

        let offset = self.camera_offset(area);
        self.draw_area(area, offset, data);
        self.draw_player(offset);
        self.draw_hud(area, data);
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
            self.draw_field_journal(data);
        } else if !self.alchemy.open {
            self.draw_prompt(area, offset, data);
        } else {
            self.draw_alchemy_overlay(data);
        }
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
            return;
        }

        let speed = data.config.move_speed * self.move_speed_multiplier();
        let candidate = self.world.player.position + direction.normalize() * speed * frame_time;
        self.world.player.position = self.resolve_area_collision(area, candidate);
    }

    fn handle_interactions(&mut self, data: &GameData) {
        if let Some(station) = self.nearby_station(data) {
            if station.kind == StationKind::Alchemy && is_key_pressed(KeyCode::Tab) {
                self.alchemy.open = true;
                self.alchemy.index = 0;
                self.runtime.status_text = ui_text().statuses.open_alchemy.clone();
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
            self.runtime.status_text = format!("Talking to {}.", npc.name);
            return;
        }

        if let Some(station) = self.nearby_station(data) {
            if station.kind == StationKind::Shop {
                self.ui.shop_open = true;
                self.ui.shop_buy_tab = true;
                self.ui.shop_index = 0;
                self.runtime.status_text = format!("Opened {}.", station.name);
                return;
            }
            if station.kind == StationKind::RuneWorkshop {
                self.ui.rune_open = true;
                self.ui.rune_index = 0;
                self.runtime.status_text = format!("Opened {}.", station.name);
                return;
            }
            if station.kind == StationKind::ArchiveConsole {
                self.ui.archive_open = true;
                self.ui.archive_tab = 0;
                self.ui.archive_index = 0;
                self.runtime.status_text = format!("Opened {}.", station.name);
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
                    self.runtime.status_text = "The observatory aligns with the restored tower.".to_owned();
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
                    self.push_event_toast(
                        format!("Route restored: {}.", warp.label),
                        Color::from_rgba(188, 255, 220, 255),
                    );
                    self.runtime.status_text = format!("Repaired access to {}.", warp.label);
                } else {
                    self.runtime.status_text = self.warp_lock_text(data, warp);
                    return;
                }
            }
            self.world.current_area_id = warp.target_area.clone();
            self.world.player.position = vec2(warp.target_position[0], warp.target_position[1]);
            self.refresh_available_nodes(data);
            self.runtime.status_text = format!("Entered {}", warp.label);
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
            } else {
                self.runtime.status_text = format!(
                    "{} is unavailable. {}",
                    node.name,
                    self.gather_unavailable_reason(node)
                );
            }
        }
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

    fn draw_area(&self, area: &AreaDefinition, offset: Vec2, data: &GameData) {
        let bg = rgba(area.background);
        draw_rectangle(offset.x, offset.y, area.size[0], area.size[1], bg);
        for y in 0..(area.size[1] / 64.0).ceil() as i32 {
            for x in 0..(area.size[0] / 64.0).ceil() as i32 {
                let tint = if (x + y) % 2 == 0 { 0.0 } else { 0.05 };
                draw_rectangle(
                    offset.x + x as f32 * 64.0,
                    offset.y + y as f32 * 64.0,
                    62.0,
                    62.0,
                    Color::new(
                        (bg.r + tint).min(1.0),
                        (bg.g + tint).min(1.0),
                        (bg.b + tint).min(1.0),
                        1.0,
                    ),
                );
            }
        }
        for blocker in &area.blockers {
            draw_rectangle(
                offset.x + blocker.x,
                offset.y + blocker.y,
                blocker.w,
                blocker.h,
                rgba(area.accent),
            );
        }
        for warp in &area.warps {
            draw_rectangle_lines(
                offset.x + warp.rect.x,
                offset.y + warp.rect.y,
                warp.rect.w,
                warp.rect.h,
                3.0,
                Color::from_rgba(255, 245, 160, 255),
            );
        }
        for station in self
            .visible_stations(data)
            .into_iter()
            .filter(|station| station.area_id == area.id)
        {
            draw_circle(
                offset.x + station.position[0],
                offset.y + station.position[1],
                24.0,
                rgba(station.color),
            );
            draw_circle_lines(
                offset.x + station.position[0],
                offset.y + station.position[1],
                28.0,
                3.0,
                WHITE,
            );
        }
        for npc in self.visible_npcs(data) {
            let runtime = self.npc_runtime_state(data, npc);
            if runtime.area_id != area.id {
                continue;
            }
            let pos = self.npc_draw_position(npc, &runtime);
            draw_circle(offset.x + pos.x, offset.y + pos.y, 18.0, rgba(npc.color));
            draw_circle_lines(offset.x + pos.x, offset.y + pos.y, 20.0, 2.0, WHITE);
        }
        for node in &area.gather_nodes {
            if self.world.gathered_nodes.contains(&node.id) {
                continue;
            }
            let available = self.node_is_available(node);
            let mut color = rgba(node.color);
            if !available {
                color.a = 0.28;
            }
            let center = vec2(offset.x + node.position[0], offset.y + node.position[1]);
            self.draw_gather_node(data, node, center, color, available);
        }
    }

    fn draw_player(&self, offset: Vec2) {
        let center = offset + self.world.player.position;
        if self.effect_active(EffectKind::Glow) {
            draw_circle(
                center.x,
                center.y,
                PLAYER_RADIUS + 18.0,
                Color::from_rgba(215, 202, 255, 70),
            );
        }
        draw_circle(
            center.x,
            center.y,
            PLAYER_RADIUS,
            Color::from_rgba(133, 204, 255, 255),
        );
        draw_circle_lines(center.x, center.y, PLAYER_RADIUS, 2.0, WHITE);
        draw_circle(center.x + 5.0, center.y - 4.0, 2.5, WHITE);
    }

    fn draw_hud(&self, area: &AreaDefinition, data: &GameData) {
        draw_panel(18.0, 18.0, 430.0, 176.0, "Status");
        draw_text(&area.name, 34.0, 58.0, 32.0, dark::TEXT_BRIGHT);
        draw_text(
            &format!("Coins {}", self.coins),
            276.0,
            58.0,
            28.0,
            Color::from_rgba(255, 214, 132, 255),
        );
        draw_text(
            &format!("Vitality {:.0}", self.vitality),
            34.0,
            86.0,
            24.0,
            Color::from_rgba(126, 220, 158, 255),
        );
        draw_text(
            &format!(
                "Time {}",
                clock_text(self.world.day_clock_seconds, data.config.day_length_seconds)
            ),
            34.0,
            112.0,
            24.0,
            dark::TEXT,
        );
        draw_text(
            &format!("Tower Progress {}/10 brews", self.progression.total_brews.min(10)),
            34.0,
            138.0,
            22.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &format!(
                "{} / {} / Day {}",
                self.current_season(),
                self.current_weather(),
                self.world.day_index + 1
            ),
            220.0,
            112.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_text(&self.runtime.status_text, 34.0, 138.0, 20.0, dark::TEXT_DIM);
        if let Some(quest_title) = self.active_quest_title(data) {
            draw_text(
                &format!("Quest: {}", quest_title),
                34.0,
                160.0,
                20.0,
                Color::from_rgba(255, 230, 170, 255),
            );
            if let Some(location_hint) = self.active_quest_location_hint(data) {
                draw_text(
                    &format!("Turn-in trail: {}", location_hint),
                    34.0,
                    180.0,
                    18.0,
                    dark::TEXT_DIM,
                );
            }
        }

        draw_panel(screen_width() - 320.0, 18.0, 302.0, 222.0, "Inventory");
        let mut y = 58.0;
        let inventory_items = self.sorted_inventory_items(data);
        if inventory_items.is_empty() {
            draw_text(
                "No stock recorded.",
                screen_width() - 302.0,
                y,
                22.0,
                dark::TEXT_DIM,
            );
        } else {
            for item_id in inventory_items {
                let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                draw_text(
                    &format!("{} x{}", data.item_name(&item_id), amount),
                    screen_width() - 302.0,
                    y,
                    20.0,
                    dark::TEXT,
                );
                y += 18.0;
                let context = self.inventory_reference_summary(data, &item_id);
                draw_text(
                    if context.is_empty() {
                        "stocked"
                    } else {
                        &context
                    },
                    screen_width() - 302.0,
                    y,
                    16.0,
                    dark::TEXT_DIM,
                );
                y += 26.0;
                if y > 214.0 {
                    break;
                }
            }
        }

        draw_panel(screen_width() - 320.0, 252.0, 302.0, 154.0, "Effects");
        let mut ey = 292.0;
        if self.runtime.active_effects.is_empty() {
            draw_text(
                "No active potion effects.",
                screen_width() - 302.0,
                ey,
                22.0,
                dark::TEXT_DIM,
            );
        } else {
            for effect in &self.runtime.active_effects {
                draw_text(
                    &format!(
                        "{} {:.0}s",
                        effect_name(effect.kind),
                        effect.remaining_seconds.ceil()
                    ),
                    screen_width() - 302.0,
                    ey,
                    22.0,
                    dark::TEXT_BRIGHT,
                );
                ey += 22.0;
                draw_text(
                    &effect.description,
                    screen_width() - 302.0,
                    ey,
                    18.0,
                    dark::TEXT_DIM,
                );
                ey += 24.0;
            }
        }

        draw_panel(18.0, screen_height() - 166.0, 560.0, 148.0, "Potion Belt");
        let potions = self.quick_potions(data);
        let mut py = screen_height() - 126.0;
        if potions.is_empty() {
            draw_text(
                "No brewed potions available.",
                34.0,
                py,
                22.0,
                dark::TEXT_DIM,
            );
        } else {
            for (index, item_id) in potions.iter().take(3).enumerate() {
                let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                draw_text(
                    &format!(
                        "{}: {} x{}",
                        ["Z", "X", "C"][index],
                        data.item_name(item_id),
                        amount
                    ),
                    34.0,
                    py,
                    22.0,
                    dark::TEXT_BRIGHT,
                );
                py += 22.0;
                let detail = data
                    .item(item_id)
                    .map(|item| {
                        item.effects
                            .iter()
                            .map(|effect| effect.description.as_str())
                            .collect::<Vec<_>>()
                            .join(" / ")
                    })
                    .unwrap_or_default();
                draw_text(&detail, 34.0, py, 18.0, dark::TEXT_DIM);
                py += 26.0;
            }
        }
        draw_text(
            "J: Field Journal",
            418.0,
            screen_height() - 26.0,
            18.0,
            dark::TEXT_DIM,
        );
        self.draw_gather_toasts();
        self.draw_gather_feedbacks(area);
    }

    fn draw_gather_node(
        &self,
        data: &GameData,
        node: &crate::data::GatherNodeDefinition,
        center: Vec2,
        color: Color,
        available: bool,
    ) {
        let outline = if available {
            WHITE
        } else {
            Color::from_rgba(220, 220, 220, 120)
        };
        let pulse = ((get_time() as f32 * 3.2) + node.radius).sin() * 0.5 + 0.5;
        let aura_alpha = if available {
            (34.0 + pulse * 28.0) as u8
        } else {
            14
        };
        let aura = Color::new(color.r, color.g, color.b, aura_alpha as f32 / 255.0);
        draw_circle(center.x, center.y, node.radius + 4.0, aura);

        let item = data.item(&node.item_id);
        let is_creature = item
            .map(|item| {
                item.traits.iter().any(|item_trait| {
                    item_trait.contains("wing")
                        || item_trait.contains("creature")
                        || item_trait.contains("insect")
                })
            })
            .unwrap_or(false);
        let is_mineral = item
            .map(|item| {
                item.category == ItemCategory::Catalyst
                    || item
                        .traits
                        .iter()
                        .any(|item_trait| item_trait.contains("arcane"))
            })
            .unwrap_or(false);
        let is_luminous = item
            .map(|item| {
                item.traits
                    .iter()
                    .any(|item_trait| item_trait.contains("luminous"))
            })
            .unwrap_or(false);

        if is_creature {
            draw_circle(center.x, center.y, node.radius - 2.0, color);
            draw_circle_lines(center.x, center.y, node.radius + 1.0, 2.0, outline);
            draw_circle_lines(center.x, center.y, node.radius - 6.0, 2.0, outline);
            draw_circle(center.x - 5.0, center.y - 3.0, 2.0, outline);
            draw_circle(center.x + 5.0, center.y - 3.0, 2.0, outline);
        } else if is_mineral {
            draw_poly(center.x, center.y, 4, node.radius + 2.0, 45.0, color);
            draw_poly_lines(center.x, center.y, 4, node.radius + 2.0, 45.0, 2.0, outline);
            draw_circle(center.x, center.y, 3.0, outline);
        } else if is_luminous {
            draw_circle(center.x, center.y, node.radius - 2.0, color);
            draw_circle_lines(center.x, center.y, node.radius + 1.0, 2.0, outline);
            draw_circle(
                center.x + node.radius * 0.4,
                center.y - node.radius * 0.5,
                3.0,
                outline,
            );
            draw_circle(
                center.x - node.radius * 0.45,
                center.y + node.radius * 0.3,
                2.0,
                outline,
            );
        } else {
            draw_circle(center.x, center.y, node.radius - 3.0, color);
            draw_circle_lines(center.x, center.y, node.radius, 2.0, outline);
            draw_line(
                center.x,
                center.y + node.radius * 0.2,
                center.x,
                center.y + node.radius + 4.0,
                2.0,
                outline,
            );
            draw_circle(center.x - 5.0, center.y - 2.0, 4.0, outline);
            draw_circle(center.x + 5.0, center.y + 1.0, 4.0, outline);
        }
    }

    fn draw_gather_toasts(&self) {
        let start_x = screen_width() * 0.5 - 200.0;
        let mut y = 28.0;
        for toast in self.runtime.gather_toasts.iter().take(3).rev() {
            let alpha = (toast.remaining_seconds / 2.2).clamp(0.0, 1.0);
            let bg = Color::new(18.0 / 255.0, 18.0 / 255.0, 24.0 / 255.0, alpha * 0.9);
            let border = Color::new(toast.color.r, toast.color.g, toast.color.b, alpha);
            let text = Color::new(toast.color.r, toast.color.g, toast.color.b, alpha);
            draw_rectangle(start_x, y, 400.0, 28.0, bg);
            draw_rectangle_lines(start_x, y, 400.0, 28.0, 2.0, border);
            draw_text(&toast.text, start_x + 10.0, y + 19.0, 20.0, text);
            y += 34.0;
        }
    }

    fn draw_gather_feedbacks(&self, area: &AreaDefinition) {
        let offset = self.camera_offset(area);
        for feedback in &self.runtime.gather_feedbacks {
            let life = feedback.remaining_seconds;
            let t = 1.0
                - if feedback.emphasis {
                    life / 0.8
                } else {
                    life / 0.45
                };
            let radius = if feedback.emphasis {
                12.0 + t * 24.0
            } else {
                10.0 + t * 16.0
            };
            let alpha = (1.0 - t).clamp(0.0, 1.0);
            let color = Color::new(feedback.color.r, feedback.color.g, feedback.color.b, alpha);
            let screen_pos = offset + feedback.position;
            draw_circle_lines(screen_pos.x, screen_pos.y, radius, 2.0, color);
            for index in 0..4 {
                let angle = t * 0.8 + index as f32 * std::f32::consts::FRAC_PI_2;
                let sparkle = vec2(angle.cos(), angle.sin()) * (radius + 4.0);
                draw_circle(
                    screen_pos.x + sparkle.x,
                    screen_pos.y + sparkle.y,
                    2.0,
                    color,
                );
            }
        }
    }

    fn draw_prompt(&self, area: &AreaDefinition, offset: Vec2, data: &GameData) {
        if let Some(npc) = self.nearby_npc(data) {
            let npc_pos = self.npc_runtime_state(data, npc).position;
            let pos = vec2(offset.x + npc_pos.x, offset.y + npc_pos.y - 42.0);
            draw_interaction_prompt(pos, &format!("E: Talk to {}", npc.name));
            return;
        }

        if let Some(station) = self.nearby_station(data) {
            let pos = vec2(
                offset.x + station.position[0],
                offset.y + station.position[1] - 42.0,
            );
            if station.kind == StationKind::Alchemy {
                draw_interaction_prompt(
                    pos,
                    &format!(
                        "{}: {}",
                        input_bindings().alchemy.open,
                        ui_text().prompts.open_alchemy
                    ),
                );
            } else if station.kind == StationKind::Shop {
                draw_interaction_prompt(
                    pos,
                    &format!(
                        "{}: {}",
                        input_bindings().global.interact,
                        ui_text().prompts.browse_shop
                    ),
                );
            } else if station.kind == StationKind::RuneWorkshop {
                draw_interaction_prompt(
                    pos,
                    &format!(
                        "{}: {}",
                        input_bindings().global.interact,
                        ui_text().prompts.open_rune_workshop
                    ),
                );
            } else if station.kind == StationKind::ArchiveConsole {
                draw_interaction_prompt(
                    pos,
                    &format!(
                        "{}: {}",
                        input_bindings().global.interact,
                        ui_text().prompts.reconstruct_archives
                    ),
                );
            } else if station.kind == StationKind::EndingFocus {
                draw_interaction_prompt(
                    pos,
                    &format!(
                        "{}: {}",
                        input_bindings().global.interact,
                        ui_text().prompts.focus_observatory
                    ),
                );
            } else if station.kind == StationKind::QuestBoard {
                draw_interaction_prompt(
                    pos,
                    &format!(
                        "{}: {}",
                        input_bindings().global.interact,
                        ui_text().prompts.read_request_board
                    ),
                );
            } else if station.kind == StationKind::Planter {
                let label = self
                    .progression
                    .planter_states
                    .get(&station.id)
                    .map(|state| {
                        if state.ready {
                            "E: Harvest planter"
                        } else if state.planted_item_id.is_empty() {
                            "E: Plant rare specimen"
                        } else if state.tended_day != self.world.day_index {
                            "E: Tend planter"
                        } else {
                            "E: Check growth"
                        }
                    })
                    .unwrap_or("E: Plant rare specimen");
                draw_interaction_prompt(pos, label);
            } else if station.kind == StationKind::Habitat {
                let label = self
                    .progression
                    .habitat_states
                    .get(&station.id)
                    .map(|state| {
                        if state.creature_item_id.is_empty() {
                            "E: Place creature"
                        } else if self.world.day_index
                            >= state
                                .last_harvest_day
                                .saturating_add(station.habitat_harvest_days.max(1))
                        {
                            "E: Harvest habitat"
                        } else {
                            "E: Check habitat"
                        }
                    })
                    .unwrap_or("E: Place creature");
                draw_interaction_prompt(pos, label);
            }
            return;
        }

        if let Some(warp) = area
            .warps
            .iter()
            .find(|warp| warp.rect.contains_point(self.world.player.position))
        {
            let pos = vec2(
                offset.x + self.world.player.position.x,
                offset.y + self.world.player.position.y - 36.0,
            );
            if self.warp_is_unlocked(warp) {
                draw_interaction_prompt(pos, &format!("E: Enter {}", warp.label));
            } else {
                draw_interaction_prompt(pos, &self.warp_lock_text(data, warp));
            }
            return;
        }

        if let Some(node) = area.gather_nodes.iter().find(|node| {
            !self.world.gathered_nodes.contains(&node.id)
                && self
                    .world
                    .player
                    .position
                    .distance(vec2(node.position[0], node.position[1]))
                    <= node.radius + data.config.interaction_range + 28.0
        }) {
            let pos = vec2(
                offset.x + node.position[0],
                offset.y + node.position[1] - node.radius - 18.0,
            );
            if self.node_is_available(node) {
                draw_interaction_prompt(pos, &format!("E: Gather {}", node.name));
            } else {
                draw_interaction_prompt(
                    pos,
                    &format!("{}: {}", node.name, self.gather_unavailable_reason(node)),
                );
            }
        }
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
        self.push_event_toast(
            format!("New journal note: {}.", title),
            Color::from_rgba(176, 226, 255, 255),
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

    fn warp_lock_text(&self, data: &GameData, warp: &WarpDefinition) -> String {
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
            format!("{} needs {}.", warp.label, parts.join(", "))
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
                        format!("A careful tending pushes {} to ripeness.", station.name)
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
        self.runtime.status_text = format!("Planted {} in {}.", data.item_name(&item_id), station.name);
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

