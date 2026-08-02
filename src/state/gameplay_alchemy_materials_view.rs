use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::alchemy::{AlchemyMaterialRowView, AlchemyMaterialsPanelView};

impl GameplayState {
    pub(super) fn alchemy_materials_panel_view(
        &self,
        data: &GameData,
    ) -> AlchemyMaterialsPanelView {
        let sort_label = self.inventory_sort_label();
        AlchemyMaterialsPanelView {
            title: ui_copy("overlay_materials"),
            sort_text: ui_format("overlay_sort_mode", &[("mode", sort_label)]),
            empty_text: self.unavailable_state_text(ui_copy("overlay_alchemy_empty_materials")),
            rows: self
                .alchemy_material_cards(data)
                .into_iter()
                .map(|card| {
                    let reference = self.inventory_reference_summary(data, &card.item_id);
                    let extra = ui_format(
                        "overlay_materials_meta",
                        &[
                            ("ready", &card.ready.to_string()),
                            ("reserved", &card.reserved.to_string()),
                            ("reference", &reference),
                        ],
                    );
                    AlchemyMaterialRowView {
                        // A held wild variant is worth more in the pot and the
                        // bench spends it first, and the belt shows one stack
                        // per id — so until now nothing on screen said which
                        // stacks had one in them. The meta column is about a
                        // dozen characters wide, so the mark is a mark.
                        title: match self.best_held_variant(data, &card.item_id) {
                            Some(_) => ui_format(
                                "overlay_materials_variant_held",
                                &[("name", data.item_name(&card.item_id))],
                            ),
                            None => data.item_name(&card.item_id).to_owned(),
                        },
                        meta: self.item_card_meta_at_quality(
                            data,
                            &card.item_id,
                            card.amount,
                            &extra,
                            self.reagent_quality(data, &card.item_id),
                        ),
                        selected: card.selected,
                        enabled: card.ready > 0,
                    }
                })
                .collect(),
        }
    }
}
