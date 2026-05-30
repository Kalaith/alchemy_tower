use super::GameplayState;
use crate::content::{ui_copy, ui_format};
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
                title: format!(
                    "{} -> {}",
                    data.item_name(&recipe.input_item_id),
                    data.item_name(&recipe.output_item_id)
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
            empty_text: self.unavailable_state_text(ui_copy("overlay_rune_empty")),
            entries,
        })
    }
}
