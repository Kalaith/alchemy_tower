use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct FieldJournalEntry {
    pub(crate) item_id: String,
    pub(crate) route_id: String,
    pub(crate) season: String,
    pub(crate) weather: String,
    pub(crate) time_window: String,
    pub(crate) note: String,
    #[serde(default)]
    pub(crate) best_quality: u32,
    #[serde(default)]
    pub(crate) best_quality_band: String,
    #[serde(default)]
    pub(crate) variant_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct HerbMemoryEntry {
    pub(crate) item_id: String,
    #[serde(default)]
    pub(crate) first_seen_day: u32,
    #[serde(default)]
    pub(crate) first_seen_route_id: String,
    #[serde(default)]
    pub(crate) seen: bool,
    #[serde(default)]
    pub(crate) learned: bool,
    #[serde(default)]
    pub(crate) learned_day: u32,
    #[serde(default)]
    pub(crate) learned_route_id: String,
    #[serde(default)]
    pub(crate) note: String,
    #[serde(default)]
    pub(crate) best_quality: u32,
    #[serde(default)]
    pub(crate) best_quality_band: String,
    #[serde(default)]
    pub(crate) variant_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PotionMemoryEntry {
    pub(crate) item_id: String,
    #[serde(default)]
    pub(crate) first_seen_day: u32,
    #[serde(default)]
    pub(crate) seen: bool,
    #[serde(default)]
    pub(crate) learned: bool,
    #[serde(default)]
    pub(crate) learned_day: u32,
    #[serde(default)]
    pub(crate) successful_brews: u32,
    #[serde(default)]
    pub(crate) best_quality_score: u32,
    #[serde(default)]
    pub(crate) best_quality_band: String,
    #[serde(default)]
    pub(crate) last_recipe_id: String,
}
