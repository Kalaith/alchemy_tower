use super::gameplay_overlay_window::visible_window_start;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{
    JournalHerbMemoriesView, JournalHerbMemoryView, JournalHerbRowView, JournalRouteProgressView,
    JournalRouteRowView, JournalRoutesTabView,
};

/// Route rows are one line each, with the selected route's description beneath
/// them — the same shape as the herb column beside it. Drawn as title plus full
/// paragraph, the column had room for two of seventeen.
const VISIBLE_ROUTE_ROWS: usize = 7;

/// Herb rows are one line each and the block beneath belongs to the selected
/// row. Drawn at full detail for every herb, the column had room for one.
const VISIBLE_HERB_ROWS: usize = 5;

impl GameplayState {
    pub(super) fn journal_routes_tab_view(&self, data: &GameData) -> JournalRoutesTabView {
        let route_total = data.gathering_routes.len();
        // Routes ride the same index as the herb list rather than claiming a
        // second key: walking the herbs walks the routes past them too.
        let route_selected = self.ui.journal_index.min(route_total.saturating_sub(1));
        let route_start = visible_window_start(route_selected, route_total, VISIBLE_ROUTE_ROWS);
        let locked_lines = self
            .locked_warps(data)
            .into_iter()
            .take(2)
            .map(|warp| {
                ui_format(
                    "overlay_route_locked_line",
                    &[
                        ("label", &warp.label),
                        ("requirements", &self.warp_lock_text(data, warp)),
                    ],
                )
            })
            .collect::<Vec<_>>();

        JournalRoutesTabView {
            title: ui_copy("overlay_known_routes"),
            progress_title: ui_copy("overlay_progress_routes"),
            route_rows: data
                .gathering_routes
                .iter()
                .skip(route_start)
                .take(VISIBLE_ROUTE_ROWS)
                .enumerate()
                .map(|(offset, route)| JournalRouteRowView {
                    title: route.name.clone(),
                    selected: route_start + offset == route_selected,
                })
                .collect(),
            route_range_text: (route_total > VISIBLE_ROUTE_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(route_start + 1).to_string()),
                        (
                            "last",
                            &(route_start + VISIBLE_ROUTE_ROWS)
                                .min(route_total)
                                .to_string(),
                        ),
                        ("total", &route_total.to_string()),
                    ],
                )
            }),
            route_detail: data
                .gathering_routes
                .get(route_selected)
                .map(|route| route.description.clone()),
            herb_memories: self.journal_herb_memories_view(data),
            route_progress: JournalRouteProgressView {
                all_restored_text: locked_lines
                    .is_empty()
                    .then(|| ui_copy("overlay_routes_all_restored").to_owned()),
                locked_lines,
            },
        }
    }

    fn journal_herb_memories_view(&self, data: &GameData) -> JournalHerbMemoriesView {
        let herb_memories = self.herb_memories(data);
        if herb_memories.is_empty() {
            return JournalHerbMemoriesView {
                title: ui_copy("overlay_herb_memories"),
                empty_text: ui_copy("journal_memory_no_herbs").to_owned(),
                range_text: None,
                rows: Vec::new(),
                detail: None,
            };
        }

        let total = herb_memories.len();
        let selected = self.ui.journal_index.min(total - 1);
        let start = visible_window_start(selected, total, VISIBLE_HERB_ROWS);
        let entries = herb_memories
            .into_iter()
            .map(|entry| {
                let route_id = if entry.learned {
                    &entry.learned_route_id
                } else {
                    &entry.first_seen_route_id
                };
                let route_label = data
                    .route(route_id)
                    .map(|route| route.name.as_str())
                    .unwrap_or_else(|| ui_copy("journal_memory_unknown_place"));
                let route_copy_key = if entry.learned {
                    "journal_memory_learned_at"
                } else {
                    "journal_memory_observed_at"
                };
                JournalHerbMemoryView {
                    title: data.item_name(&entry.item_id).to_owned(),
                    state_line: ui_format(
                        "journal_memory_state_line",
                        &[("state", ui_copy(self.herb_memory_state_key(&entry.item_id)))],
                    ),
                    route_line: ui_format(route_copy_key, &[("route", route_label)]),
                    summary: self.journal_herb_lead(data, &entry.item_id),
                    // Learned means the conditions are known exactly. Short of
                    // that the entry carries what the valley says about the
                    // herb, which is enough to know when to go looking and not
                    // enough to save the trip.
                    conditions: if entry.learned {
                        self.learned_gathering_conditions(data, &entry.item_id)
                    } else {
                        self.heard_gathering_conditions(data, &entry.item_id)
                    }
                    .unwrap_or_else(|| ui_copy("journal_memory_conditions_unknown").to_owned()),
                    used_in_text: self.herb_used_in_text(data, &entry.item_id),
                    best_specimen_text: (entry.best_quality > 0).then(|| {
                        ui_format(
                            "journal_memory_best_specimen",
                            &[
                                ("quality", &entry.best_quality.to_string()),
                                ("band", &entry.best_quality_band),
                            ],
                        )
                    }),
                    // What was seen once, and — the part that decides whether to
                    // walk back out — whether one is in the bag now.
                    variant_text: self
                        .held_variant_summary(data, &entry.item_id)
                        .map(|(name, count)| {
                            ui_format(
                                "journal_memory_variant_held",
                                &[("variant", &name), ("count", &count.to_string())],
                            )
                        })
                        .or_else(|| {
                            (!entry.variant_name.is_empty()).then(|| {
                                ui_format(
                                    "journal_memory_variant",
                                    &[("variant", &entry.variant_name)],
                                )
                            })
                        }),
                    note_text: (entry.learned && !entry.note.is_empty())
                        .then(|| entry.note.clone()),
                }
            })
            .collect::<Vec<_>>();

        JournalHerbMemoriesView {
            title: ui_copy("overlay_herb_memories"),
            empty_text: String::new(),
            range_text: (total > VISIBLE_HERB_ROWS).then(|| {
                ui_format(
                    "journal_showing_range",
                    &[
                        ("first", &(start + 1).to_string()),
                        ("last", &(start + VISIBLE_HERB_ROWS).min(total).to_string()),
                        ("total", &total.to_string()),
                    ],
                )
            }),
            rows: entries
                .iter()
                .enumerate()
                .skip(start)
                .take(VISIBLE_HERB_ROWS)
                .map(|(index, entry)| JournalHerbRowView {
                    title: entry.title.clone(),
                    state_line: entry.state_line.clone(),
                    selected: index == selected,
                })
                .collect(),
            detail: entries.into_iter().nth(selected),
        }
    }

    /// Names the brews this ingredient feeds. Only recipes the player has
    /// discovered are named; still-unknown uses are counted, not spoiled, so
    /// the journal teaches what a gathered herb is *for* without giving away
    /// the whole catalogue.
    fn herb_used_in_text(&self, data: &GameData, item_id: &str) -> Option<String> {
        let mut known = Vec::new();
        let mut undiscovered = 0u32;
        for recipe in &data.recipes {
            if recipe
                .ingredients
                .iter()
                .any(|ingredient| ingredient.item_id == item_id)
            {
                if self.recipe_is_known(&recipe.id) {
                    known.push(recipe.name.clone());
                } else {
                    undiscovered += 1;
                }
            }
        }

        if known.is_empty() && undiscovered == 0 {
            return None;
        }
        if known.is_empty() {
            return Some(ui_copy("journal_memory_used_in_unknown").to_owned());
        }

        let recipes = known.join(", ");
        Some(if undiscovered > 0 {
            ui_format(
                "journal_memory_used_in_more",
                &[("recipes", &recipes), ("count", &undiscovered.to_string())],
            )
        } else {
            ui_format("journal_memory_used_in", &[("recipes", &recipes)])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    #[test]
    fn herb_usage_names_known_recipes_and_hides_undiscovered() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);

        // Whisper Moss feeds the starter Healing Draught (known at new game) plus
        // other formulae that are still discovery-only.
        let text = state
            .herb_used_in_text(&data, "whisper_moss")
            .expect("whisper moss is used in recipes");
        assert!(text.contains("Healing Draught"), "got: {text}");
        assert!(
            text.contains("discover"),
            "undiscovered uses hinted: {text}"
        );

        // Field Bloom is not in any starter recipe, so its uses read as
        // undiscovered rather than naming a formula.
        let field_bloom = state
            .herb_used_in_text(&data, "field_bloom")
            .expect("field bloom is used in recipes");
        assert!(!field_bloom.contains("Brews into:"), "got: {field_bloom}");

        // The southern pass herbs returned None here for two iterations, which
        // rendered as a blank where every other herb explains itself: nothing
        // brewed with them, so the journal had nothing to say. A herb the
        // player can pick should always be able to answer "what is this for".
        for herb_id in ["coldiron_lichen", "rimeflower"] {
            let text = state
                .herb_used_in_text(&data, herb_id)
                .unwrap_or_else(|| panic!("{herb_id} should read as used somewhere"));
            assert!(
                text.contains("discover"),
                "{herb_id} uses should hint at discovery: {text}"
            );
        }
    }
}

#[cfg(test)]
mod window_tests {
    use super::{GameplayState, VISIBLE_HERB_ROWS, VISIBLE_ROUTE_ROWS};

    fn seeded_state(data: &crate::data::GameData) -> GameplayState {
        let mut state = GameplayState::new(data);
        state.open_journal_sample(data);
        state
    }

    /// Both columns draw into fixed boxes with no scrollbar. Emitting more rows
    /// than fit is how this tab came to be hiding ten routes and twenty-odd
    /// herbs without saying so.
    #[test]
    fn neither_column_emits_more_rows_than_its_box_holds() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = seeded_state(&data);

        for index in 0..40 {
            state.ui.journal_index = index;
            let view = state.journal_routes_tab_view(&data);
            assert!(
                view.route_rows.len() <= VISIBLE_ROUTE_ROWS,
                "{} route rows at index {index}",
                view.route_rows.len()
            );
            assert!(
                view.herb_memories.rows.len() <= VISIBLE_HERB_ROWS,
                "{} herb rows at index {index}",
                view.herb_memories.rows.len()
            );
        }
    }

    /// Walking the list must actually reach the far end of it, and must always
    /// show the thing it says is selected.
    #[test]
    fn walking_the_list_keeps_the_selection_visible_and_reaches_the_end() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = seeded_state(&data);
        let herb_total = state.herb_memories(&data).len();
        assert!(
            herb_total > VISIBLE_HERB_ROWS,
            "this only proves anything with more herbs than fit"
        );

        let mut last_row_seen = String::new();
        for index in 0..herb_total {
            state.ui.journal_index = index;
            let view = state.journal_routes_tab_view(&data);
            assert!(
                view.herb_memories.rows.iter().any(|row| row.selected),
                "nothing selected at index {index}"
            );
            assert!(
                view.route_rows.iter().any(|row| row.selected),
                "no route selected at index {index}"
            );
            assert!(view.herb_memories.detail.is_some(), "no detail at {index}");
            if let Some(row) = view.herb_memories.rows.last() {
                last_row_seen = row.title.clone();
            }
        }

        state.ui.journal_index = 0;
        let first = state.journal_routes_tab_view(&data);
        let first_page_last = first
            .herb_memories
            .rows
            .last()
            .map(|row| row.title.clone())
            .unwrap_or_default();
        assert_ne!(
            last_row_seen, first_page_last,
            "walking to the end never moved past the first page"
        );
    }

    /// Measured off `screenshots/hud/journal_hearsay.png`: the 600-wide herb
    /// column wraps at about 88 characters per line at font 16. Deliberately
    /// generous — a long word wraps early and costs a line, so a real entry
    /// takes at least as many lines as this arithmetic says.
    const CHARS_PER_LINE: usize = 88;
    /// The shortest window the game is laid out for.
    const REFERENCE_SCREEN_HEIGHT: f32 = 720.0;

    /// The detail box has room for about four lines, and every entry was
    /// leading with a description that wraps to three of them — so the
    /// gathering conditions ran down through the Tower Access panel and the
    /// "brews into" line fell off the bottom without a mark to say so. Those
    /// two are the whole reason to open this tab.
    ///
    /// This checks the worst entry the content can produce still gets both,
    /// using the same layout numbers the renderer uses.
    #[test]
    fn every_herb_entry_gets_its_conditions_and_its_uses() {
        use crate::ui::{
            HERB_DETAIL_BLOCK_GAP, HERB_DETAIL_LINE_HEIGHT, HERB_DETAIL_TOP_GAP, HERB_LINE_STEP,
            HERB_ROW_STEP,
        };

        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = seeded_state(&data);
        // The panel is `journal_panel_rect()` at the reference height, and the
        // herb detail is bounded by `y + h - 170` with the rows above it.
        let panel_y = 72.0;
        let panel_h = REFERENCE_SCREEN_HEIGHT - 144.0;
        let bottom_limit = panel_y + panel_h - 170.0;
        let rows_end = panel_y + 136.0 + 32.0 + super::VISIBLE_HERB_ROWS as f32 * HERB_ROW_STEP;
        let block_height = |text: &str| {
            let lines = text.len().div_ceil(CHARS_PER_LINE).max(1) as f32;
            lines * HERB_DETAIL_LINE_HEIGHT + HERB_DETAIL_BLOCK_GAP
        };

        let total = state.herb_memories(&data).len();
        let mut clipped = Vec::new();
        for index in 0..total {
            state.ui.journal_index = index;
            let view = state.journal_routes_tab_view(&data);
            let Some(entry) = view.herb_memories.detail else {
                continue;
            };
            let mut y = rows_end + HERB_DETAIL_TOP_GAP + HERB_LINE_STEP;
            y += block_height(&entry.conditions);
            if let Some(used_in) = &entry.used_in_text {
                y += block_height(used_in);
            }
            if y > bottom_limit {
                clipped.push(format!(
                    "{}: needs {y:.0} against a {bottom_limit:.0} floor",
                    entry.title
                ));
            }
        }

        clipped.sort();
        clipped.dedup();
        assert!(
            clipped.is_empty(),
            "herb entries whose conditions or uses fall out of the box:
{clipped:#?}"
        );
    }

    #[test]
    fn the_counts_appear_only_when_something_is_out_of_sight() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let seeded = seeded_state(&data);
        let view = seeded.journal_routes_tab_view(&data);
        assert!(view.route_range_text.is_some(), "17 routes need a count");
        assert!(
            view.herb_memories.range_text.is_some(),
            "a full shelf needs a count"
        );

        let empty = GameplayState::new(&data);
        assert!(
            empty
                .journal_routes_tab_view(&data)
                .herb_memories
                .range_text
                .is_none(),
            "an empty shelf should not claim to be hiding anything"
        );
    }
}
