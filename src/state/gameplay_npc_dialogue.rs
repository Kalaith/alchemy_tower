use super::GameplayState;
use crate::content::{narrative_text, NarrativeReaction};
use crate::data::{GameData, NpcDefinition, QuestDefinition};

#[path = "gameplay_npc_dialogue_text.rs"]
mod npc_dialogue_text;

pub(super) struct NpcDialogueSelection<'a> {
    pub(super) start: &'a str,
    pub(super) progress: &'a str,
    pub(super) complete: &'a str,
}

impl GameplayState {
    /// The one step of this townsperson's arc that is currently live: the first
    /// request they have not finished. `None` once the whole chain is done,
    /// which every caller already treats as "nothing left to ask of them".
    pub(super) fn npc_active_quest<'a>(
        &self,
        data: &'a GameData,
        npc: &NpcDefinition,
    ) -> Option<&'a QuestDefinition> {
        npc.quest_chain()
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .find(|quest| !self.progression.completed_quests.contains(&quest.id))
    }

    /// Whether the player has finished anything at all for this townsperson.
    /// Drives the warmer "you already helped me" lines, which must survive the
    /// arc moving on to its next beat.
    pub(super) fn npc_has_been_helped(&self, npc: &NpcDefinition) -> bool {
        npc.quest_chain()
            .iter()
            .any(|quest_id| self.progression.completed_quests.contains(quest_id))
    }

    pub(super) fn phase1_town_recovery_reached(&self) -> bool {
        self.has_journal_milestone("greenhouse_repaired")
            || self
                .progression
                .completed_quests
                .contains("cultivation_for_brin")
    }

    pub(super) fn phase1_first_relief_reached(&self) -> bool {
        self.has_journal_milestone("first_town_relief")
            || self
                .progression
                .completed_quests
                .contains("healing_for_mira")
    }

    pub(super) fn phase1_first_brew_reached(&self) -> bool {
        self.has_journal_milestone("first_true_brew") || self.progression.total_brews > 0
    }

    pub(super) fn npc_dialogue_selection<'a>(
        &'a self,
        data: &'a GameData,
        npc: &'a NpcDefinition,
    ) -> NpcDialogueSelection<'a> {
        let mut selection = NpcDialogueSelection {
            start: npc.dialogue_start.as_str(),
            progress: npc.dialogue_progress.as_str(),
            complete: npc.dialogue_complete.as_str(),
        };

        if npc.id == "crow_guide" {
            let crow = &npc.crow_phase1_dialogue;
            if !crow.first_meeting.is_empty() {
                let line = if self.phase1_town_recovery_reached()
                    && !crow.first_tower_restoration.is_empty()
                {
                    crow.first_tower_restoration.as_str()
                } else if self.phase1_first_relief_reached()
                    && !crow.first_quest_complete.is_empty()
                {
                    crow.first_quest_complete.as_str()
                } else if self.phase1_first_brew_reached() && !crow.first_brew.is_empty() {
                    crow.first_brew.as_str()
                } else {
                    crow.first_meeting.as_str()
                };
                selection.start = line;
                selection.progress = line;
                selection.complete = line;
            }
            return selection;
        }

        let phase1 = &npc.phase1_dialogue;
        let quest = self.npc_active_quest(data, npc);
        let quest_started = quest
            .map(|quest| self.progression.started_quests.contains(&quest.id))
            .unwrap_or(false);
        let quest_completed = self.npc_has_been_helped(npc);
        let quest_available = quest
            .map(|quest| self.quest_is_available(quest))
            .unwrap_or(false);

        // A line authored on the live step of an arc is the most specific thing
        // this NPC has to say, so it wins over every broader phase line. Without
        // this, later beats are drowned out by the town-recovery observation.
        if let Some(quest) = quest.filter(|_| quest_started || quest_available) {
            let beat = if quest_started {
                quest.giver_active_line.as_str()
            } else {
                quest.giver_intro_line.as_str()
            };
            if !beat.is_empty() {
                selection.start = beat;
                selection.progress = beat;
                return selection;
            }
        }

        if self.phase1_town_recovery_reached() && !phase1.town_recovery_observation.is_empty() {
            selection.start = phase1.town_recovery_observation.as_str();
            selection.progress = phase1.town_recovery_observation.as_str();
            selection.complete = phase1.town_recovery_observation.as_str();
            return selection;
        }
        if quest_completed && !phase1.post_help_relief.is_empty() {
            selection.complete = phase1.post_help_relief.as_str();
        }
        if (quest_started || quest_available) && !phase1.active_request.is_empty() {
            // Only collapse to the terse "active request" reminder once the
            // player has actually accepted the quest. While it is merely
            // available, the opener stays as the NPC's full pitch
            // (`dialogue_start`), which is where the *reason* for the errand
            // lives — otherwise the first conversation jumps straight to "make
            // me X" with no motivation.
            if quest_started {
                selection.start = phase1.active_request.as_str();
            }
            selection.progress = phase1.active_request.as_str();
        } else if self.phase1_first_brew_reached() && !phase1.pre_help_concern.is_empty() {
            selection.start = phase1.pre_help_concern.as_str();
            selection.progress = phase1.pre_help_concern.as_str();
        } else if !phase1.intro.is_empty() {
            selection.start = phase1.intro.as_str();
            selection.progress = phase1.intro.as_str();
        }

        selection
    }

    pub(super) fn npc_phase1_followup_line(&self, npc_id: &str) -> Option<&'static str> {
        narrative_text()
            .reactions
            .iter()
            .filter(|reaction| reaction.npc_id == npc_id)
            .filter(|reaction| self.reaction_is_earned(reaction))
            .max_by_key(|reaction| reaction.order)
            .map(|reaction| reaction.line.as_str())
    }

    fn reaction_is_earned(&self, reaction: &NarrativeReaction) -> bool {
        let quest_done = reaction.after_quest.is_empty()
            || self
                .progression
                .completed_quests
                .contains(&reaction.after_quest);
        let milestone_done = reaction.after_milestone.is_empty()
            || self.has_journal_milestone(&reaction.after_milestone);
        quest_done && milestone_done
    }

    pub(super) fn append_npc_story_line(&self, npc_id: &str, base: String) -> String {
        let extra = match self.npc_phase1_followup_line(npc_id) {
            Some(extra) => extra,
            None => return base,
        };

        if base.contains(extra) {
            return base;
        }

        npc_dialogue_text::with_followup(&base, extra)
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    #[test]
    fn npc_arc_offers_one_beat_at_a_time() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let rowan = data
            .npc("rowan_herbalist")
            .expect("rowan should exist")
            .clone();
        let chain = rowan.quest_chain().to_vec();
        assert!(chain.len() >= 3, "rowan should carry a multi-beat arc");

        for (index, quest_id) in chain.iter().enumerate() {
            let active = state
                .npc_active_quest(&data, &rowan)
                .expect("an unfinished arc always has a live step");
            assert_eq!(
                &active.id, quest_id,
                "beat {index} should be the live step before it is completed"
            );
            assert!(
                !active.giver_intro_line.is_empty(),
                "{quest_id} should speak in Rowan's voice while on offer"
            );
            state.progression.completed_quests.insert(quest_id.clone());
        }

        assert!(
            state.npc_active_quest(&data, &rowan).is_none(),
            "a finished arc leaves nothing further to ask"
        );
        assert!(state.npc_has_been_helped(&rowan));
    }

    /// The town should be talking about the most recent thing that happened,
    /// not the first. Walk Elric forward through his reactions and check each
    /// new beat displaces the last.
    #[test]
    fn town_reactions_move_on_as_the_story_does() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let npc_id = "mayor_elric";

        assert_eq!(state.npc_phase1_followup_line(npc_id), None);

        state
            .progression
            .completed_quests
            .insert("healing_for_mira".to_owned());
        let after_healing = state
            .npc_phase1_followup_line(npc_id)
            .expect("elric reacts to the first delivered draught");

        state.push_journal_milestone("greenhouse_repaired", "Greenhouse", "");
        let after_greenhouse = state
            .npc_phase1_followup_line(npc_id)
            .expect("elric reacts to the greenhouse");
        assert_ne!(after_healing, after_greenhouse);

        state.push_journal_milestone("harvest_beds_turned", "Bed Rows", "");
        let after_harvest = state
            .npc_phase1_followup_line(npc_id)
            .expect("elric reacts to the square growing");
        assert_ne!(after_greenhouse, after_harvest);

        // An earlier beat arriving late must not drag the conversation back.
        state
            .progression
            .completed_quests
            .insert("glow_for_rowan".to_owned());
        assert_eq!(state.npc_phase1_followup_line(npc_id), Some(after_harvest));
    }

    #[test]
    fn single_quest_givers_still_resolve_without_a_chain() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);
        let mira = data.npc("mira_apothecary").expect("mira should exist");
        assert!(mira.quest_ids.is_empty(), "mira is a one-shot giver");
        assert_eq!(
            state
                .npc_active_quest(&data, mira)
                .map(|quest| quest.id.as_str()),
            Some(mira.quest_id.as_str())
        );
    }

    #[test]
    fn quest_gated_nodes_stay_bare_until_their_beat_is_done() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let town = data.area("town_square").expect("town square should exist");
        let gated = town
            .gather_nodes
            .iter()
            .find(|node| !node.required_completed_quest.is_empty())
            .expect("the turned bed rows should be quest-gated");
        let quest_id = gated.required_completed_quest.clone();
        let node_id = gated.id.clone();

        state.world.current_area_id = "town_square".to_owned();
        // Walk a full season/weather/time cycle so the node's own conditions are
        // never the reason it is absent.
        let mut seen_before = false;
        for day in 0..20 {
            state.world.day_index = day;
            state.refresh_available_nodes(&data);
            seen_before |= state.world.available_nodes.contains(&node_id);
        }
        assert!(
            !seen_before,
            "gated ground should stay bare before its beat"
        );

        state.progression.completed_quests.insert(quest_id);
        let mut seen_after = false;
        for day in 0..20 {
            state.world.day_index = day;
            state.refresh_available_nodes(&data);
            seen_after |= state.world.available_nodes.contains(&node_id);
        }
        assert!(
            seen_after,
            "the turned row should grow once the beat is done"
        );
    }
}
