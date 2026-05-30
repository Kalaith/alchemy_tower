use super::gameplay_duplication::duplication_cost;
use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::archive::{
    ArchiveDuplicationDetailView, ArchiveDuplicationItemEntry, ArchiveDuplicationSectionView,
};

impl GameplayState {
    pub(super) fn archive_duplication_section_view(
        &self,
        data: &GameData,
    ) -> ArchiveDuplicationSectionView {
        let item_ids = self.duplication_candidates(data);
        if item_ids.is_empty() {
            return ArchiveDuplicationSectionView {
                empty_text: self.unavailable_state_text(
                    ui_copy("overlay_archive_empty_duplication"),
                ),
                entries: Vec::new(),
                detail: None,
            };
        }

        let selected_index = self.archive_selected_index(item_ids.len());
        let entries = item_ids
            .iter()
            .take(6)
            .enumerate()
            .filter_map(|(index, item_id)| {
                let item = data.item(item_id)?;
                Some(ArchiveDuplicationItemEntry {
                    title: data.item_name(item_id).to_owned(),
                    detail: self.inventory_reference_summary(data, item_id),
                    meta: ui_format(
                        "overlay_archive_owned_cost",
                        &[
                            (
                                "count",
                                &self
                                    .inventory
                                    .get(item_id)
                                    .copied()
                                    .unwrap_or_default()
                                    .to_string(),
                            ),
                            ("cost", &duplication_cost(item).to_string()),
                        ],
                    ),
                    enabled: self.can_duplicate_item(data, item_id),
                    selected: index == selected_index,
                })
            })
            .collect();

        let detail = item_ids
            .get(selected_index)
            .and_then(|item_id| data.item(item_id))
            .map(|item| ArchiveDuplicationDetailView {
                target_text: ui_format("overlay_target", &[("item", &item.name)]),
                coin_text: ui_format("overlay_coins", &[("count", &duplication_cost(item).to_string())]),
                catalyst_text: ui_format(
                    "overlay_archive_duplication_catalyst",
                    &[(
                        "item",
                        self.duplication_catalyst_item_id(data)
                            .as_deref()
                            .map(|id| data.item_name(id))
                            .unwrap_or(ui_copy("overlay_archive_duplication_catalyst_required")),
                    )],
                ),
            });

        ArchiveDuplicationSectionView {
            empty_text: String::new(),
            entries,
            detail,
        }
    }
}
