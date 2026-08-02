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

    /// Open a conversation with the whole story already behind it — every quest
    /// finished and every journal beat recorded, which is the state the valley
    /// is in once the epilogue has run.
    ///
    /// The `ending` scene shows the epilogue panel; it does not show what
    /// anybody says afterwards, and until now nothing could. Nine townsfolk
    /// have a last word keyed to `observatory_ending` and the only route to any
    /// of them was to finish the game by hand.
    pub(crate) fn open_dialogue_after_everything(&mut self, data: &GameData, npc_id: &str) {
        for quest in &data.quests {
            self.progression.completed_quests.insert(quest.id.clone());
            for milestone in &quest.completion_milestones {
                self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            }
        }
        for recipe in &data.recipes {
            for milestone in &recipe.discovery_milestones {
                self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
            }
        }
        for milestone in crate::content::narrative_text().milestones.all() {
            self.push_journal_milestone(&milestone.id, &milestone.title, &milestone.text);
        }
        self.open_dialogue_with(npc_id);
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
        self.ui.ending_page = 0;
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Ending);
    }

    /// Open the archive on a chosen tab with a selection past the first page —
    /// the console has five lists and there was no way to look at any of them.
    pub(crate) fn open_archive_sample(&mut self, data: &GameData, tab: usize, index: usize) {
        for recipe in &data.recipes {
            self.progression.known_recipes.insert(recipe.id.clone());
            self.progression.recipe_mastery.insert(recipe.id.clone(), 3);
        }
        for item in &data.items {
            self.inventory.insert(item.id.clone(), 2);
        }
        self.coins = 500;
        self.ui.archive_tab = tab;
        self.ui.archive_index = index;
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Archive);
    }

    /// Stand in a named room with every gate already satisfied, so the capture
    /// harness can look at a floor or biome that was just authored. Nodes whose
    /// season, weather or hour do not match the current moment still stay
    /// absent — that is the honest view of the room right now.
    ///
    /// `time_window` picks the hour. The clock used to be left wherever a new
    /// game starts it, which is morning, so every night-gated node in the game
    /// was invisible to every area capture — and the observatory, whose whole
    /// premise is that the lens only works after dark, photographed as an empty
    /// room. Season and weather were already selectable through the day index;
    /// the hour was the one gate the harness could not reach.
    pub(crate) fn preview_area(
        &mut self,
        data: &GameData,
        area_id: &str,
        day_index: u32,
        time_window: &str,
    ) {
        let Some(area) = data.area(area_id) else {
            return;
        };
        self.world.day_index = day_index;
        // Mid-window in each case, so a capture never lands on a boundary
        // minute and disagrees with the label it was asked for. Night is 22:00
        // rather than the small hours on purpose: `handle_sleep_pressure` drags
        // the player home between 01:00 and 02:00, so a capture aimed at 01:00
        // photographs the entry lab and a faint-home banner instead of the room
        // it asked for.
        let minutes = match time_window {
            "night" => 1320.0,
            "evening" => 1140.0,
            "day" => 840.0,
            _ => 480.0,
        };
        self.set_clock_minutes(minutes);
        for quest in &data.quests {
            self.progression.completed_quests.insert(quest.id.clone());
        }
        for other in &data.areas {
            for warp in &other.warps {
                self.progression.unlocked_warps.insert(warp.id.clone());
            }
        }
        // Stations can also be gated on a journal milestone, and for a long
        // while this only satisfied quests and warps — so every milestone-gated
        // bench was missing from every area capture, and the rooms holding them
        // photographed as empty floors. A capture that quietly omits the thing
        // being verified is worse than no capture at all.
        //
        // The same omission then repeated one layer out: a flourish waits on a
        // beat, and seeding only station gates meant every commission payoff and
        // every treated bank was invisible to the harness — the scene claims
        // *every gate satisfied*, so it has to mean all four writers.
        let mut milestone_ids = data
            .stations
            .iter()
            .map(|station| station.required_journal_milestone.clone())
            .collect::<std::collections::BTreeSet<_>>();
        for other in &data.areas {
            milestone_ids.extend(
                other
                    .gather_nodes
                    .iter()
                    .map(|node| node.required_journal_milestone.clone()),
            );
            milestone_ids.extend(
                other
                    .warps
                    .iter()
                    .map(|warp| warp.required_journal_milestone.clone()),
            );
            for flourish in &other.flourishes {
                milestone_ids.extend(flourish.after_any_journal_milestone.iter().cloned());
            }
        }
        milestone_ids.retain(|id| !id.is_empty());
        for milestone_id in milestone_ids {
            self.push_journal_milestone(&milestone_id, "", "");
        }
        self.progression.total_brews = 40;
        self.world.current_area_id = area_id.to_owned();
        self.refresh_available_nodes(data);
        // Townsfolk are seeded onto their active schedule mark when the state is
        // built, which happened before the clock above was moved — so they stood
        // on their morning marks and then *walked* to wherever the scene asked
        // for, at walking pace, over far more frames than a capture runs. Re-seed
        // now that the hour is right, or a night capture photographs the town at
        // breakfast and the schedule work is invisible.
        self.initialize_npc_motion_states(data);
        // Stand where the content is. Centring the room instead means anything
        // authored in a corner is simply off-camera, which has twice made a
        // capture look like nothing had been added. The *last* available node,
        // because content is appended: the newest thing in a room is the thing
        // most likely to be worth looking at.
        let focus = area
            .gather_nodes
            .iter()
            .rev()
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
        // One stack holding a wild variant, because "which of these came up
        // under the right sky" is the one thing the materials list could not
        // say until now — and a capture of a bench with no variants in the bag
        // proves nothing about it.
        if let Some(variant) = data
            .item("whisper_moss")
            .and_then(|item| item.wild_variants.first())
        {
            let variant_id = variant.id.clone();
            self.note_variant_gathered("whisper_moss", &variant_id);
        }
        self.alchemy.slots[0] = Some("sunleaf".to_string());
        self.alchemy.slots[1] = Some("whisper_moss".to_string());
        self.alchemy.heat = 1;
        self.alchemy.stirs = 1;
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Alchemy);
    }

    /// Seed a second-order bench with the bottles a compound formula asks for,
    /// graded as brews rather than as shop stock, and open it. The plain `brew`
    /// scene cannot show this: a bench that does not take bottles never lists
    /// one, so the grade a poured bottle carries has nowhere to appear.
    pub(crate) fn open_compound_brew_sample(&mut self, data: &GameData) {
        let Some(recipe) = data.recipes.iter().find(|recipe| {
            data.stations
                .iter()
                .any(|station| station.id == recipe.station_id && station.accepts_potions)
                && recipe.ingredients.iter().any(|ingredient| {
                    data.item(&ingredient.item_id)
                        .is_some_and(|item| item.category == crate::data::ItemCategory::Potion)
                })
        }) else {
            return;
        };
        if let Some(station) = data
            .stations
            .iter()
            .find(|station| station.id == recipe.station_id)
        {
            self.world.current_area_id = station.area_id.clone();
            self.world.player.position =
                macroquad::prelude::vec2(station.position[0], station.position[1]);
            // A second-order bench is behind a gate by definition — it is the
            // deepest floor in the tower — so the scene has to have opened it.
            if !station.required_journal_milestone.is_empty() {
                let milestone = station.required_journal_milestone.clone();
                self.push_journal_milestone(&milestone, "", "");
            }
        }
        for (slot, ingredient) in recipe.ingredients.iter().take(3).enumerate() {
            self.inventory.insert(ingredient.item_id.clone(), 2);
            self.alchemy.slots[slot] = Some(ingredient.item_id.clone());
            let Some(item) = data.item(&ingredient.item_id) else {
                continue;
            };
            if item.category != crate::data::ItemCategory::Potion {
                continue;
            }
            self.progression.bottle_stock.insert(
                ingredient.item_id.clone(),
                vec![crate::data::BottleBatchEntry {
                    item_id: ingredient.item_id.clone(),
                    quality_score: 78,
                    quality_band: "Excellent".to_owned(),
                    traits: vec!["luminous".to_owned()],
                    count: 2,
                }],
            );
        }
        if !recipe.catalyst_tag.is_empty() {
            if let Some(catalyst) = data
                .items
                .iter()
                .find(|item| item.catalyst_tags.contains(&recipe.catalyst_tag))
            {
                self.inventory.insert(catalyst.id.clone(), 1);
                self.alchemy.catalyst = Some(catalyst.id.clone());
            }
        }
        self.progression.known_recipes.insert(recipe.id.clone());
        self.progression.total_brews = 40;
        self.alchemy.heat = recipe.required_heat;
        self.alchemy.stirs = recipe.required_stirs;
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Alchemy);
    }

    /// Raise the three event banners the game can stack at once and stand in
    /// the world to look at them. They fade after a couple of seconds, so a
    /// capture has to be taken with them freshly raised — which is the whole
    /// reason this scene exists rather than trying to catch one by hand.
    pub(crate) fn show_event_toasts_sample(&mut self, data: &GameData) {
        self.progression.total_brews = 12;
        let here = [self.world.player.position.x, self.world.player.position.y];
        self.trigger_route_restored_feedback("The switchback is walkable again.", here);
        self.trigger_quest_complete_feedback("Delivered: Something For The Headaches.");
        // The longest thing a banner can be asked to carry: a townsperson's own
        // words about work that beat what they asked for.
        self.remark_on_exceptional_delivery(data, "wren_physician");
        self.runtime.status_text = self.next_goal_summary(data);
    }

    /// Seed a couple of learned herb memories and open the journal, so the
    /// capture harness can render the herb-memory tab (including the new
    /// "brews into" recipe usage line).
    pub(crate) fn open_journal_sample(&mut self, data: &GameData) {
        // Seed every gatherable the valley holds, not a token two: the point of
        // looking at this tab is to see whether it copes with a full shelf.
        let gathered = data
            .areas
            .iter()
            .flat_map(|area| area.gather_nodes.iter())
            .map(|node| (node.item_id.clone(), node.route_id.clone()))
            .collect::<std::collections::BTreeMap<_, _>>();
        // A real shelf is half worked out and half hearsay, and the two states
        // say different things — the learned entry gives exact conditions, the
        // seen one gives what the valley says about the herb. Seeding
        // everything as learned meant the second of those had never been
        // photographed. The first entry is deliberately the unlearned one, so a
        // capture opens on it.
        for (index, (item_id, route_id)) in gathered.iter().enumerate() {
            let (item_id, route_id) = (item_id.as_str(), route_id.as_str());
            let learned = index % 3 != 0;
            self.progression.herb_memories.insert(
                item_id.to_owned(),
                crate::data::HerbMemoryEntry {
                    item_id: item_id.to_owned(),
                    first_seen_day: 0,
                    first_seen_route_id: route_id.to_owned(),
                    seen: true,
                    learned,
                    learned_day: u32::from(learned),
                    learned_route_id: if learned {
                        route_id.to_owned()
                    } else {
                        String::new()
                    },
                    note: String::new(),
                    best_quality: if learned { 28 } else { 0 },
                    best_quality_band: "Serviceable".to_owned(),
                    variant_name: String::new(),
                },
            );
        }
        // Half the shelf is worth more than the data file says, and the entry
        // could not tell the player whether any of it was in the bag.
        for (index, item_id) in gathered.keys().enumerate() {
            if index % 4 != 0 {
                continue;
            }
            let Some(variant) = data
                .item(item_id)
                .and_then(|item| item.wild_variants.first())
                .map(|variant| variant.id.clone())
            else {
                continue;
            };
            let item_id = item_id.clone();
            self.note_variant_gathered(&item_id, &variant);
            self.note_variant_gathered(&item_id, &variant);
        }
        self.ui.journal_tab = 0;
        // The list sorts worked-out herbs first, so the hearsay entries are all
        // at the far end and a capture opening on row one photographs the state
        // that was already visible. Point at the first unlearned one.
        if let Some(index) = self
            .herb_memories(data)
            .iter()
            .position(|entry| !entry.learned)
        {
            self.ui.journal_index = index;
        }
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Journal);
    }

    /// Point the herb column at one named entry. The list is sorted rather than
    /// filed, so there is no arithmetic that finds a given herb's row — and the
    /// entry worth photographing is the longest one, not the first.
    pub(crate) fn select_journal_herb(&mut self, data: &GameData, item_id: &str) {
        if let Some(index) = self
            .herb_memories(data)
            .iter()
            .position(|entry| entry.item_id == item_id)
        {
            self.ui.journal_index = index;
        }
    }

    /// Seed a ready-to-hand-in repeatable board request and open the quest
    /// board, so the capture harness can render the delivery flow.
    /// Stand at the rune workbench holding every potion a rune will rework, so
    /// the drafts list can actually be looked at. It had no capture scene at
    /// all until the salvage reworks grew it from nine entries to thirteen — a
    /// list nobody had ever photographed, which is how the drafts overflow got
    /// in the first time.
    pub(crate) fn open_rune_bench_sample(&mut self, data: &GameData) {
        if let Some(station) = data
            .stations
            .iter()
            .find(|station| station.kind == crate::data::StationKind::RuneWorkshop)
        {
            self.world.current_area_id = station.area_id.clone();
            self.world.player.position =
                macroquad::prelude::vec2(station.position[0], station.position[1]);
        }
        for recipe in &data.rune_recipes {
            self.inventory.insert(recipe.input_item_id.clone(), 2);
            self.inventory.insert(recipe.rune_item_id.clone(), 2);
        }
        self.progression.total_brews = 24;
        // Point at the end of the list. Drafts are appended, so the first page
        // is always the oldest content — a capture of rows 1-5 would have looked
        // identical before and after four reworks were added.
        self.ui.rune_index = data.rune_recipes.len().saturating_sub(1);
        self.set_overlay(super::gameplay_overlay_types::OverlayScreen::Rune);
    }

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
