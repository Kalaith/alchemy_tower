use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::journal::{
    JournalHerbMemoriesView, JournalHerbMemoryView, JournalRouteProgressView, JournalRouteRowView,
    JournalRoutesTabView,
};

impl GameplayState {
    pub(super) fn journal_routes_tab_view(&self, data: &GameData) -> JournalRoutesTabView {
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
                .map(|route| JournalRouteRowView {
                    title: route.name.clone(),
                    detail: route.description.clone(),
                })
                .collect(),
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
                entries: Vec::new(),
            };
        }

        JournalHerbMemoriesView {
            title: ui_copy("overlay_herb_memories"),
            empty_text: String::new(),
            entries: herb_memories
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
                        summary: self.journal_herb_summary(data, &entry.item_id),
                        conditions: if entry.learned {
                            self.learned_gathering_conditions(data, &entry.item_id)
                                .unwrap_or_else(|| {
                                    ui_copy("journal_memory_conditions_unknown").to_owned()
                                })
                        } else {
                            ui_copy("journal_memory_conditions_unknown").to_owned()
                        },
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
                        variant_text: (!entry.variant_name.is_empty()).then(|| {
                            ui_format(
                                "journal_memory_variant",
                                &[("variant", &entry.variant_name)],
                            )
                        }),
                        note_text: (entry.learned && !entry.note.is_empty())
                            .then(|| entry.note.clone()),
                    }
                })
                .collect(),
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
    }
}
