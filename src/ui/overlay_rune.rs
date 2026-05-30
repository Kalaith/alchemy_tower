use super::{draw_overlay_section_box, draw_overlay_section_title, GameplayState};
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::ui::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_selection_card, draw_state_banner,
};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_rune_overlay(&self, data: &GameData) {
        let Some(station) = self.nearby_station(data) else {
            return;
        };
        draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, &station.name);
        draw_overlay_subtitle(x, y, &ui_text().overlays.rune_subtitle);
        let recipes = self.available_rune_recipes(data, station);
        draw_overlay_section_title(x + 20.0, y + 124.0, ui_copy("overlay_rune_drafts"), None);
        draw_overlay_section_box(x + 20.0, y + 138.0, w - 40.0, h - 200.0);
        let mut row_y = y + 172.0;
        if recipes.is_empty() {
            draw_state_banner(
                x + 32.0,
                row_y - 16.0,
                w - 64.0,
                &self.unavailable_state_text(ui_copy("overlay_rune_empty")),
                false,
            );
        } else {
            for (index, recipe) in recipes.iter().enumerate() {
                let selected = index == self.ui.rune_index;
                draw_selection_card(
                    x + 32.0,
                    row_y - 24.0,
                    w - 64.0,
                    58.0,
                    selected,
                    true,
                    &format!(
                        "{} -> {}",
                        data.item_name(&recipe.input_item_id),
                        data.item_name(&recipe.output_item_id)
                    ),
                    &recipe.description,
                    &ui_format(
                        "overlay_rune_label",
                        &[("item", data.item_name(&recipe.rune_item_id))],
                    ),
                );
                row_y += 64.0;
            }
        }
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_copy("overlay_rune_footer")
                .replace("{select}", &input_bindings().navigation.select)
                .replace("{confirm}", &input_bindings().global.confirm)
                .replace("{close}", &input_bindings().global.cancel),
        );
    }
}
