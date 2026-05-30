use crate::input::{cancel_pressed, load_pressed, save_pressed};
use crate::pause_layout::{
    load_pause_button_rect, pause_menu_button_rect, resume_pause_button_rect,
    save_pause_button_rect,
};
use macroquad::prelude::Rect;
use macroquad_toolkit::input::was_clicked;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PauseAction {
    Resume,
    Save,
    Load,
    ReturnToMenu,
}

pub(super) fn selected_pause_action() -> Option<PauseAction> {
    if cancel_pressed() || clicked(resume_pause_button_rect()) {
        return Some(PauseAction::Resume);
    }

    if save_pressed() || clicked(save_pause_button_rect()) {
        return Some(PauseAction::Save);
    }

    if load_pressed() || clicked(load_pause_button_rect()) {
        return Some(PauseAction::Load);
    }

    clicked(pause_menu_button_rect()).then_some(PauseAction::ReturnToMenu)
}

fn clicked(rect: Rect) -> bool {
    was_clicked(rect.x, rect.y, rect.w, rect.h)
}
