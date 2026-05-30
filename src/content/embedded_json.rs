use serde::de::DeserializeOwned;

pub(super) fn parse_required_json<T>(source: &'static str, label: &'static str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_str(source)
        .unwrap_or_else(|error| panic!("embedded {label} should be valid: {error}"))
}

pub(super) fn parse_json_or_else<T, F>(source: &'static str, label: &'static str, fallback: F) -> T
where
    T: DeserializeOwned,
    F: FnOnce() -> T,
{
    serde_json::from_str(source).unwrap_or_else(|error| {
        eprintln!("Failed to load embedded {label}: {error}");
        fallback()
    })
}
