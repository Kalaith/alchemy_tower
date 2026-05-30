use crate::input::{cancel_pressed, confirm_pressed, fullscreen_pressed};
use crate::menu_layout::{fullscreen_toggle_rect, settings_back_rect, title_button_rect};
use macroquad::prelude::Rect;
use macroquad_toolkit::input::was_clicked;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TitleAction {
    NewGame,
    LoadGame,
    Settings,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SettingsAction {
    Back,
    ToggleFullscreen,
}

pub(super) fn selected_title_action() -> Option<TitleAction> {
    if confirm_pressed() || clicked(title_button_rect(0)) {
        return Some(TitleAction::NewGame);
    }

    if clicked(title_button_rect(1)) {
        return Some(TitleAction::LoadGame);
    }

    clicked(title_button_rect(2)).then_some(TitleAction::Settings)
}

pub(super) fn selected_settings_action() -> Option<SettingsAction> {
    if cancel_pressed() || clicked(settings_back_rect()) {
        return Some(SettingsAction::Back);
    }

    (fullscreen_pressed() || clicked(fullscreen_toggle_rect()))
        .then_some(SettingsAction::ToggleFullscreen)
}

fn clicked(rect: Rect) -> bool {
    was_clicked(rect.x, rect.y, rect.w, rect.h)
}
