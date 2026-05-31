#[path = "input_keys.rs"]
mod input_keys;

use macroquad::prelude::{is_mouse_button_pressed, mouse_position, MouseButton, Rect, Vec2};
use macroquad_toolkit::input::was_clicked;

use crate::content::input_bindings;
use self::input_keys::{any_label_down, pressed_label};

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

pub(crate) fn left_mouse_pressed() -> bool {
    is_mouse_button_pressed(MouseButton::Left)
}

pub(crate) fn mouse_position_vec() -> Vec2 {
    mouse_position().into()
}

pub(crate) fn mouse_position_point() -> [f32; 2] {
    let (x, y) = mouse_position();
    [x, y]
}

pub(crate) fn rect_clicked(rect: Rect) -> bool {
    was_clicked(rect.x, rect.y, rect.w, rect.h)
}

pub(crate) fn rect_contains_point(rect: Rect, point: [f32; 2]) -> bool {
    rect.contains(Vec2::new(point[0], point[1]))
}

pub(crate) fn right_mouse_pressed() -> bool {
    is_mouse_button_pressed(MouseButton::Right)
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
