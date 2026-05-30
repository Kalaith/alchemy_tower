use serde::de::DeserializeOwned;

pub(super) fn load_embedded_json<T>(label: &'static str, source: &'static str) -> T
where
    T: DeserializeOwned,
{
    macroquad_toolkit::data_loader::load_embedded_json_labeled(label, source)
        .unwrap_or_else(|error| panic!("embedded {label} should be valid: {error}"))
}
