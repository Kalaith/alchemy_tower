use super::GameplayState;
use crate::data::{GameData, ItemCategory};

impl GameplayState {
    pub(super) fn sorted_inventory_items(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .inventory
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, false);
        items
    }

    pub(super) fn sell_price(&self, data: &GameData, item_id: &str) -> u32 {
        let Some(item) = data.item(item_id) else {
            return 0;
        };
        let base = if item.category == ItemCategory::Potion {
            item.base_value + (item.base_value / 4).max(1)
        } else {
            item.base_value
        };
        // What a counter pays depends on what is in the bottle. A flat price
        // meant a Masterwork and a Crude brew of the same recipe fetched the
        // same coin, so quality was worth nothing outside a quest gate.
        self.quality_adjusted_value(data, item_id, base)
    }

    pub(super) fn quick_potions(&self, data: &GameData) -> Vec<String> {
        let mut potions = self
            .inventory
            .iter()
            .filter(|(item_id, amount)| {
                **amount > 0
                    && data
                        .item(item_id)
                        .map(|item| item.category == ItemCategory::Potion)
                        .unwrap_or(false)
            })
            .map(|(item_id, _)| item_id.clone())
            .collect::<Vec<_>>();
        potions.sort_by(|left, right| {
            let left_value = data.item(left).map(|item| item.base_value).unwrap_or(0);
            let right_value = data.item(right).map(|item| item.base_value).unwrap_or(0);
            right_value.cmp(&left_value).then(left.cmp(right))
        });
        potions
    }

    pub(super) fn sell_candidates(&self, data: &GameData) -> Vec<String> {
        let mut items = self
            .sorted_inventory_items(data)
            .into_iter()
            .filter(|item_id| self.active_quest_reference_count(data, item_id) == 0)
            .collect::<Vec<_>>();
        self.sort_item_ids(data, &mut items, true);
        items
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::GameData;

    #[test]
    fn the_belt_offers_potions_dearest_first_and_nothing_else() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        state.inventory.insert("healing_draught".to_owned(), 1);
        state.inventory.insert("glow_potion".to_owned(), 2);
        state.inventory.insert("sunleaf".to_owned(), 5);

        let potions = state.quick_potions(&data);

        assert_eq!(
            potions,
            vec!["glow_potion".to_owned(), "healing_draught".to_owned()]
        );
    }

    /// A counter must never pay more for a thing than it charges for one.
    ///
    /// The apothecary sold a starlight shard for 28 and bought it back for 33:
    /// five coins a click, unbounded, no travel and no cost but the keypress.
    /// The shard's authored quality of 62 puts it in the Excellent band, which
    /// pays 140% — so the value multipliers the sell-price work introduced
    /// quietly turned one hand-authored price into a faucet, at exactly the
    /// moment the commissions gave coins somewhere to go.
    ///
    /// Prices stay hand-authored; this only asks that each one clears what the
    /// counter would pay back.
    #[test]
    fn no_counter_pays_more_for_a_thing_than_it_charges() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let mut faucets = Vec::new();

        for station in &data.stations {
            for stocked in &station.stock {
                // A bought unit carries no batch, so it sells for what the item
                // itself is worth — which is exactly what a buy-and-sell loop
                // would be trading.
                state.inventory.insert(stocked.item_id.clone(), 1);
                let paid_back = state.sell_price(&data, &stocked.item_id);
                state.take_from_inventory(&stocked.item_id, 1);
                if paid_back >= stocked.price {
                    faucets.push(format!(
                        "{}: {} costs {} and sells back for {paid_back}",
                        station.id, stocked.item_id, stocked.price
                    ));
                }
            }
        }

        assert!(
            faucets.is_empty(),
            "counters that pay more than they charge:
{faucets:#?}"
        );
    }

    #[test]
    fn drinking_a_potion_spends_the_bottle_and_starts_its_effects() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        state.inventory.insert("glow_potion".to_owned(), 1);

        state.consume_potion(&data, "glow_potion");

        assert!(!state.inventory.contains_key("glow_potion"));
        assert!(!state.runtime.active_effects.is_empty());
    }
}
