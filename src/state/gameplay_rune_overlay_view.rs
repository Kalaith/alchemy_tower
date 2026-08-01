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

/// First row of the window that keeps `selected` visible. Clamping to the first
/// `window` entries instead would silently make every later pattern unreachable,
/// which is the failure this list was one recipe away from having.
fn visible_window_start(selected: usize, total: usize, window: usize) -> usize {
    if total <= window {
        return 0;
    }
    selected.saturating_sub(window - 1).min(total - window)
}

#[cfg(test)]
mod tests {
    use super::visible_window_start;

    #[test]
    fn the_selected_draft_is_always_inside_the_window() {
        let window = 5;
        for total in 1..40usize {
            for selected in 0..total {
                let start = visible_window_start(selected, total, window);
                assert!(
                    selected >= start && selected < start + window,
                    "selection {selected} of {total} fell outside window at {start}"
                );
                assert!(
                    start + window <= total.max(window),
                    "window ran past the list"
                );
            }
        }
    }

    #[test]
    fn a_short_list_never_scrolls() {
        assert_eq!(visible_window_start(0, 3, 5), 0);
        assert_eq!(visible_window_start(2, 3, 5), 0);
    }
}
