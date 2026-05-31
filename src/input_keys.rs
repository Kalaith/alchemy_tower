use macroquad::prelude::{is_key_down, is_key_pressed, KeyCode};

pub(super) fn pressed_label(label: &str) -> bool {
    let Some(key) = key_code_for_label(label) else {
        return false;
    };
    is_key_pressed(key)
}

pub(super) fn any_label_down(labels: &[String]) -> bool {
    labels.iter().any(|label| key_down_label(label))
}

fn key_down_label(label: &str) -> bool {
    let Some(key) = key_code_for_label(label) else {
        return false;
    };
    is_key_down(key)
}

fn key_code_for_label(label: &str) -> Option<KeyCode> {
    match label {
        "A" => Some(KeyCode::A),
        "B" => Some(KeyCode::B),
        "C" => Some(KeyCode::C),
        "D" => Some(KeyCode::D),
        "E" => Some(KeyCode::E),
        "F" => Some(KeyCode::F),
        "G" => Some(KeyCode::G),
        "H" => Some(KeyCode::H),
        "I" => Some(KeyCode::I),
        "J" => Some(KeyCode::J),
        "K" => Some(KeyCode::K),
        "L" => Some(KeyCode::L),
        "M" => Some(KeyCode::M),
        "N" => Some(KeyCode::N),
        "O" => Some(KeyCode::O),
        "P" => Some(KeyCode::P),
        "Q" => Some(KeyCode::Q),
        "R" => Some(KeyCode::R),
        "S" => Some(KeyCode::S),
        "T" => Some(KeyCode::T),
        "U" => Some(KeyCode::U),
        "V" => Some(KeyCode::V),
        "W" => Some(KeyCode::W),
        "X" => Some(KeyCode::X),
        "Y" => Some(KeyCode::Y),
        "Z" => Some(KeyCode::Z),
        "Esc" | "Escape" => Some(KeyCode::Escape),
        "Enter" => Some(KeyCode::Enter),
        "Tab" => Some(KeyCode::Tab),
        "F5" => Some(KeyCode::F5),
        "F9" => Some(KeyCode::F9),
        "F11" => Some(KeyCode::F11),
        "Space" => Some(KeyCode::Space),
        "1" => Some(KeyCode::Key1),
        "2" => Some(KeyCode::Key2),
        "3" => Some(KeyCode::Key3),
        "Down" => Some(KeyCode::Down),
        "Left" => Some(KeyCode::Left),
        "Right" => Some(KeyCode::Right),
        "Up" => Some(KeyCode::Up),
        _ => None,
    }
}
