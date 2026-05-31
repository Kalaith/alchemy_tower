use super::GameplayState;
use crate::content::narrative_text;
use crate::data::{GameData, NpcDefinition};

#[path = "gameplay_npc_dialogue_text.rs"]
mod npc_dialogue_text;

pub(super) struct NpcDialogueSelection<'a> {
    pub(super) start: &'a str,
    pub(super) progress: &'a str,
    pub(super) complete: &'a str,
}

impl GameplayState {
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
        data: &GameData,
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
        let quest = (!npc.quest_id.is_empty())
            .then(|| data.quest(&npc.quest_id))
            .flatten();
        let quest_started = quest
            .map(|quest| self.progression.started_quests.contains(&quest.id))
            .unwrap_or(false);
        let quest_completed = quest
            .map(|quest| self.progression.completed_quests.contains(&quest.id))
            .unwrap_or(false);
        let quest_available = quest
            .map(|quest| self.quest_is_available(quest))
            .unwrap_or(false);

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
            selection.start = phase1.active_request.as_str();
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

    pub(super) fn npc_phase1_followup_line(&self, npc_id: &str) -> Option<&str> {
        let phase1 = &narrative_text().phase1;
        if self.phase1_town_recovery_reached() {
            return match npc_id {
                "crow_guide" => Some(phase1.crow_after_greenhouse.as_str()),
                "mayor_elric" => Some(phase1.elric_after_greenhouse.as_str()),
                "mira_apothecary" => Some(phase1.mira_after_greenhouse.as_str()),
                "rowan_herbalist" => Some(phase1.rowan_after_greenhouse.as_str()),
                _ => None,
            };
        }
        if self.progression.completed_quests.contains("glow_for_rowan") {
            return match npc_id {
                "crow_guide" => Some(phase1.crow_after_glow.as_str()),
                "mayor_elric" => Some(phase1.elric_after_glow.as_str()),
                "ione_archivist" => Some(phase1.ione_after_glow.as_str()),
                _ => None,
            };
        }
        if self
            .progression
            .completed_quests
            .contains("healing_for_mira")
        {
            return match npc_id {
                "crow_guide" => Some(phase1.crow_after_healing.as_str()),
                "mayor_elric" => Some(phase1.elric_after_healing.as_str()),
                "brin_groundskeeper" => Some(phase1.brin_after_healing.as_str()),
                _ => None,
            };
        }
        if npc_id == "crow_guide" {
            return Some(phase1.crow_default.as_str());
        }
        None
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
