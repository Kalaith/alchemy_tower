use std::sync::OnceLock;

use serde::Deserialize;

use super::embedded_json::parse_required_json;
use crate::data::JournalMilestoneEntry;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeText {
    pub(crate) milestones: NarrativeMilestones,
    pub(crate) statuses: NarrativeStatuses,
    pub(crate) overlays: NarrativeOverlays,
    /// Filled from `narrative_reactions.json` after parsing rather than read
    /// from this file: the townsfolk's lines outgrew the rest of the narrative
    /// text several times over and were split out at 860 lines.
    #[serde(default)]
    pub(crate) reactions: Vec<NarrativeReaction>,
    pub(crate) epilogue_beats: Vec<NarrativeEpilogueBeat>,
}

/// A closing line the epilogue earns. The ending used to be one fixed paragraph
/// however much of the valley had been put back, which made the last thing the
/// game says the only thing it says that the player had no hand in.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeEpilogueBeat {
    /// Every one of these must be recorded. Empty means always earned.
    #[serde(default)]
    pub(crate) after_milestones: Vec<String>,
    /// Narrative weight, not chronology. The panel has room for a few beats, so
    /// the heaviest earned ones are the ones it finds room for.
    pub(crate) order: u32,
    pub(crate) line: String,
}

/// `deny_unknown_fields` because three entries had accumulated in this block
/// that the struct does not read — byte-identical copies of milestones the
/// quests already record. Nothing broke, which is the problem: rewriting one of
/// them would have changed nothing in the game and looked like it should have.
/// A stray entry is now a load failure rather than prose that goes nowhere.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeMilestones {
    pub(crate) entry_lab_recovered: NarrativeMilestone,
    pub(crate) archive_revelation: NarrativeMilestone,
    pub(crate) first_true_brew: NarrativeMilestone,
    pub(crate) containment_started: NarrativeMilestone,
    pub(crate) first_rune_imbuing: NarrativeMilestone,
    pub(crate) observatory_ending: NarrativeMilestone,
}

impl NarrativeMilestones {
    /// Every milestone this file declares. Used by the content check that
    /// verifies authored reactions are gated on beats something actually
    /// records, and by the capture harness, which needs the whole spine
    /// recorded to photograph a conversation held after the epilogue.
    pub(crate) fn all(&self) -> [&NarrativeMilestone; 6] {
        [
            &self.entry_lab_recovered,
            &self.archive_revelation,
            &self.first_true_brew,
            &self.containment_started,
            &self.first_rune_imbuing,
            &self.observatory_ending,
        ]
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeStatuses {
    pub(crate) archive_timeline_complete: String,
    pub(crate) archive_timeline_incomplete: String,
    pub(crate) archive_reconstruction_ready: String,
    pub(crate) archive_reconstruction_missing: String,
    pub(crate) save_unknown_area: String,
    pub(crate) cauldron_empty: String,
    pub(crate) greenhouse_unlock: String,
    pub(crate) found_formula: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeOverlays {
    pub(crate) observatory_epilogue: String,
    pub(crate) observatory_footer: String,
}

/// What a townsperson says about how far the valley has come, over and above
/// whatever request is currently between you. Authored as a list rather than a
/// fixed set of fields so a new story beat only needs writing, not code: give
/// the reaction a condition and an `order`, and the highest-ordered earned line
/// for that person is the one they speak.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeReaction {
    pub(crate) npc_id: String,
    /// Earned once this quest is completed. Empty means no quest condition.
    #[serde(default)]
    pub(crate) after_quest: String,
    /// Earned once this journal milestone is recorded. Empty means none.
    #[serde(default)]
    pub(crate) after_milestone: String,
    /// Later beats carry a higher order and win over earlier ones.
    pub(crate) order: u32,
    pub(crate) line: String,
}

/// One file per speaker. The single reactions file passed 800 lines and each
/// townsperson's voice is the obvious seam — "what does Brin say" is now a
/// question you answer by opening one small file rather than scrolling a list
/// of everyone. Adding a speaker means adding a line here.
const REACTION_SOURCES: &[(&str, &str)] = &[
    (
        "narrative/reactions_brin_groundskeeper",
        include_str!("../../assets/data/narrative/reactions_brin_groundskeeper.json"),
    ),
    (
        "narrative/reactions_crow_guide",
        include_str!("../../assets/data/narrative/reactions_crow_guide.json"),
    ),
    (
        "narrative/reactions_ione_archivist",
        include_str!("../../assets/data/narrative/reactions_ione_archivist.json"),
    ),
    (
        "narrative/reactions_lyra_keeper",
        include_str!("../../assets/data/narrative/reactions_lyra_keeper.json"),
    ),
    (
        "narrative/reactions_mayor_elric",
        include_str!("../../assets/data/narrative/reactions_mayor_elric.json"),
    ),
    (
        "narrative/reactions_mira_apothecary",
        include_str!("../../assets/data/narrative/reactions_mira_apothecary.json"),
    ),
    (
        "narrative/reactions_rowan_herbalist",
        include_str!("../../assets/data/narrative/reactions_rowan_herbalist.json"),
    ),
    (
        "narrative/reactions_tarn_wayfarer",
        include_str!("../../assets/data/narrative/reactions_tarn_wayfarer.json"),
    ),
    (
        "narrative/reactions_wren_physician",
        include_str!("../../assets/data/narrative/reactions_wren_physician.json"),
    ),
];

/// The reactions file on its own. Only exists so the split file can be parsed
/// and folded into [`NarrativeText`].
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NarrativeReactions {
    reactions: Vec<NarrativeReaction>,
}

pub(crate) fn narrative_text() -> &'static NarrativeText {
    static TEXT: OnceLock<NarrativeText> = OnceLock::new();
    TEXT.get_or_init(|| {
        let mut text: NarrativeText = parse_required_json(
            include_str!("../../assets/data/narrative_text.json"),
            "narrative_text.json",
        );
        text.reactions = REACTION_SOURCES
            .iter()
            .flat_map(|(label, source)| {
                let spoken: NarrativeReactions = parse_required_json(source, label);
                spoken.reactions
            })
            .collect();
        text
    })
}
