use super::{ElementProfile, JournalMilestoneEntry};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub(crate) struct RecipeIngredient {
    pub(crate) item_id: String,
    pub(crate) amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
    /// Journal beats recorded the first time this formula is worked out at a
    /// bench. Quests have written to the journal since the beginning and
    /// discoveries never have, so the game's own memory held every delivery the
    /// player made and not one thing they figured out. Left empty for the bulk
    /// of the catalogue — only the discoveries that are turning points.
    #[serde(default)]
    pub(crate) discovery_milestones: Vec<JournalMilestoneEntry>,
    /// When true, this recipe is logged as known at the start of a new game.
    /// Left false for the wider catalogue so those formulae are discovered by
    /// experimentation at the bench. See `seed_starter_recipes`.
    #[serde(default)]
    pub(crate) starter_known: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RuneRecipeDefinition {
    pub(crate) id: String,
    pub(crate) station_id: String,
    pub(crate) input_item_id: String,
    pub(crate) rune_item_id: String,
    pub(crate) output_item_id: String,
    pub(crate) description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
