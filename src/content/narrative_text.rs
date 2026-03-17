use std::sync::OnceLock;

use serde::Deserialize;

use crate::data::JournalMilestoneEntry;

#[derive(Debug, Deserialize)]
pub struct NarrativeText {
    pub milestones: NarrativeMilestones,
    pub statuses: NarrativeStatuses,
    pub overlays: NarrativeOverlays,
}

#[derive(Debug, Deserialize)]
pub struct NarrativeMilestones {
    pub entry_lab_recovered: NarrativeMilestone,
    pub archive_revelation: NarrativeMilestone,
    pub greenhouse_expanded: NarrativeMilestone,
    pub containment_stable: NarrativeMilestone,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NarrativeMilestone {
    pub id: String,
    pub title: String,
    pub text: String,
}

impl NarrativeMilestone {
    pub fn to_journal_entry(&self) -> JournalMilestoneEntry {
        JournalMilestoneEntry {
            id: self.id.clone(),
            title: self.title.clone(),
            text: self.text.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NarrativeStatuses {
    pub archive_timeline_complete: String,
    pub archive_timeline_incomplete: String,
    pub archive_reconstruction_ready: String,
    pub archive_reconstruction_missing: String,
    pub save_unknown_area: String,
    pub cauldron_empty: String,
    pub greenhouse_unlock: String,
}

#[derive(Debug, Deserialize)]
pub struct NarrativeOverlays {
    pub observatory_epilogue: String,
    pub observatory_footer: String,
}

pub fn narrative_text() -> &'static NarrativeText {
    static TEXT: OnceLock<NarrativeText> = OnceLock::new();
    TEXT.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/data/narrative_text.json"))
            .expect("embedded narrative_text.json should be valid")
    })
}
