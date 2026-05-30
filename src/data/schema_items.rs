use super::{EffectKind, ElementProfile, ItemCategory};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WildVariantDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub required_conditions: Vec<String>,
    #[serde(default)]
    pub bonus_traits: Vec<String>,
    #[serde(default)]
    pub quality_bonus: u32,
    #[serde(default)]
    pub elements: ElementProfile,
    #[serde(default)]
    pub synthesis_weight_bonus: u32,
    #[serde(default)]
    pub synthesis_value_bonus: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub category: ItemCategory,
    pub base_value: u32,
    pub color: [u8; 4],
    pub description: String,
    #[serde(default = "default_item_quality")]
    pub quality: u32,
    #[serde(default = "default_item_rarity")]
    pub rarity: u8,
    #[serde(default)]
    pub elements: ElementProfile,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub source_conditions: Vec<String>,
    #[serde(default)]
    pub wild_variants: Vec<WildVariantDefinition>,
    #[serde(default = "default_synthesis_weight")]
    pub synthesis_weight: u32,
    #[serde(default)]
    pub synthesis_value: u32,
    #[serde(default)]
    pub catalyst_tags: Vec<String>,
    #[serde(default)]
    pub effects: Vec<EffectDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EffectDefinition {
    pub kind: EffectKind,
    pub magnitude: f32,
    pub duration_seconds: f32,
    pub description: String,
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
