use super::GameplayState;
use crate::data::{GameData, ItemCategory, ItemDefinition};

#[path = "gameplay_duplication_text.rs"]
mod duplication_text;

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
            let left_cost = self.duplication_cost_for_item(data, left);
            let right_cost = self.duplication_cost_for_item(data, right);
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
            && self.coins >= self.duplication_cost_for_item(data, item_id)
            && self.duplication_catalyst_item_id(data).is_some()
    }

    pub(super) fn duplicate_item(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            self.runtime.status_text = duplication_text::unstable();
            return;
        };
        if !duplication_item_allowed(item) {
            self.runtime.status_text = duplication_text::resists(&item.name);
            return;
        }
        if self.inventory.get(item_id).copied().unwrap_or_default() == 0 {
            self.runtime.status_text = duplication_text::missing_source(&item.name);
            return;
        }

        let cost = self.duplication_cost_for_item(data, item_id);
        if self.coins < cost {
            self.runtime.status_text =
                duplication_text::need_coins(&item.name, cost.saturating_sub(self.coins));
            return;
        }

        let Some(catalyst_item_id) = self.duplication_catalyst_item_id(data) else {
            self.runtime.status_text = duplication_text::need_catalyst();
            return;
        };

        self.coins = self.coins.saturating_sub(cost);
        self.take_from_inventory(&catalyst_item_id, 1);
        if item.category == ItemCategory::Potion {
            if !self.duplicate_worst_held_bottle(data, item_id) {
                self.runtime.status_text = duplication_text::missing_source(&item.name);
                return;
            }
        } else {
            *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        }
        self.note_inventory_observation(data, item_id);

        self.trigger_duplication_feedback(duplication_text::toast(&item.name));
        self.runtime.status_text =
            duplication_text::duplicated(data, &item.name, cost, &catalyst_item_id);
    }

    /// The starlight catalyst the console would spend.
    ///
    /// It used to take the *highest-quality* one held, and duplication reads
    /// nothing from a catalyst's quality — it is spent, not measured. So the
    /// console reached past a 24-coin shard you can buy at two counters for
    /// Mira's `counterkept_shard`, which is a friendship gift, sold nowhere and
    /// gathered nowhere, and burned it for exactly the same result. The
    /// planter had the same shape of bug and the same answer: take the least
    /// valuable thing that qualifies.
    pub(super) fn duplication_catalyst_item_id(&self, data: &GameData) -> Option<String> {
        self.inventory
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .filter_map(|(item_id, _)| {
                let item = data.item(item_id)?;
                item.catalyst_tags
                    .iter()
                    .any(|tag| tag == "starlight")
                    .then_some((item_id.clone(), item.base_value))
            })
            .min_by(|left, right| left.1.cmp(&right.1).then(left.0.cmp(&right.0)))
            .map(|entry| entry.0)
    }

    /// Exceptional bottles cost at least one coin more than the copy could be
    /// sold for. The catalyst remains an additional material cost, but this
    /// keeps the console from becoming a mint even when it copies Masterwork.
    pub(super) fn duplication_cost_for_item(&self, data: &GameData, item_id: &str) -> u32 {
        let Some(item) = data.item(item_id) else {
            return u32::MAX;
        };
        let base = duplication_cost(item);
        if item.category != ItemCategory::Potion {
            return base;
        }
        base.max(self.sell_price(data, item_id).saturating_add(1))
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
    use crate::data::{BottleBatchEntry, GameData};

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

    /// A copy must never be worth more than it cost to make.
    ///
    /// The band multipliers were being applied to raw materials as well as to
    /// brews, and a catalyst's `quality` is potency rather than craft — so
    /// Tarn's `elevenyear_amber` (quality 82, paying 200%) sold for 640 against
    /// a 360 duplication cost. Two more catalysts were the same shape. The
    /// console cannot be a mint: whatever the fix, this is the rule.
    #[test]
    fn a_copy_never_sells_for_more_than_it_cost_to_make() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let mut mints = Vec::new();

        for item in &data.items {
            if !super::duplication_item_allowed(item) {
                continue;
            }
            state.inventory.insert(item.id.clone(), 1);
            let paid_back = state.sell_price(&data, &item.id);
            state.take_from_inventory(&item.id, 1);
            let cost = super::duplication_cost(item);
            if paid_back >= cost {
                mints.push(format!(
                    "{}: copy costs {cost}, sells for {paid_back}",
                    item.id
                ));
            }
        }

        assert!(
            mints.is_empty(),
            "things the console can mint coins from:
{mints:#?}"
        );
    }

    /// Duplication reads nothing from a catalyst's quality — the shard is spent,
    /// not measured — and it used to take the *best* one held. Mira's
    /// `counterkept_shard` is a friendship gift, sold nowhere and gathered
    /// nowhere, and the console burned it in preference to a 24-coin shard two
    /// counters sell, for exactly the same result.
    #[test]
    fn duplication_spends_the_shard_you_can_replace() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        let gift = data
            .item("counterkept_shard")
            .expect("Mira's gift catalyst should exist");
        let stock = data
            .item("starlight_shard")
            .expect("the buyable starlight shard should exist");
        assert!(
            gift.quality > stock.quality && gift.base_value > stock.base_value,
            "the gift has to be both better and dearer for this to mean anything"
        );

        state.inventory.insert(gift.id.clone(), 1);
        state.inventory.insert(stock.id.clone(), 1);

        assert_eq!(
            state.duplication_catalyst_item_id(&data).as_deref(),
            Some(stock.id.as_str()),
            "the console reached past the replaceable shard for the gift"
        );
    }

    #[test]
    fn duplication_copies_the_worst_live_bottle_without_flattening_its_grade() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        state.coins = 999;
        state.inventory.insert("glow_potion".to_owned(), 2);
        state.inventory.insert("starlight_shard".to_owned(), 1);
        state.progression.bottle_stock.insert(
            "glow_potion".to_owned(),
            vec![
                BottleBatchEntry {
                    item_id: "glow_potion".to_owned(),
                    quality_score: 15,
                    quality_band: "Crude".to_owned(),
                    traits: vec!["faint".to_owned()],
                    count: 1,
                },
                BottleBatchEntry {
                    item_id: "glow_potion".to_owned(),
                    quality_score: 90,
                    quality_band: "Masterwork".to_owned(),
                    traits: vec!["luminous".to_owned()],
                    count: 1,
                },
            ],
        );

        state.duplicate_item(&data, "glow_potion");

        let batches = state.live_batches("glow_potion");
        assert_eq!(state.inventory["glow_potion"], 3);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].quality_score, 15);
        assert_eq!(batches[0].traits, vec!["faint"]);
        assert_eq!(batches[0].count, 2, "the Crude source was not the copy");
        assert_eq!(batches[1].quality_score, 90);
        assert_eq!(batches[1].count, 1);
    }

    #[test]
    fn even_a_masterwork_copy_costs_more_than_it_can_be_sold_for() {
        let data = GameData::fallback();
        let mut state = GameplayState::new(&data);
        state.inventory.insert("glow_potion".to_owned(), 1);
        state.progression.bottle_stock.insert(
            "glow_potion".to_owned(),
            vec![BottleBatchEntry {
                item_id: "glow_potion".to_owned(),
                quality_score: 90,
                quality_band: "Masterwork".to_owned(),
                traits: vec!["luminous".to_owned()],
                count: 1,
            }],
        );

        let sale = state.sell_price(&data, "glow_potion");
        let cost = state.duplication_cost_for_item(&data, "glow_potion");
        assert!(cost > sale, "copy costs {cost}, but sells for {sale}");
    }
}
