use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlanterStateEntry {
    pub station_id: String,
    #[serde(default)]
    pub planted_item_id: String,
    #[serde(default)]
    pub planted_day: u32,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub tended_day: u32,
    #[serde(default)]
    pub growth_days: u32,
    #[serde(default)]
    pub mutation_formula_id: String,
    #[serde(default)]
    pub mutation_yield_bonus: u32,
    #[serde(default)]
    pub mutation_growth_bonus_days: u32,
    #[serde(default)]
    pub mutation_note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HabitatStateEntry {
    pub station_id: String,
    #[serde(default)]
    pub creature_item_id: String,
    #[serde(default)]
    pub placed_day: u32,
    #[serde(default)]
    pub last_harvest_day: u32,
}
