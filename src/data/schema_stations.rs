use super::{RoomBonusDefinition, StationKind, StationRenderDefinition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct StationDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) kind: StationKind,
    pub(crate) area_id: String,
    pub(crate) position: [f32; 2],
    pub(crate) interaction_radius: f32,
    pub(crate) color: [u8; 4],
    #[serde(default)]
    pub(crate) stock: Vec<ShopStockDefinition>,
    #[serde(default)]
    pub(crate) room_bonus: RoomBonusDefinition,
    #[serde(default)]
    pub(crate) planter_harvest_days: u32,
    #[serde(default)]
    pub(crate) planter_seed_ids: Vec<String>,
    #[serde(default)]
    pub(crate) planter_yield_bonus: u32,
    #[serde(default)]
    pub(crate) required_completed_quest: String,
    #[serde(default)]
    pub(crate) required_total_brews: u32,
    #[serde(default)]
    pub(crate) required_journal_milestone: String,
    #[serde(default)]
    pub(crate) habitat_creature_ids: Vec<String>,
    #[serde(default)]
    pub(crate) habitat_output_item_id: String,
    #[serde(default)]
    pub(crate) habitat_harvest_days: u32,
    #[serde(default)]
    pub(crate) render: StationRenderDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ShopStockDefinition {
    pub(crate) item_id: String,
    pub(crate) price: u32,
}
