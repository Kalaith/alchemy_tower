use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{ui_copy, ui_text};
use crate::data::GameData;
use super::GameplayState;
use crate::ui::{
    draw_action_button, draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle,
    draw_panel,
};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_alchemy_overlay(&self, data: &GameData, art: &ArtAssets) {
        draw_overlay_backdrop();
        let x = 80.0;
        let y = 64.0;
        let w = screen_width() - 160.0;
        let h = screen_height() - 128.0;
        draw_panel(x, y, w, h, ui_copy("overlay_alchemy_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.alchemy_subtitle);
        if let Some(texture) = art.effect("brew_bubble_effect") {
            let alpha = 0.55 + ((get_time() as f32 * 2.4).sin() * 0.5 + 0.5) * 0.25;
            draw_texture_centered(
                texture,
                vec2(x + w - 54.0, y + 44.0),
                vec2(42.0, 42.0),
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }

        self.draw_alchemy_materials_panel(data, x, y);

        self.draw_alchemy_controls_panel(x, y);

        self.draw_alchemy_slots_panel(data, x, y, w);

        self.draw_alchemy_preview_panel(data, x, y, w);

        self.draw_alchemy_formulae_panel(data, x, y);

        draw_action_button(
            Rect::new(x + 20.0, y + 368.0, 82.0, 28.0),
            ui_copy("overlay_alchemy_sort_button"),
            0.0,
        );
        draw_action_button(
            Rect::new(x + 114.0, y + 368.0, 82.0, 28.0),
            ui_copy("overlay_alchemy_clear_button"),
            0.0,
        );
        draw_action_button(
            Rect::new(x + 208.0, y + 368.0, 90.0, 28.0),
            ui_copy("overlay_alchemy_repeat_button"),
            0.0,
        );
        draw_rectangle(
            x + 310.0,
            y + 368.0,
            90.0,
            28.0,
            Color::from_rgba(38, 58, 46, 210),
        );
        draw_rectangle_lines(
            x + 310.0,
            y + 368.0,
            90.0,
            28.0,
            1.5,
            Color::from_rgba(188, 255, 220, 96),
        );
        draw_text(
            ui_copy("overlay_alchemy_brew_button"),
            x + 338.0,
            y + 388.0,
            18.0,
            dark::TEXT_BRIGHT,
        );

        draw_overlay_footer(x, y, w, h, ui_copy("overlay_alchemy_mouse_footer"));
    }
}
