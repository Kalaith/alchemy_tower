use serde::de::DeserializeOwned;

pub(crate) fn load_labeled_json<T>(label: &'static str, source: &'static str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    macroquad_toolkit::data_loader::load_embedded_json_labeled(label, source)
}

#[cfg(debug_assertions)]
pub(crate) fn expect_labeled_json<T>(label: &'static str, source: &'static str) -> T
where
    T: DeserializeOwned,
{
    load_labeled_json(label, source)
        .unwrap_or_else(|error| panic!("embedded {label} should be valid: {error}"))
}
