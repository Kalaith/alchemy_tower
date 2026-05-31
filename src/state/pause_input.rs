use crate::input::{cancel_pressed, load_pressed, rect_clicked, save_pressed};
use crate::pause_layout::{
    load_pause_button_rect, pause_menu_button_rect, resume_pause_button_rect,
    save_pause_button_rect,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PauseAction {
    Resume,
    Save,
    Load,
    ReturnToMenu,
}

pub(super) fn selected_pause_action() -> Option<PauseAction> {
    if cancel_pressed() || rect_clicked(resume_pause_button_rect()) {
        return Some(PauseAction::Resume);
    }

    if save_pressed() || rect_clicked(save_pause_button_rect()) {
        return Some(PauseAction::Save);
    }

    if load_pressed() || rect_clicked(load_pause_button_rect()) {
        return Some(PauseAction::Load);
    }

    rect_clicked(pause_menu_button_rect()).then_some(PauseAction::ReturnToMenu)
}
