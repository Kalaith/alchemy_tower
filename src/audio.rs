use crate::data::AreaDefinition;
use macroquad::audio::Sound;

#[path = "audio_loading.rs"]
mod audio_loading;
#[path = "audio_playback.rs"]
mod audio_playback;

use self::audio_loading::{load_generated_asset_pack, load_variations};
use self::audio_playback::play_random;

#[cfg(test)]
const REQUIRED_VARIATION_SETS: &[(&str, usize)] = &[
    ("footstep_stone", 6),
    ("footstep_dirt_path", 6),
    ("footstep_greenhouse", 5),
    ("gather_herb_pickup", 5),
    ("alchemy_station_open", 2),
    ("alchemy_stir", 4),
    ("brew_success", 3),
    ("brew_collapse", 3),
];

pub(crate) struct AudioAssets {
    footstep_stone: Vec<Sound>,
    footstep_dirt_path: Vec<Sound>,
    footstep_greenhouse: Vec<Sound>,
    gather_pickup: Vec<Sound>,
    alchemy_open: Vec<Sound>,
    alchemy_stir: Vec<Sound>,
    brew_success: Vec<Sound>,
    brew_collapse: Vec<Sound>,
}

impl AudioAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        let asset_pack = match load_generated_asset_pack().await {
            Ok(pack) => Some(pack),
            Err(error) => {
                eprintln!(
                    "Generated asset pack was not loaded for audio; using loose asset files instead: {error}"
                );
                None
            }
        };

        Ok(Self {
            footstep_stone: load_variations("footstep_stone", 6, asset_pack.as_ref()).await?,
            footstep_dirt_path: load_variations("footstep_dirt_path", 6, asset_pack.as_ref())
                .await?,
            footstep_greenhouse: load_variations("footstep_greenhouse", 5, asset_pack.as_ref())
                .await?,
            gather_pickup: load_variations("gather_herb_pickup", 5, asset_pack.as_ref()).await?,
            alchemy_open: load_variations("alchemy_station_open", 2, asset_pack.as_ref()).await?,
            alchemy_stir: load_variations("alchemy_stir", 4, asset_pack.as_ref()).await?,
            brew_success: load_variations("brew_success", 3, asset_pack.as_ref()).await?,
            brew_collapse: load_variations("brew_collapse", 3, asset_pack.as_ref()).await?,
        })
    }

    pub(crate) fn play_footstep_for_area(&self, area: &AreaDefinition) {
        match area.footstep_sound_set.as_str() {
            "stone" => play_random(&self.footstep_stone, 0.34),
            "greenhouse" => play_random(&self.footstep_greenhouse, 0.30),
            _ => play_random(&self.footstep_dirt_path, 0.32),
        }
    }

    pub(crate) fn play_gather_pickup(&self) {
        play_random(&self.gather_pickup, 0.42);
    }

    pub(crate) fn play_alchemy_open(&self) {
        play_random(&self.alchemy_open, 0.42);
    }

    pub(crate) fn play_alchemy_stir(&self) {
        play_random(&self.alchemy_stir, 0.38);
    }

    pub(crate) fn play_brew_result(&self, success: bool) {
        if success {
            play_random(&self.brew_success, 0.48);
        } else {
            play_random(&self.brew_collapse, 0.44);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn required_audio_variation_files_exist() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut missing = Vec::new();

        for (base_name, count) in REQUIRED_VARIATION_SETS {
            for index in 1..=*count {
                let path = audio_loading::variation_path(base_name, index);
                if !root.join(&path).exists() {
                    missing.push(path);
                }
            }
        }

        assert!(
            missing.is_empty(),
            "missing required audio files:\n{}",
            missing.join("\n")
        );
    }

    #[test]
    fn area_footstep_sound_sets_are_known() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let known_sets = ["dirt_path", "greenhouse", "stone"];
        let unknown = data
            .areas
            .iter()
            .filter(|area| !known_sets.contains(&area.footstep_sound_set.as_str()))
            .map(|area| format!("{} -> {}", area.id, area.footstep_sound_set))
            .collect::<Vec<_>>();

        assert!(
            unknown.is_empty(),
            "unknown area footstep sound set(s):\n{}",
            unknown.join("\n")
        );
    }
}
