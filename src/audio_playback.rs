use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::rand::gen_range;

pub(super) fn play_random(sounds: &[Sound], volume: f32) {
    let Some(sound) = random_sound(sounds) else {
        return;
    };

    play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume,
        },
    );
}

fn random_sound(sounds: &[Sound]) -> Option<&Sound> {
    if sounds.is_empty() {
        return None;
    }

    let index = gen_range(0, sounds.len() as i32) as usize;
    sounds.get(index)
}
