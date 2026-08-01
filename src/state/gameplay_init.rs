use super::gameplay_alchemy_types::AlchemySession;
use super::gameplay_journal_support::initial_journal_milestones;
use super::gameplay_overlay_types::OverlayState;
use super::gameplay_progression_types::ProgressionState;
use super::gameplay_runtime_types::RuntimeState;
use super::gameplay_support::starting_day_time;
use super::gameplay_world_types::WorldState;
use super::GameplayState;
use crate::data::GameData;
use std::collections::BTreeMap;

impl GameplayState {
    pub(crate) fn new(data: &GameData) -> Self {
        let mut state = Self {
            world: WorldState::new(data, starting_day_time(data)),
            progression: ProgressionState::new(initial_journal_milestones()),
            coins: 24,
            vitality: 100.0,
            inventory: BTreeMap::new(),
            runtime: RuntimeState::new(data),
            ui: OverlayState::new_gameplay(),
            alchemy: AlchemySession::default(),
        };
        state.seed_starter_recipes(data);
        state.initialize_npc_motion_states(data);
        state.refresh_available_nodes(data);
        state
    }

    /// Open a conversation with the given NPC. Used by the screenshot capture
    /// harness to seed a dialogue scene.
    pub(crate) fn open_dialogue_with(&mut self, npc_id: &str) {
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Dialogue(
            npc_id.to_string(),
        ));
    }

    /// Open the epilogue with every journal beat it can respond to recorded, so
    /// the fullest version of the ending can be looked at. It is the one screen
    /// with no other route to it short of finishing the game.
    pub(crate) fn open_full_ending(&mut self) {
        for beat in &crate::content::narrative_text().epilogue_beats {
            for milestone_id in &beat.after_milestones {
                self.push_journal_milestone(milestone_id, milestone_id, "");
            }
        }
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Ending);
    }

    /// Stand in a named room with every gate already satisfied, so the capture
    /// harness can look at a floor or biome that was just authored. Nodes whose
    /// season, weather or hour do not match the current moment still stay
    /// absent — that is the honest view of the room right now.
    pub(crate) fn preview_area(&mut self, data: &GameData, area_id: &str, day_index: u32) {
        let Some(area) = data.area(area_id) else {
            return;
        };
        self.world.day_index = day_index;
        for quest in &data.quests {
            self.progression.completed_quests.insert(quest.id.clone());
        }
        for other in &data.areas {
            for warp in &other.warps {
                self.progression.unlocked_warps.insert(warp.id.clone());
            }
        }
        self.progression.total_brews = 40;
        self.world.current_area_id = area_id.to_owned();
        self.refresh_available_nodes(data);
        // Stand where the content is. Centring the room instead means anything
        // authored in a corner is simply off-camera, which has twice made a
        // capture look like nothing had been added.
        let focus = area
            .gather_nodes
            .iter()
            .find(|node| self.world.available_nodes.contains(&node.id))
            .map(|node| node.position)
            .unwrap_or([area.size[0] * 0.5, area.size[1] * 0.5]);
        self.world.player.position = macroquad::prelude::vec2(focus[0], focus[1]);
    }

    /// Advance a townsperson's arc past its earlier beats and open the
    /// conversation, so the capture harness can see a mid-arc beat rather than
    /// only the opening request every time.
    pub(crate) fn open_dialogue_at_arc_beat(&mut self, data: &GameData, npc_id: &str, beat: usize) {
        let Some(npc) = data.npc(npc_id) else {
            return;
        };
        for quest_id in npc.quest_chain().iter().take(beat) {
            self.progression.completed_quests.insert(quest_id.clone());
            if let Some(quest) = data.quest(quest_id) {
                for prerequisite in &quest.prerequisite_quests {
                    self.progression
                        .completed_quests
                        .insert(prerequisite.clone());
                }
                // Finishing a request in play records its journal beats, and the
                // town's reactions are gated on those. Skipping them here would
                // show a conversation the player can never actually have.
                self.push_quest_completion_milestones(quest);
            }
        }
        self.progression.total_brews = 40;
        self.open_dialogue_with(npc_id);
    }

    /// Seed a filled cauldron and open the alchemy bench, so the capture harness
    /// can render a resolved brew preview. Moves the avatar onto the cauldron so
    /// the overlay survives the station-proximity check in `update`.
    pub(crate) fn open_alchemy_sample_brew(&mut self, data: &GameData) {
        if let Some(station) = data.stations.iter().find(|station| {
            station.kind == crate::data::StationKind::Alchemy
                && station.area_id == self.world.current_area_id
        }) {
            self.world.player.position =
                macroquad::prelude::vec2(station.position[0], station.position[1]);
        }
        self.inventory.insert("sunleaf".to_string(), 3);
        self.inventory.insert("whisper_moss".to_string(), 3);
        self.alchemy.slots[0] = Some("sunleaf".to_string());
        self.alchemy.slots[1] = Some("whisper_moss".to_string());
        self.alchemy.heat = 1;
        self.alchemy.stirs = 1;
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Alchemy);
    }

    /// Seed a couple of learned herb memories and open the journal, so the
    /// capture harness can render the herb-memory tab (including the new
    /// "brews into" recipe usage line).
    pub(crate) fn open_journal_sample(&mut self, _data: &GameData) {
        for (item_id, route_id) in [
            ("whisper_moss", "tower_ruin_edge"),
            ("field_bloom", "plains_crossing"),
        ] {
            self.progression.herb_memories.insert(
                item_id.to_owned(),
                crate::data::HerbMemoryEntry {
                    item_id: item_id.to_owned(),
                    first_seen_day: 0,
                    first_seen_route_id: route_id.to_owned(),
                    seen: true,
                    learned: true,
                    learned_day: 1,
                    learned_route_id: route_id.to_owned(),
                    note: String::new(),
                    best_quality: 28,
                    best_quality_band: "Serviceable".to_owned(),
                    variant_name: String::new(),
                },
            );
        }
        self.ui.journal_tab = 0;
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Journal);
    }

    /// Seed a ready-to-hand-in repeatable board request and open the quest
    /// board, so the capture harness can render the delivery flow.
    pub(crate) fn open_quest_board_sample(&mut self, data: &GameData) {
        if let Some(station) = data
            .stations
            .iter()
            .find(|station| station.kind == crate::data::StationKind::QuestBoard)
        {
            self.world.current_area_id = station.area_id.clone();
            self.world.player.position =
                macroquad::prelude::vec2(station.position[0], station.position[1]);
        }
        self.progression.total_brews = 12;
        self.progression
            .started_quests
            .insert("board_restorative_stash".to_owned());
        self.inventory.insert("healing_draught".to_owned(), 1);
        self.progression.crafted_item_profiles.insert(
            "healing_draught".to_owned(),
            crate::data::CraftedItemProfileEntry {
                item_id: "healing_draught".to_owned(),
                best_quality_score: 60,
                best_quality_band: "Fine".to_owned(),
                inherited_traits: vec!["restorative".to_owned()],
                effect_kinds: vec!["restore".to_owned()],
            },
        );
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::QuestBoard);
    }

    /// Log the handful of recipes flagged `starter_known` so a new player can
    /// see how to brew the town's first potions instead of facing an empty
    /// formulae panel. Every other formula in the catalogue — including the
    /// wider entry-cauldron recipes — stays discovery-only and is learned by
    /// experimenting at the bench. See [[starter-recipe-seeding]].
    fn seed_starter_recipes(&mut self, data: &GameData) {
        for recipe in &data.recipes {
            if recipe.starter_known {
                self.progression.known_recipes.insert(recipe.id.clone());
            }
        }
    }
}
