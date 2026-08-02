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

    /// Key for remembering that a line has been said.
    ///
    /// Reactions carry no id, and speaker-plus-order will not serve: seven of
    /// them already share an order with a sibling, which would mark both said
    /// the moment either was. Hashing the words themselves gives a short stable
    /// key that survives reordering and re-authoring elsewhere in the file, and
    /// two byte-identical lines from one speaker are interchangeable anyway.
    fn reaction_key(reaction: &NarrativeReaction) -> String {
        // FNV-1a. Small, dependency-free, and stable across builds — which a
        // key written into save files has to be.
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in reaction.npc_id.bytes().chain(reaction.line.bytes()) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x1000_0000_01b3);
        }
        format!("{hash:016x}")
    }

    /// The reaction this townsperson would offer next.
    ///
    /// This used to be `max_by_key(order)` over everything earned, and earning
    /// is monotonic — so a line that came due at the same moment as a later one,
    /// or behind an ancestor of it, lost every time and could never be spoken.
    /// Thirty-six of the hundred and sixty authored lines were unreachable that
    /// way, including three of Ione's in a row.
    ///
    /// Earned-but-unsaid lines now go first, earliest first, so a run of beats
    /// that came due together is worked through one conversation at a time. Once
    /// everything is said the latest line stands as their current word, which is
    /// what the old behaviour was reaching for.
    ///
    /// The conversation still only moves forward: a line that becomes earned
    /// after later ones have already been spoken is skipped rather than dragging
    /// the townsperson back to an older beat. Lines that come due *together* are
    /// all above the last thing said, so they still get their turn.
    fn npc_phase1_followup(&self, npc_id: &str) -> Option<&'static NarrativeReaction> {
        let earned = || {
            narrative_text()
                .reactions
                .iter()
                .filter(move |reaction| reaction.npc_id == npc_id)
                .filter(|reaction| self.reaction_is_earned(reaction))
        };
        let already_said = |reaction: &NarrativeReaction| {
            self.progression
                .spoken_reactions
                .contains(&Self::reaction_key(reaction))
        };
        let furthest_said = earned()
            .filter(|reaction| already_said(reaction))
            .map(|reaction| reaction.order)
            .max();

        earned()
            .filter(|reaction| !already_said(reaction))
            .filter(|reaction| furthest_said.is_none_or(|said| reaction.order >= said))
            .min_by_key(|reaction| reaction.order)
            .or_else(|| earned().max_by_key(|reaction| reaction.order))
    }

    pub(super) fn npc_phase1_followup_line(&self, npc_id: &str) -> Option<&'static str> {
        self.npc_phase1_followup(npc_id)
            .map(|reaction| reaction.line.as_str())
    }

    /// Note that the line currently on offer has now been said. Called when the
    /// player advances the conversation — that is the moment they have read it —
    /// so the next one is waiting the next time they come by.
    pub(super) fn mark_followup_spoken(&mut self, npc_id: &str) {
        let Some(key) = self.npc_phase1_followup(npc_id).map(Self::reaction_key) else {
            return;
        };
        self.progression.spoken_reactions.insert(key);
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

    /// The town talks about what has happened, one thing per conversation, and
    /// keeps moving forward. Walk Elric through his reactions: each new beat is
    /// remarked on once it has been heard, and a beat that lands late does not
    /// drag him back to an older subject.
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

        // Until the player has actually heard it, that is still what he has to
        // say — a new beat does not overwrite one that was never spoken.
        state.push_journal_milestone("greenhouse_repaired", "Greenhouse", "");
        assert_eq!(state.npc_phase1_followup_line(npc_id), Some(after_healing));

        state.mark_followup_spoken(npc_id);
        let after_greenhouse = state
            .npc_phase1_followup_line(npc_id)
            .expect("elric reacts to the greenhouse");
        assert_ne!(after_healing, after_greenhouse);

        state.mark_followup_spoken(npc_id);
        state.push_journal_milestone("harvest_beds_turned", "Bed Rows", "");
        let after_harvest = state
            .npc_phase1_followup_line(npc_id)
            .expect("elric reacts to the square growing");
        assert_ne!(after_greenhouse, after_harvest);

        // An earlier beat arriving late must not drag the conversation back.
        state.mark_followup_spoken(npc_id);
        state
            .progression
            .completed_quests
            .insert("glow_for_rowan".to_owned());
        assert_eq!(state.npc_phase1_followup_line(npc_id), Some(after_harvest));
    }

    /// Thirty-six of the hundred and sixty authored reactions could never be
    /// spoken: the selector took the highest-order earned line, and earning is
    /// monotonic, so anything that came due alongside a later beat lost forever.
    /// With every gate satisfied, a townsperson should work through all of their
    /// lines rather than repeating their last one.
    #[test]
    fn every_reaction_a_townsperson_has_earned_eventually_gets_said() {
        use crate::content::narrative_text;

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        // Satisfy everything: every quest done, every recordable beat recorded.
        for quest in &data.quests {
            state.progression.completed_quests.insert(quest.id.clone());
        }
        for reaction in &narrative_text().reactions {
            if !reaction.after_milestone.is_empty() {
                state.push_journal_milestone(&reaction.after_milestone, "", "");
            }
        }

        let npc_id = "ione_archivist";
        let authored = narrative_text()
            .reactions
            .iter()
            .filter(|reaction| reaction.npc_id == npc_id)
            .count();
        assert!(authored >= 4, "ione should have several lines");

        let mut heard = std::collections::HashSet::new();
        for _ in 0..authored {
            let line = state
                .npc_phase1_followup_line(npc_id)
                .expect("an earned line should be on offer");
            heard.insert(line);
            state.mark_followup_spoken(npc_id);
        }

        assert_eq!(
            heard.len(),
            authored,
            "ione repeated herself instead of working through her {authored} lines"
        );
    }

    /// `quest_ids` is the arc; `quest_id` is the older single-request field that
    /// givers without an arc still use. Test the fallback on a stripped clone
    /// rather than on whichever townsperson happens not to have an arc yet —
    /// that list shrinks every time this loop writes one.
    #[test]
    fn single_quest_givers_still_resolve_without_a_chain() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);
        let mut npc = data
            .npcs
            .iter()
            .find(|npc| !npc.quest_id.is_empty())
            .expect("some townsperson still carries a plain quest_id")
            .clone();
        npc.quest_ids.clear();

        assert_eq!(npc.quest_chain(), std::slice::from_ref(&npc.quest_id));
        assert_eq!(
            state
                .npc_active_quest(&data, &npc)
                .map(|quest| quest.id.as_str()),
            Some(npc.quest_id.as_str())
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
