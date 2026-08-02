use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::view_models::alchemy::{AlchemyActionButtonsView, AlchemyChromeView};

impl GameplayState {
    pub(super) fn alchemy_chrome_view(&self, data: &GameData) -> AlchemyChromeView {
        AlchemyChromeView {
            title: ui_copy("overlay_alchemy_title"),
            // What this room does for the work, in the room's own words. The
            // five authored `room_bonus.description` strings had no reader at
            // all, and the line they replace is the same sentence at every
            // bench in the tower — which is the one thing a bench subtitle
            // should never be, given the whole point of a deep floor is that
            // the room changes the brew.
            subtitle: self
                .nearby_station(data)
                .map(|station| station.room_bonus.description.clone())
                .filter(|description| !description.is_empty())
                .unwrap_or_else(|| ui_text().overlays.alchemy_subtitle.clone()),
            footer_text: ui_format(
                "overlay_alchemy_mouse_footer",
                &[("close", &input_bindings().global.cancel)],
            ),
            close_label: ui_copy("overlay_alchemy_close_button").to_string(),
            action_buttons: alchemy_action_buttons_view(),
        }
    }
}

fn alchemy_action_buttons_view() -> AlchemyActionButtonsView {
    AlchemyActionButtonsView {
        sort_label: ui_copy("overlay_alchemy_sort_button"),
        clear_label: ui_copy("overlay_alchemy_clear_button"),
        repeat_label: ui_copy("overlay_alchemy_repeat_button"),
        brew_label: ui_copy("overlay_alchemy_brew_button"),
    }
}
