use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::view_models::alchemy::{AlchemyActionButtonsView, AlchemyChromeView};

impl GameplayState {
    pub(super) fn alchemy_chrome_view(&self) -> AlchemyChromeView {
        AlchemyChromeView {
            title: ui_copy("overlay_alchemy_title"),
            subtitle: ui_text().overlays.alchemy_subtitle.clone(),
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
