use super::{EffectKind, ElementProfile, ItemCategory};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct WildVariantDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) required_conditions: Vec<String>,
    #[serde(default)]
    pub(crate) bonus_traits: Vec<String>,
    #[serde(default)]
    pub(crate) quality_bonus: u32,
    #[serde(default)]
    pub(crate) elements: ElementProfile,
    #[serde(default)]
    pub(crate) synthesis_weight_bonus: u32,
    #[serde(default)]
    pub(crate) synthesis_value_bonus: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ItemDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) category: ItemCategory,
    pub(crate) base_value: u32,
    pub(crate) color: [u8; 4],
    pub(crate) description: String,
    #[serde(default = "default_item_quality")]
    pub(crate) quality: u32,
    #[serde(default = "default_item_rarity")]
    pub(crate) rarity: u8,
    #[serde(default)]
    pub(crate) elements: ElementProfile,
    #[serde(default)]
    pub(crate) traits: Vec<String>,
    #[serde(default)]
    pub(crate) source_conditions: Vec<String>,
    #[serde(default)]
    pub(crate) wild_variants: Vec<WildVariantDefinition>,
    #[serde(default = "default_synthesis_weight")]
    pub(crate) synthesis_weight: u32,
    #[serde(default)]
    pub(crate) synthesis_value: u32,
    #[serde(default)]
    pub(crate) catalyst_tags: Vec<String>,
    #[serde(default)]
    pub(crate) effects: Vec<EffectDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct EffectDefinition {
    pub(crate) kind: EffectKind,
    pub(crate) magnitude: f32,
    pub(crate) duration_seconds: f32,
    pub(crate) description: String,
}

fn default_item_quality() -> u32 {
    20
}

fn default_item_rarity() -> u8 {
    1
}

fn default_synthesis_weight() -> u32 {
    1
}
