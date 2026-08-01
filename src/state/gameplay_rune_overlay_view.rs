use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::view_models::rune::{RuneOverlayEntry, RuneOverlayView};

/// Cards are 64px apart inside a section box that leaves room for about this
/// many before they would run past it and over the footer. The drafts list grew
/// past that as soon as the workbench had more than a handful of patterns.
const VISIBLE_RUNE_ROWS: usize = 5;

impl GameplayState {
    pub(super) fn rune_overlay_view(&self, data: &GameData) -> Option<RuneOverlayView> {
        let station = self.nearby_station(data)?;
        let recipes = self.available_rune_recipes(data, station);
        let total = recipes.len();
        let start = visible_window_start(self.ui.rune_index, total, VISIBLE_RUNE_ROWS);
        let range_text = (total > VISIBLE_RUNE_ROWS).then(|| {
            ui_format(
                "overlay_rune_range",
                &[
                    ("first", &(start + 1).to_string()),
                    ("last", &(start + VISIBLE_RUNE_ROWS).min(total).to_string()),
                    ("total", &total.to_string()),
                ],
            )
        });
        let entries = recipes
            .into_iter()
            .enumerate()
            .skip(start)
            .take(VISIBLE_RUNE_ROWS)
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
            range_text,
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

#[cfg(test)]
mod tests {
    /// A draft row gives its description one line and truncates the rest with
    /// an ellipsis. Measured off a capture: the nine original drafts run 55 to
    /// 83 characters and read in full, while a 141-character line came back cut
    /// mid-sentence. The budget is the house style, not the truncation point —
    /// a row that only just fits is a row that stops fitting when the font or
    /// the panel width next changes.
    const RUNE_DESCRIPTION_BUDGET: usize = 120;

    #[test]
    fn every_rune_draft_description_reads_in_full() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let overlong = data
            .rune_recipes
            .iter()
            .filter(|recipe| recipe.description.chars().count() > RUNE_DESCRIPTION_BUDGET)
            .map(|recipe| {
                format!(
                    "{}: {} chars",
                    recipe.id,
                    recipe.description.chars().count()
                )
            })
            .collect::<Vec<_>>();

        assert!(
            overlong.is_empty(),
            "draft rows that will be cut off mid-sentence: {overlong:#?}"
        );
    }
}
