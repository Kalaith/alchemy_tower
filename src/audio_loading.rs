use macroquad::audio::{load_sound, load_sound_from_bytes, Sound};
use macroquad_toolkit::assets::AssetPack;

const GENERATED_ASSET_PACK: &str = "assets/generated.zip";

pub(super) async fn load_generated_asset_pack() -> Result<AssetPack, String> {
    AssetPack::load(GENERATED_ASSET_PACK).await
}

pub(super) async fn load_variations(
    base_name: &str,
    count: usize,
    asset_pack: Option<&AssetPack>,
) -> Result<Vec<Sound>, String> {
    let mut sounds = Vec::new();
    let mut failures = Vec::new();
    for index in 1..=count {
        match load_variation(base_name, index, asset_pack).await {
            Ok(sound) => sounds.push(sound),
            Err(error) => failures.push(error),
        }
    }
    if failures.is_empty() {
        Ok(sounds)
    } else {
        Err(format!(
            "failed to load {}/{} variation(s) for {}: {}",
            failures.len(),
            count,
            base_name,
            failures.join("; ")
        ))
    }
}

async fn load_variation(
    base_name: &str,
    index: usize,
    asset_pack: Option<&AssetPack>,
) -> Result<Sound, String> {
    let path = variation_path(base_name, index);
    let mut failures = Vec::new();
    if let Some(bytes) = asset_pack.and_then(|pack| pack.bytes(&path)) {
        match load_sound_from_bytes(bytes).await {
            Ok(sound) => return Ok(sound),
            Err(error) => failures.push(format!("packed decode failed: {error:?}")),
        }
    } else if asset_pack.is_some() {
        failures.push("missing from generated asset pack".to_owned());
    }

    match load_sound(&path).await {
        Ok(sound) => Ok(sound),
        Err(error) => {
            failures.push(format!("loose file load failed: {error:?}"));
            Err(format!("{} ({})", path, failures.join("; ")))
        }
    }
}

pub(super) fn variation_path(base_name: &str, index: usize) -> String {
    format!("assets/generated/audio/{base_name}_{index}.wav")
}
