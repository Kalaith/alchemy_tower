use super::GameplayState;
use crate::content::ui_format;
use crate::data::{GameData, ItemCategory, ItemDefinition};
use macroquad::prelude::Color;

impl GameplayState {
    pub(super) fn duplication_candidates(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(duplication_item_allowed)
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            let left_cost = data.item(left).map(duplication_cost).unwrap_or(u32::MAX);
            let right_cost = data.item(right).map(duplication_cost).unwrap_or(u32::MAX);
            left_cost
                .cmp(&right_cost)
                .then(data.item_name(left).cmp(data.item_name(right)))
        });
        items
    }

    pub(super) fn can_duplicate_item(&self, data: &GameData, item_id: &str) -> bool {
        let Some(item) = data.item(item_id) else {
            return false;
        };
        duplication_item_allowed(item)
            && self.inventory.get(item_id).copied().unwrap_or_default() > 0
            && self.coins >= duplication_cost(item)
            && self.duplication_catalyst_item_id(data).is_some()
    }

    pub(super) fn duplicate_item(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            self.runtime.status_text = ui_format("progression_duplicate_unstable", &[]);
            return;
        };
        if !duplication_item_allowed(item) {
            self.runtime.status_text =
                ui_format("progression_duplicate_resists", &[("name", &item.name)]);
            return;
        }
        if self.inventory.get(item_id).copied().unwrap_or_default() == 0 {
            self.runtime.status_text =
                ui_format("progression_duplicate_none", &[("name", &item.name)]);
            return;
        }

        let cost = duplication_cost(item);
        if self.coins < cost {
            self.runtime.status_text = ui_format(
                "progression_duplicate_need_coins",
                &[
                    ("coins", &cost.saturating_sub(self.coins).to_string()),
                    ("name", &item.name),
                ],
            );
            return;
        }

        let Some(catalyst_item_id) = self.duplication_catalyst_item_id(data) else {
            self.runtime.status_text = ui_format("progression_duplicate_need_catalyst", &[]);
            return;
        };

        self.coins = self.coins.saturating_sub(cost);
        if let Some(amount) = self.inventory.get_mut(&catalyst_item_id) {
            *amount = amount.saturating_sub(1);
            if *amount == 0 {
                self.inventory.remove(&catalyst_item_id);
            }
        }
        *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        self.note_inventory_observation(data, item_id);

        self.push_event_toast_with_icon(
            ui_format("progression_duplicate_toast", &[("name", &item.name)]),
            Color::from_rgba(216, 182, 255, 255),
            "best_quality",
        );
        self.runtime.status_text = ui_format(
            "progression_duplicate_status",
            &[
                ("name", &item.name),
                ("cost", &cost.to_string()),
                ("catalyst", data.item_name(&catalyst_item_id)),
            ],
        );
    }

    pub(super) fn duplication_catalyst_item_id(&self, data: &GameData) -> Option<String> {
        self.inventory
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .filter_map(|(item_id, _)| {
                let item = data.item(item_id)?;
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == "starlight")
                    .then_some((item_id.clone(), item.quality))
            })
            .max_by(|left, right| left.1.cmp(&right.1).then(left.0.cmp(&right.0)))
            .map(|entry| entry.0)
    }
}

fn duplication_item_allowed(item: &ItemDefinition) -> bool {
    matches!(
        item.category,
        ItemCategory::Ingredient | ItemCategory::Catalyst | ItemCategory::Potion
    )
}

pub(super) fn duplication_cost(item: &ItemDefinition) -> u32 {
    item.base_value + u32::from(item.rarity) * 10
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::GameData;

    #[test]
    fn duplication_consumes_catalyst_and_coins() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);

        state.coins = 99;
        state.inventory.insert("glow_potion".to_owned(), 1);
        state.inventory.insert("starlight_shard".to_owned(), 1);

        state.duplicate_item(&data, "glow_potion");

        assert_eq!(
            state
                .inventory
                .get("glow_potion")
                .copied()
                .unwrap_or_default(),
            2
        );
        assert_eq!(
            state
                .inventory
                .get("starlight_shard")
                .copied()
                .unwrap_or_default(),
            0
        );
        assert_eq!(state.coins, 63);
    }
}
