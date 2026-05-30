use super::ElementProfile;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct MorphDefinition {
    pub(crate) output_item_id: String,
    #[serde(default)]
    pub(crate) minimum_quality: u32,
    #[serde(default)]
    pub(crate) catalyst_tag: String,
    #[serde(default = "default_heat")]
    pub(crate) required_heat: i32,
    #[serde(default)]
    pub(crate) required_stirs: u32,
    #[serde(default)]
    pub(crate) required_timing: String,
    #[serde(default)]
    pub(crate) required_sequence: Vec<String>,
    #[serde(default)]
    pub(crate) room_bonus_required: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct RoomBonusDefinition {
    #[serde(default)]
    pub(crate) quality_bonus: u32,
    #[serde(default)]
    pub(crate) favored_traits: Vec<String>,
    #[serde(default)]
    pub(crate) favored_categories: Vec<String>,
    #[serde(default)]
    pub(crate) description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RecipeIngredient {
    pub(crate) item_id: String,
    pub(crate) amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RecipeDefinition {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) station_id: String,
    pub(crate) ingredients: Vec<RecipeIngredient>,
    pub(crate) output_item_id: String,
    pub(crate) output_amount: u32,
    pub(crate) description: String,
    #[serde(default = "default_heat")]
    pub(crate) required_heat: i32,
    #[serde(default)]
    pub(crate) required_stirs: u32,
    #[serde(default = "default_unstable_output")]
    pub(crate) unstable_output_item_id: String,
    #[serde(default)]
    pub(crate) lore_note: String,
    #[serde(default)]
    pub(crate) minimum_quality: u32,
    #[serde(default)]
    pub(crate) preferred_traits: Vec<String>,
    #[serde(default)]
    pub(crate) guaranteed_traits: Vec<String>,
    #[serde(default)]
    pub(crate) minimum_elements: ElementProfile,
    #[serde(default)]
    pub(crate) catalyst_tag: String,
    #[serde(default)]
    pub(crate) catalyst_quality_bonus: u32,
    #[serde(default)]
    pub(crate) required_timing: String,
    #[serde(default)]
    pub(crate) required_sequence: Vec<String>,
    #[serde(default)]
    pub(crate) morph_targets: Vec<MorphDefinition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RuneRecipeDefinition {
    pub(crate) id: String,
    pub(crate) station_id: String,
    pub(crate) input_item_id: String,
    pub(crate) rune_item_id: String,
    pub(crate) output_item_id: String,
    pub(crate) description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MutationFormulaDefinition {
    pub(crate) id: String,
    pub(crate) seed_item_id: String,
    #[serde(default)]
    pub(crate) required_effect_kind: String,
    #[serde(default)]
    pub(crate) yield_bonus: u32,
    #[serde(default)]
    pub(crate) growth_bonus_days: u32,
    #[serde(default)]
    pub(crate) mutation_note: String,
}

fn default_heat() -> i32 {
    2
}

fn default_unstable_output() -> String {
    "murky_concoction".to_owned()
}
