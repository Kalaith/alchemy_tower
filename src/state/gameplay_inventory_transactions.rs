use super::gameplay_support::quality_band_rank;
use super::GameplayState;
use crate::data::{EffectDefinition, EffectKind, GameData};

#[path = "gameplay_inventory_transaction_text.rs"]
mod transaction_text;

impl GameplayState {
    pub(super) fn consume_potion(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            return;
        };
        if self.inventory.get(item_id).copied().unwrap_or_default() == 0 {
            return;
        }
        let Some(bottle) = self.worst_held_bottle(data, item_id) else {
            return;
        };
        let effect_percent = data
            .config
            .balance
            .quality_effect_percent
            .for_rank(quality_band_rank(&bottle.quality_band));
        self.take_from_inventory(item_id, 1);
        for effect in &item.effects {
            self.apply_effect(&effect_scaled_for_quality(effect, effect_percent));
        }
        self.runtime.status_text = transaction_text::potion_used(&item.name);
    }

    pub(super) fn buy_item(&mut self, data: &GameData, item_id: &str, price: u32) {
        if self.coins < price {
            self.runtime.status_text = transaction_text::not_enough_coins(data, item_id);
            return;
        }
        self.coins -= price;
        *self.inventory.entry(item_id.to_owned()).or_insert(0) += 1;
        self.note_inventory_observation(data, item_id);
        self.runtime.status_text = transaction_text::bought(data, item_id);
    }

    pub(super) fn sell_item(&mut self, data: &GameData, item_id: &str) {
        let Some(item) = data.item(item_id) else {
            return;
        };
        let price = self.sell_price(data, item_id);
        if self.inventory.get(item_id).copied().unwrap_or_default() == 0 {
            return;
        }
        self.take_from_inventory(item_id, 1);
        self.coins += price;
        self.runtime.status_text = transaction_text::sold(&item.name, price);
        if self.sell_is_safe(data, item_id) {
            self.trigger_safe_sale_feedback(transaction_text::sold_safe(&item.name, price));
        }
    }
}

/// Quality improves the useful work promised by the alchemy design. The
/// authored numbers are a Serviceable bottle's baseline: restoration scales
/// in magnitude, and positive timed effects scale in duration. Misfire is a
/// failed brew, not a buff, so better ingredients never make its penalty last
/// longer.
fn effect_scaled_for_quality(effect: &EffectDefinition, percent: u32) -> EffectDefinition {
    let mut scaled = effect.clone();
    let multiplier = percent as f32 / 100.0;
    match effect.kind {
        EffectKind::Restore => scaled.magnitude *= multiplier,
        EffectKind::Speed | EffectKind::Glow => scaled.duration_seconds *= multiplier,
        EffectKind::Misfire => {}
    }
    scaled
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{BottleBatchEntry, EffectKind, GameData};

    fn stock_bottle(
        state: &mut GameplayState,
        item_id: &str,
        quality_score: u32,
        quality_band: &str,
    ) {
        state.inventory.insert(item_id.to_owned(), 1);
        state.progression.bottle_stock.insert(
            item_id.to_owned(),
            vec![BottleBatchEntry {
                item_id: item_id.to_owned(),
                quality_score,
                quality_band: quality_band.to_owned(),
                traits: Vec::new(),
                count: 1,
            }],
        );
    }

    fn drink_restore(data: &GameData, quality_score: u32, quality_band: &str) -> f32 {
        let mut state = GameplayState::new(data);
        state.vitality = 0.0;
        stock_bottle(&mut state, "healing_draught", quality_score, quality_band);
        state.consume_potion(data, "healing_draught");
        state.vitality
    }

    fn drink_timed(
        data: &GameData,
        item_id: &str,
        kind: EffectKind,
        quality_score: u32,
        quality_band: &str,
    ) -> f32 {
        let mut state = GameplayState::new(data);
        stock_bottle(&mut state, item_id, quality_score, quality_band);
        state.consume_potion(data, item_id);
        state
            .runtime
            .active_effects
            .iter()
            .find(|effect| effect.kind == kind)
            .map(|effect| effect.remaining_seconds)
            .expect("drinking should start the authored timed effect")
    }

    #[test]
    fn a_better_healing_bottle_restores_more_of_the_day() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let crude = drink_restore(&data, 10, "Crude");
        let masterwork = drink_restore(&data, 95, "Masterwork");

        assert!((crude - 15.0).abs() < f32::EPSILON);
        assert!((masterwork - 30.0).abs() < f32::EPSILON);
        assert!(masterwork > crude);
    }

    #[test]
    fn a_better_positive_brew_lasts_longer_without_lengthening_a_misfire() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let crude_glow = drink_timed(&data, "glow_potion", EffectKind::Glow, 10, "Crude");
        let masterwork_glow = drink_timed(&data, "glow_potion", EffectKind::Glow, 95, "Masterwork");
        assert!((crude_glow - 67.5).abs() < f32::EPSILON);
        assert!((masterwork_glow - 135.0).abs() < f32::EPSILON);
        assert!(masterwork_glow > crude_glow);

        let crude_misfire =
            drink_timed(&data, "murky_concoction", EffectKind::Misfire, 10, "Crude");
        let masterwork_misfire = drink_timed(
            &data,
            "murky_concoction",
            EffectKind::Misfire,
            95,
            "Masterwork",
        );
        assert!((crude_misfire - masterwork_misfire).abs() < f32::EPSILON);
    }
}
