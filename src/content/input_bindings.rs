use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputBindings {
    pub global: GlobalBindings,
    pub alchemy: AlchemyBindings,
}

#[derive(Debug, Deserialize)]
pub struct GlobalBindings {
    pub interact: String,
}

#[derive(Debug, Deserialize)]
pub struct AlchemyBindings {
    pub open: String,
}

pub fn input_bindings() -> &'static InputBindings {
    static INPUTS: OnceLock<InputBindings> = OnceLock::new();
    INPUTS.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/data/input_bindings.json"))
            .expect("embedded input_bindings.json should be valid")
    })
}
