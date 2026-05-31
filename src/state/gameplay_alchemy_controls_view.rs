use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::view_models::alchemy::AlchemyControlsPanelView;

impl GameplayState {
    pub(super) fn alchemy_controls_panel_view(&self) -> AlchemyControlsPanelView {
        AlchemyControlsPanelView {
            title: ui_copy("overlay_alchemy_controls"),
            browse_heat_text: ui_format(
                "overlay_alchemy_controls_line1",
                &[
                    ("browse", &input_bindings().navigation.select),
                    ("heat", &input_bindings().alchemy.heat),
                ],
            ),
            action_text: ui_format(
                "overlay_alchemy_controls_line2",
                &[
                    ("fill", &input_bindings().alchemy.fill_slots),
                    ("catalyst", &input_bindings().alchemy.catalyst),
                    ("brew", &input_bindings().alchemy.brew),
                    ("brew_alt", &input_bindings().alchemy.brew_alternate),
                    ("repeat", &input_bindings().alchemy.repeat),
                    ("sort", &input_bindings().global.sort),
                    ("clear", &input_bindings().alchemy.clear),
                ],
            ),
        }
    }
}
