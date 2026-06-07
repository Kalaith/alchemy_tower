use super::gameplay_alchemy_types::ALCHEMY_TIMINGS;
use super::GameplayState;
use crate::data::{GameData, ItemCategory};

pub(super) struct AlchemyMaterialCard {
    pub(super) item_id: String,
    pub(super) selected: bool,
    pub(super) amount: u32,
    pub(super) ready: u32,
    pub(super) reserved: u32,
}

pub(super) struct AlchemyProcessSummary {
    pub(super) heat: i32,
    pub(super) stirs: u32,
    pub(super) timing: &'static str,
}

impl GameplayState {
    pub(super) fn alchemy_materials(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| {
                            item.category == ItemCategory::Ingredient
                                || item.category == ItemCategory::Catalyst
                        })
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, false);
        items
    }

    pub(super) fn alchemy_material_cards(&self, data: &GameData) -> Vec<AlchemyMaterialCard> {
        self.alchemy_materials(data)
            .into_iter()
            .enumerate()
            .map(|(index, item_id)| {
                let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                let reserved = self.reserved_count(&item_id);
                AlchemyMaterialCard {
                    item_id,
                    selected: index == self.alchemy.index,
                    amount,
                    ready: amount.saturating_sub(reserved),
                    reserved,
                }
            })
            .collect()
    }

    pub(super) fn alchemy_process_summary(&self) -> AlchemyProcessSummary {
        AlchemyProcessSummary {
            heat: self.alchemy.heat,
            stirs: self.alchemy.stirs,
            timing: self.alchemy_timing(),
        }
    }

    pub(super) fn alchemy_timing(&self) -> &'static str {
        ALCHEMY_TIMINGS[self.alchemy.timing_index]
    }
}
