use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputBindings {
    pub global: GlobalBindings,
    pub navigation: NavigationBindings,
    pub alchemy: AlchemyBindings,
    pub shop: ShopBindings,
}

#[derive(Debug, Deserialize)]
pub struct GlobalBindings {
    pub confirm: String,
    pub cancel: String,
    pub interact: String,
    pub journal: String,
}

#[derive(Debug, Deserialize)]
pub struct NavigationBindings {
    pub select: String,
}

#[derive(Debug, Deserialize)]
pub struct AlchemyBindings {
    pub open: String,
    pub heat: String,
    pub fill_slots: String,
    pub catalyst: String,
}

#[derive(Debug, Deserialize)]
pub struct ShopBindings {
    pub switch_tab: String,
}

pub fn input_bindings() -> &'static InputBindings {
    static INPUTS: OnceLock<InputBindings> = OnceLock::new();
    INPUTS.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/data/input_bindings.json"))
            .expect("embedded input_bindings.json should be valid")
    })
}
