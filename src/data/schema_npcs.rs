use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct NpcDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) area_id: String,
    pub(crate) position: [f32; 2],
    pub(crate) interaction_radius: f32,
    pub(crate) color: [u8; 4],
    pub(crate) dialogue_start: String,
    pub(crate) dialogue_progress: String,
    pub(crate) dialogue_complete: String,
    #[serde(default)]
    pub(crate) quest_id: String,
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
