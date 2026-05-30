use macroquad::prelude::{is_key_down, is_key_pressed, Vec2, KeyCode};

use crate::content::input_bindings;

pub(crate) fn quick_potion_pressed(index: usize) -> bool {
    let Some(label) = input_bindings().global.quick_potions.get(index) else {
        return false;
    };
    pressed_label(label)
}

pub(crate) fn sort_pressed() -> bool {
    pressed_label(&input_bindings().global.sort)
}

pub(crate) fn alchemy_open_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.open)
}

pub(crate) fn alchemy_brew_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.brew)
        || pressed_label(&input_bindings().alchemy.brew_alternate)
}

pub(crate) fn alchemy_catalyst_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.catalyst)
}

pub(crate) fn alchemy_clear_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.clear)
}

pub(crate) fn alchemy_clear_slot_pressed(index: usize) -> bool {
    let Some(label) = input_bindings().alchemy.clear_slot_keys.get(index) else {
        return false;
    };
    pressed_label(label)
}

pub(crate) fn alchemy_fill_slot_pressed(index: usize) -> bool {
    let Some(label) = input_bindings().alchemy.fill_slot_keys.get(index) else {
        return false;
    };
    pressed_label(label)
}

pub(crate) fn alchemy_heat_decrease_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.heat_decrease)
}

pub(crate) fn alchemy_heat_increase_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.heat_increase)
}

pub(crate) fn alchemy_remove_catalyst_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.remove_catalyst)
}

pub(crate) fn alchemy_repeat_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.repeat)
}

pub(crate) fn alchemy_stir_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.stir)
}

pub(crate) fn alchemy_timing_pressed() -> bool {
    pressed_label(&input_bindings().alchemy.timing)
}

pub(crate) fn archive_filter_pressed() -> bool {
    pressed_label(&input_bindings().archive.filter)
}

pub(crate) fn cancel_pressed() -> bool {
    pressed_label(&input_bindings().global.cancel)
}

pub(crate) fn confirm_pressed() -> bool {
    pressed_label(&input_bindings().global.confirm)
}

pub(crate) fn dialogue_advance_pressed() -> bool {
    pressed_label(&input_bindings().dialogue.advance)
        || pressed_label(&input_bindings().dialogue.advance_alternate)
}

pub(crate) fn fullscreen_pressed() -> bool {
    pressed_label(&input_bindings().global.fullscreen)
}

pub(crate) fn interact_pressed() -> bool {
    pressed_label(&input_bindings().global.interact)
}

pub(crate) fn journal_pressed() -> bool {
    pressed_label(&input_bindings().global.journal)
}

pub(crate) fn load_pressed() -> bool {
    pressed_label(&input_bindings().global.load)
}

pub(crate) fn movement_direction() -> Vec2 {
    let movement = &input_bindings().movement;
    let mut direction = Vec2::ZERO;
    if any_label_down(&movement.up) {
        direction.y -= 1.0;
    }
    if any_label_down(&movement.down) {
        direction.y += 1.0;
    }
    if any_label_down(&movement.left) {
        direction.x -= 1.0;
    }
    if any_label_down(&movement.right) {
        direction.x += 1.0;
    }
    direction
}

pub(crate) fn save_pressed() -> bool {
    pressed_label(&input_bindings().global.save)
}

pub(crate) fn select_next_pressed() -> bool {
    pressed_label(&input_bindings().navigation.select_next)
}

pub(crate) fn select_previous_pressed() -> bool {
    pressed_label(&input_bindings().navigation.select_previous)
}

pub(crate) fn switch_next_pressed() -> bool {
    pressed_label(&input_bindings().navigation.switch_next)
}

pub(crate) fn switch_previous_pressed() -> bool {
    pressed_label(&input_bindings().navigation.switch_previous)
}

fn pressed_label(label: &str) -> bool {
    let Some(key) = key_code_for_label(label) else {
        return false;
    };
    is_key_pressed(key)
}

fn any_label_down(labels: &[String]) -> bool {
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
