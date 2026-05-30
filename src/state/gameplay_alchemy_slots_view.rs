use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::GameData;
use crate::view_models::alchemy::{
    AlchemyCatalystSlotView, AlchemySlotView, AlchemySlotsPanelView,
};

impl GameplayState {
    pub(super) fn alchemy_slots_panel_view(&self, data: &GameData) -> AlchemySlotsPanelView {
        let process = self.alchemy_process_summary();
        AlchemySlotsPanelView {
            process_text: ui_format(
                "overlay_alchemy_process",
                &[
                    ("heat", &process.heat.to_string()),
                    ("stirs", &process.stirs.to_string()),
                    ("timing", process.timing),
                ],
            ),
            slots: self
                .alchemy_slot_items()
                .into_iter()
                .enumerate()
                .map(|(slot, item_id)| AlchemySlotView {
                    label: ui_format("overlay_slot_label", &[("slot", &(slot + 1).to_string())]),
                    item_name: item_id
                        .as_deref()
                        .map(|id| data.item_name(id).to_owned())
                        .unwrap_or_else(|| ui_copy("overlay_alchemy_empty_slot").to_owned()),
                    action_text: if item_id.is_some() {
                        ui_copy("overlay_slot_click_clear")
                    } else {
                        ui_copy("overlay_slot_click_fill")
                    },
                })
                .collect(),
            catalyst: AlchemyCatalystSlotView {
                item_name: self
                    .selected_catalyst()
                    .map(|id| data.item_name(id).to_owned())
                    .unwrap_or_else(|| ui_copy("overlay_alchemy_empty_slot").to_owned()),
                action_text: if self.selected_catalyst().is_some() {
                    ui_copy("overlay_catalyst_click_clear")
                } else {
                    ui_copy("overlay_catalyst_click_assign")
                },
            },
        }
    }
}
