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
            .map(|warp| format!("{}: {}", warp.label, self.warp_lock_text(data, warp)))
            .collect::<Vec<_>>();

        JournalRoutesTabView {
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
                empty_text: ui_copy("journal_memory_no_herbs").to_owned(),
                entries: Vec::new(),
            };
        }

        JournalHerbMemoriesView {
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
                            ui_format("journal_memory_variant", &[("variant", &entry.variant_name)])
                        }),
                        note_text: (entry.learned && !entry.note.is_empty())
                            .then(|| entry.note.clone()),
                    }
                })
                .collect(),
        }
    }
}
