use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use macroquad::rand::gen_range;

pub struct AudioAssets {
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
    pub async fn load() -> Self {
        Self {
            footstep_stone: load_variations("footstep_stone", 6).await,
            footstep_dirt_path: load_variations("footstep_dirt_path", 6).await,
            footstep_greenhouse: load_variations("footstep_greenhouse", 5).await,
            gather_pickup: load_variations("gather_herb_pickup", 5).await,
            alchemy_open: load_variations("alchemy_station_open", 2).await,
            alchemy_stir: load_variations("alchemy_stir", 4).await,
            brew_success: load_variations("brew_success", 3).await,
            brew_collapse: load_variations("brew_collapse", 3).await,
        }
    }

    pub fn play_footstep_for_area(&self, area_id: &str) {
        match area_id {
            "tower_entry" | "archive_floor" | "containment_floor" | "rune_workshop_floor" | "observatory_floor" => {
                play_random(&self.footstep_stone, 0.34);
            }
            "greenhouse_floor" => play_random(&self.footstep_greenhouse, 0.30),
            _ => play_random(&self.footstep_dirt_path, 0.32),
        }
    }

    pub fn play_gather_pickup(&self) {
        play_random(&self.gather_pickup, 0.42);
    }

    pub fn play_alchemy_open(&self) {
        play_random(&self.alchemy_open, 0.42);
    }

    pub fn play_alchemy_stir(&self) {
        play_random(&self.alchemy_stir, 0.38);
    }

    pub fn play_brew_result(&self, success: bool) {
        if success {
            play_random(&self.brew_success, 0.48);
        } else {
            play_random(&self.brew_collapse, 0.44);
        }
    }
}

async fn load_variations(base_name: &str, count: usize) -> Vec<Sound> {
    let mut sounds = Vec::new();
    for index in 1..=count {
        let path = format!("assets/generated/audio/{base_name}_{index}.wav");
        if let Ok(sound) = load_sound(&path).await {
            sounds.push(sound);
        }
    }
    sounds
}

fn play_random(sounds: &[Sound], volume: f32) {
    if sounds.is_empty() {
        return;
    }
    let index = gen_range(0, sounds.len() as i32) as usize;
    play_sound(
        &sounds[index],
        PlaySoundParams {
            looped: false,
            volume,
        },
    );
}
