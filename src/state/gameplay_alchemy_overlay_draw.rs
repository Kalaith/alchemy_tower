use super::GameplayState;
use crate::alchemy_layout::{alchemy_close_rect, alchemy_panel_rect};
use crate::art::ArtAssets;
use crate::data::GameData;
use crate::ui::{
    draw_action_button, draw_alchemy_action_buttons, draw_brew_bubble_effect,
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
};

impl GameplayState {
    pub(super) fn draw_alchemy_overlay(&self, data: &GameData, art: &ArtAssets) {
        draw_overlay_backdrop();
        let chrome = self.alchemy_chrome_view();
        let panel = alchemy_panel_rect();
        let x = panel.x;
        let y = panel.y;
        let w = panel.w;
        let h = panel.h;
        draw_panel(x, y, w, h, chrome.title);
        draw_overlay_subtitle(x, y, &chrome.subtitle);
        draw_brew_bubble_effect(art, x, y, w);

        self.draw_alchemy_materials_panel(data, x, y);
        self.draw_alchemy_slots_panel(data, x, y, w);
        self.draw_alchemy_preview_panel(data, x, y, w, h);
        self.draw_alchemy_formulae_panel(data, x, y, h);

        draw_alchemy_action_buttons(&chrome.action_buttons, x, y);
        draw_action_button(alchemy_close_rect(), &chrome.close_label, 0.0);
        draw_overlay_footer(x, y, w, h, &chrome.footer_text);
    }
}
