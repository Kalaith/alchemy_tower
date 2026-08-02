use std::sync::OnceLock;

use serde::Deserialize;

use super::embedded_json::parse_required_json;

/// `deny_unknown_fields` throughout: `alchemy.heat` and `alchemy.fill_slots`
/// sat in this file for months as display labels no struct claimed, so serde
/// dropped them in silence and the controls they described were listed nowhere.
/// A binding nothing reads should refuse to load rather than look configured.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InputBindings {
    pub(crate) global: GlobalBindings,
    pub(crate) navigation: NavigationBindings,
    pub(crate) movement: MovementBindings,
    pub(crate) alchemy: AlchemyBindings,
    pub(crate) archive: ArchiveBindings,
    pub(crate) dialogue: DialogueBindings,
    pub(crate) shop: ShopBindings,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GlobalBindings {
    pub(crate) confirm: String,
    pub(crate) cancel: String,
    pub(crate) interact: String,
    pub(crate) journal: String,
    pub(crate) sort: String,
    pub(crate) save: String,
    pub(crate) load: String,
    pub(crate) fullscreen: String,
    pub(crate) quick_potions: [String; 3],
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NavigationBindings {
    pub(crate) select: String,
    pub(crate) select_previous: String,
    pub(crate) select_next: String,
    pub(crate) switch: String,
    pub(crate) switch_previous: String,
    pub(crate) switch_next: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MovementBindings {
    pub(crate) up: Vec<String>,
    pub(crate) down: Vec<String>,
    pub(crate) left: Vec<String>,
    pub(crate) right: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AlchemyBindings {
    pub(crate) open: String,
    pub(crate) heat_decrease: String,
    pub(crate) heat_increase: String,
    pub(crate) fill_slot_keys: [String; 3],
    pub(crate) clear_slot_keys: [String; 3],
    pub(crate) stir: String,
    pub(crate) timing: String,
    pub(crate) catalyst: String,
    pub(crate) remove_catalyst: String,
    pub(crate) clear: String,
    pub(crate) repeat: String,
    pub(crate) brew: String,
    pub(crate) brew_alternate: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArchiveBindings {
    pub(crate) filter: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DialogueBindings {
    pub(crate) advance: String,
    pub(crate) advance_alternate: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ShopBindings {
    pub(crate) switch_tab: String,
}

pub(crate) fn input_bindings() -> &'static InputBindings {
    static INPUTS: OnceLock<InputBindings> = OnceLock::new();
    INPUTS.get_or_init(|| {
        parse_required_json(
            include_str!("../../assets/data/input_bindings.json"),
            "input_bindings.json",
        )
    })
}
