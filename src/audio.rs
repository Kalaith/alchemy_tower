use macroquad::audio::Sound;

#[path = "audio_loading.rs"]
mod audio_loading;
#[path = "audio_playback.rs"]
mod audio_playback;

use self::audio_loading::{load_generated_asset_pack, load_variations};
use self::audio_playback::play_random;

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
    pub(crate) async fn load() -> Self {
        let asset_pack = load_generated_asset_pack().await;

        Self {
            footstep_stone: load_variations("footstep_stone", 6, asset_pack.as_ref()).await,
            footstep_dirt_path: load_variations("footstep_dirt_path", 6, asset_pack.as_ref()).await,
            footstep_greenhouse: load_variations("footstep_greenhouse", 5, asset_pack.as_ref())
                .await,
            gather_pickup: load_variations("gather_herb_pickup", 5, asset_pack.as_ref()).await,
            alchemy_open: load_variations("alchemy_station_open", 2, asset_pack.as_ref()).await,
            alchemy_stir: load_variations("alchemy_stir", 4, asset_pack.as_ref()).await,
            brew_success: load_variations("brew_success", 3, asset_pack.as_ref()).await,
            brew_collapse: load_variations("brew_collapse", 3, asset_pack.as_ref()).await,
        }
    }

    pub(crate) fn play_footstep_for_area(&self, area_id: &str) {
        match area_id {
            "tower_entry"
            | "archive_floor"
            | "containment_floor"
            | "rune_workshop_floor"
            | "observatory_floor" => {
                play_random(&self.footstep_stone, 0.34);
            }
            "greenhouse_floor" => play_random(&self.footstep_greenhouse, 0.30),
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
