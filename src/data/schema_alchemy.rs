use super::ElementProfile;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MorphDefinition {
    pub output_item_id: String,
    #[serde(default)]
    pub minimum_quality: u32,
    #[serde(default)]
    pub catalyst_tag: String,
    #[serde(default = "default_heat")]
    pub required_heat: i32,
    #[serde(default)]
    pub required_stirs: u32,
    #[serde(default)]
    pub required_timing: String,
    #[serde(default)]
    pub required_sequence: Vec<String>,
    #[serde(default)]
    pub room_bonus_required: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoomBonusDefinition {
    #[serde(default)]
    pub quality_bonus: u32,
    #[serde(default)]
    pub favored_traits: Vec<String>,
    #[serde(default)]
    pub favored_categories: Vec<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecipeIngredient {
    pub item_id: String,
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecipeDefinition {
    pub id: String,
    pub name: String,
    pub station_id: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub output_item_id: String,
    pub output_amount: u32,
    pub description: String,
    #[serde(default = "default_heat")]
    pub required_heat: i32,
    #[serde(default)]
    pub required_stirs: u32,
    #[serde(default = "default_unstable_output")]
    pub unstable_output_item_id: String,
    #[serde(default)]
    pub lore_note: String,
    #[serde(default)]
    pub minimum_quality: u32,
    #[serde(default)]
    pub preferred_traits: Vec<String>,
    #[serde(default)]
    pub guaranteed_traits: Vec<String>,
    #[serde(default)]
    pub minimum_elements: ElementProfile,
    #[serde(default)]
    pub catalyst_tag: String,
    #[serde(default)]
    pub catalyst_quality_bonus: u32,
    #[serde(default)]
    pub required_timing: String,
    #[serde(default)]
    pub required_sequence: Vec<String>,
    #[serde(default)]
    pub morph_targets: Vec<MorphDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuneRecipeDefinition {
    pub id: String,
    pub station_id: String,
    pub input_item_id: String,
    pub rune_item_id: String,
    pub output_item_id: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MutationFormulaDefinition {
    pub id: String,
    pub seed_item_id: String,
    #[serde(default)]
    pub required_effect_kind: String,
    #[serde(default)]
    pub yield_bonus: u32,
    #[serde(default)]
    pub growth_bonus_days: u32,
    #[serde(default)]
    pub mutation_note: String,
}

fn default_heat() -> i32 {
    2
}

fn default_unstable_output() -> String {
    "murky_concoction".to_owned()
}
