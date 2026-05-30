use std::sync::OnceLock;

use serde::Deserialize;

use super::embedded_json::parse_required_json;
use crate::data::JournalMilestoneEntry;

#[derive(Debug, Deserialize)]
pub(crate) struct NarrativeText {
    pub(crate) milestones: NarrativeMilestones,
    pub(crate) statuses: NarrativeStatuses,
    pub(crate) overlays: NarrativeOverlays,
    pub(crate) phase1: NarrativePhase1,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NarrativeMilestones {
    pub(crate) entry_lab_recovered: NarrativeMilestone,
    pub(crate) archive_revelation: NarrativeMilestone,
    pub(crate) greenhouse_expanded: NarrativeMilestone,
    pub(crate) first_true_brew: NarrativeMilestone,
    pub(crate) first_town_relief: NarrativeMilestone,
    pub(crate) containment_stable: NarrativeMilestone,
    pub(crate) containment_started: NarrativeMilestone,
    pub(crate) first_rune_imbuing: NarrativeMilestone,
    pub(crate) observatory_ending: NarrativeMilestone,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NarrativeMilestone {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) text: String,
}

impl NarrativeMilestone {
    pub(crate) fn to_journal_entry(&self) -> JournalMilestoneEntry {
        JournalMilestoneEntry {
            id: self.id.clone(),
            title: self.title.clone(),
            text: self.text.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct NarrativeStatuses {
    pub(crate) archive_timeline_complete: String,
    pub(crate) archive_timeline_incomplete: String,
    pub(crate) archive_reconstruction_ready: String,
    pub(crate) archive_reconstruction_missing: String,
    pub(crate) save_unknown_area: String,
    pub(crate) cauldron_empty: String,
    pub(crate) greenhouse_unlock: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NarrativeOverlays {
    pub(crate) observatory_epilogue: String,
    pub(crate) observatory_footer: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NarrativePhase1 {
    pub(crate) crow_default: String,
    pub(crate) crow_after_healing: String,
    pub(crate) crow_after_glow: String,
    pub(crate) crow_after_greenhouse: String,
    pub(crate) elric_after_healing: String,
    pub(crate) elric_after_glow: String,
    pub(crate) elric_after_greenhouse: String,
    pub(crate) brin_after_healing: String,
    pub(crate) ione_after_glow: String,
    pub(crate) mira_after_greenhouse: String,
    pub(crate) rowan_after_greenhouse: String,
}

pub(crate) fn narrative_text() -> &'static NarrativeText {
    static TEXT: OnceLock<NarrativeText> = OnceLock::new();
    TEXT.get_or_init(|| {
        parse_required_json(
            include_str!("../../assets/data/narrative_text.json"),
            "narrative_text.json",
        )
    })
}
