use macroquad::audio::{load_sound, load_sound_from_bytes, Sound};
use macroquad_toolkit::assets::AssetPack;

const GENERATED_ASSET_PACK: &str = "assets/generated.zip";

pub(super) async fn load_generated_asset_pack() -> Option<AssetPack> {
    AssetPack::load(GENERATED_ASSET_PACK).await.ok()
}

pub(super) async fn load_variations(
    base_name: &str,
    count: usize,
    asset_pack: Option<&AssetPack>,
) -> Vec<Sound> {
    let mut sounds = Vec::new();
    for index in 1..=count {
        if let Some(sound) = load_variation(base_name, index, asset_pack).await {
            sounds.push(sound);
        }
    }
    sounds
}

async fn load_variation(
    base_name: &str,
    index: usize,
    asset_pack: Option<&AssetPack>,
) -> Option<Sound> {
    let path = variation_path(base_name, index);
    if let Some(bytes) = asset_pack.and_then(|pack| pack.bytes(&path)) {
        if let Ok(sound) = load_sound_from_bytes(bytes).await {
            return Some(sound);
        }
    }

    load_sound(&path).await.ok()
}

fn variation_path(base_name: &str, index: usize) -> String {
    format!("assets/generated/audio/{base_name}_{index}.wav")
}
