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
                        title: data.item_name(&card.item_id).to_owned(),
                        detail: data
                            .item(&card.item_id)
                            .map(|item| item.description.clone())
                            .unwrap_or_default(),
                        meta: self.item_card_meta(data, &card.item_id, card.amount, &extra),
                        selected: card.selected,
                        enabled: card.ready > 0,
                    }
                })
                .collect(),
        }
    }
}
