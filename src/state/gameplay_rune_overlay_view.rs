use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::view_models::rune::{RuneOverlayEntry, RuneOverlayView};

impl GameplayState {
    pub(super) fn rune_overlay_view(&self, data: &GameData) -> Option<RuneOverlayView> {
        let station = self.nearby_station(data)?;
        let entries = self
            .available_rune_recipes(data, station)
            .into_iter()
            .enumerate()
            .map(|(index, recipe)| RuneOverlayEntry {
                title: ui_format(
                    "overlay_rune_recipe_title",
                    &[
                        ("input", data.item_name(&recipe.input_item_id)),
                        ("output", data.item_name(&recipe.output_item_id)),
                    ],
                ),
                detail: recipe.description.clone(),
                meta: ui_format(
                    "overlay_rune_label",
                    &[("item", data.item_name(&recipe.rune_item_id))],
                ),
                selected: self.rune_recipe_selected(index),
            })
            .collect();

        Some(RuneOverlayView {
            station_name: station.name.clone(),
            subtitle: ui_text().overlays.rune_subtitle.clone(),
            drafts_title: ui_copy("overlay_rune_drafts").to_owned(),
            empty_text: self.unavailable_state_text(ui_copy("overlay_rune_empty")),
            footer_text: rune_footer_text(),
            entries,
        })
    }
}

fn rune_footer_text() -> String {
    ui_copy("overlay_rune_footer")
        .replace("{select}", &input_bindings().navigation.select)
        .replace("{confirm}", &input_bindings().global.confirm)
        .replace("{close}", &input_bindings().global.cancel)
}
