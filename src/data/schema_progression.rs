use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PlanterStateEntry {
    pub(crate) station_id: String,
    #[serde(default)]
    pub(crate) planted_item_id: String,
    #[serde(default)]
    pub(crate) planted_day: u32,
    #[serde(default)]
    pub(crate) ready: bool,
    #[serde(default)]
    pub(crate) tended_day: u32,
    #[serde(default)]
    pub(crate) growth_days: u32,
    #[serde(default)]
    pub(crate) mutation_formula_id: String,
    #[serde(default)]
    pub(crate) mutation_yield_bonus: u32,
    #[serde(default)]
    pub(crate) mutation_growth_bonus_days: u32,
    #[serde(default)]
    pub(crate) mutation_note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct HabitatStateEntry {
    pub(crate) station_id: String,
    #[serde(default)]
    pub(crate) creature_item_id: String,
    #[serde(default)]
    pub(crate) placed_day: u32,
    #[serde(default)]
    pub(crate) last_harvest_day: u32,
}
