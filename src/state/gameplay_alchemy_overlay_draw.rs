use super::GameplayState;
use crate::art::ArtAssets;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::ui::{
    draw_alchemy_action_buttons, draw_brew_bubble_effect, draw_overlay_backdrop, draw_overlay_footer,
    draw_overlay_subtitle, draw_panel,
};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_alchemy_overlay(&self, data: &GameData, art: &ArtAssets) {
        draw_overlay_backdrop();
        let x = 80.0;
        let y = 64.0;
        let w = screen_width() - 160.0;
        let h = screen_height() - 128.0;
        draw_panel(x, y, w, h, ui_copy("overlay_alchemy_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.alchemy_subtitle);
        draw_brew_bubble_effect(art, x, y, w);

        self.draw_alchemy_materials_panel(data, x, y);
        self.draw_alchemy_controls_panel(x, y);
        self.draw_alchemy_slots_panel(data, x, y, w);
        self.draw_alchemy_preview_panel(data, x, y, w);
        self.draw_alchemy_formulae_panel(data, x, y);

        draw_alchemy_action_buttons(x, y);
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_format(
                "overlay_alchemy_mouse_footer",
                &[("close", &input_bindings().global.cancel)],
            ),
        );
    }
}
