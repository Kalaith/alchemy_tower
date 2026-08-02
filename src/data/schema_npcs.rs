use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct NpcDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) area_id: String,
    pub(crate) position: [f32; 2],
    pub(crate) interaction_radius: f32,
    pub(crate) color: [u8; 4],
    /// What this townsperson says once every request they had is finished —
    /// their settled word on the whole arc. The `dialogue_start`/`_progress`
    /// pair that used to sit beside it was an earlier, blunter draft of the
    /// same beats the `phase1_dialogue` block now covers in more detail, and
    /// nothing could ever reach it; the prose is in git history if it is wanted
    /// back.
    pub(crate) dialogue_complete: String,
    #[serde(default)]
    pub(crate) quest_id: String,
    /// An ordered arc of requests from this townsperson: setup, complication,
    /// payoff. Only the first unfinished step is ever offered, so a chain reads
    /// as one relationship rather than a pile of simultaneous errands. Takes
    /// precedence over the single `quest_id`, which stays for one-shot givers.
    #[serde(default)]
    pub(crate) quest_ids: Vec<String>,
    #[serde(default)]
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) schedule: Vec<NpcScheduleEntry>,
    #[serde(default)]
    pub(crate) phase1_dialogue: NpcPhase1DialogueDefinition,
    #[serde(default)]
    pub(crate) crow_phase1_dialogue: CrowPhase1DialogueDefinition,
    /// A warm line spoken when this townsperson first counts you a friend
    /// (rapport reaches the friend tier). Also recorded as a journal beat.
    #[serde(default)]
    pub(crate) friendship_line: String,
    /// One-time thank-you gift handed over when friendship is first reached.
    #[serde(default)]
    pub(crate) friendship_reward_coins: u32,
    #[serde(default)]
    pub(crate) friendship_reward_item_id: String,
    #[serde(default)]
    pub(crate) friendship_reward_amount: u32,
    /// Said once their whole arc is finished, with a parting gift that is a
    /// product of the work rather than stock from a shelf. The friend tier is
    /// reached less than halfway through a three-beat arc, so without this the
    /// relationship track resolves long before the relationship does.
    #[serde(default)]
    pub(crate) trusted_line: String,
    #[serde(default)]
    pub(crate) trusted_reward_coins: u32,
    #[serde(default)]
    pub(crate) trusted_reward_item_id: String,
    #[serde(default)]
    pub(crate) trusted_reward_amount: u32,
}

impl NpcDefinition {
    /// This townsperson's requests in story order. Falls back to the single
    /// `quest_id` so one-shot givers need no extra authoring.
    pub(crate) fn quest_chain(&self) -> &[String] {
        if !self.quest_ids.is_empty() {
            &self.quest_ids
        } else if !self.quest_id.is_empty() {
            std::slice::from_ref(&self.quest_id)
        } else {
            &[]
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct NpcScheduleEntry {
    pub(crate) time_window: String,
    pub(crate) area_id: String,
    pub(crate) position: [f32; 2],
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct NpcPhase1DialogueDefinition {
    #[serde(default)]
    pub(crate) intro: String,
    #[serde(default)]
    pub(crate) pre_help_concern: String,
    #[serde(default)]
    pub(crate) active_request: String,
    #[serde(default)]
    pub(crate) post_help_relief: String,
    #[serde(default)]
    pub(crate) town_recovery_observation: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct CrowPhase1DialogueDefinition {
    #[serde(default)]
    pub(crate) first_meeting: String,
    #[serde(default)]
    pub(crate) first_brew: String,
    #[serde(default)]
    pub(crate) first_quest_complete: String,
    #[serde(default)]
    pub(crate) first_tower_restoration: String,
}
