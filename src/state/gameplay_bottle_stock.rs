//! What the bottles on the shelf are actually worth, as opposed to the best one
//! the player ever made.
//!
//! A request naming a minimum band used to be checked against
//! `crafted_item_profiles`, which records the high-water mark for an item id.
//! One Masterwork healing draught therefore satisfied every later request for a
//! Masterwork healing draught permanently, including ones filled with Crude
//! bottles brewed afterwards — the gate asked what the player was capable of
//! rather than what they were holding.
//!
//! Bottles now carry the quality and traits they were brewed at. Anything that
//! did not come off the bench — bought, gifted, granted at the start — has no
//! batch and counts as a plain example of its item, which is what it is.
//!
//! Bottles leave the shelf a dozen ways: sold, drunk, imbued, fed to a planter,
//! spent as a reagent. Rather than teaching every one of those about batches,
//! the batch list is *reconciled against the inventory count* whenever it is
//! read: anything the count no longer supports is dropped, worst first, so the
//! player keeps their best. A stale batch can therefore never re-grade a bottle
//! that replaced it.

use super::gameplay_quest_requirements::{trait_requirement_met, trait_requirement_target};
use super::gameplay_support::quality_band_rank;
use super::GameplayState;
use crate::alchemy::{quality_band, BrewResolution};
use crate::data::{BottleBatchEntry, GameData, QuestDefinition};

impl GameplayState {
    /// File the bottles a brew just produced, so what they are worth travels
    /// with them instead of being flattened into a best-ever record.
    pub(super) fn record_bottle_batch(&mut self, resolution: &BrewResolution<'_>) {
        let batches = self
            .progression
            .bottle_stock
            .entry(resolution.output_item_id.clone())
            .or_default();
        // Identical bottles merge, so brewing the same thing forty times keeps
        // one row rather than forty.
        if let Some(existing) = batches.iter_mut().find(|batch| {
            batch.quality_score == resolution.quality_score
                && batch.traits == resolution.inherited_traits
        }) {
            existing.count = existing.count.saturating_add(resolution.output_amount);
        } else {
            batches.push(BottleBatchEntry {
                item_id: resolution.output_item_id.clone(),
                quality_score: resolution.quality_score,
                quality_band: resolution.quality_band.to_owned(),
                traits: resolution.inherited_traits.clone(),
                count: resolution.output_amount,
            });
        }
        batches.sort_by_key(|batch| batch.quality_score);
    }

    /// Take bottles off the shelf: the one way anything leaves the inventory.
    ///
    /// Every path that removes stock goes through here — sold, drunk, imbued,
    /// fed to a planter, spent as a reagent, handed to a townsperson — so that
    /// the batch list cannot outlive the bottles it describes. Trimming lazily
    /// on read is not enough: a bottle sold and replaced by a bought one leaves
    /// the count where it started, and the dead batch would re-grade the
    /// replacement.
    pub(super) fn take_from_inventory(&mut self, item_id: &str, amount: u32) {
        if let Some(held) = self.inventory.get_mut(item_id) {
            *held = held.saturating_sub(amount);
        }
        self.inventory.retain(|_, held| *held > 0);
        self.reconcile_bottle_stock(item_id);
    }

    /// Drop whatever the inventory count no longer supports, worst first.
    pub(super) fn reconcile_bottle_stock(&mut self, item_id: &str) {
        let held = self.inventory.get(item_id).copied().unwrap_or_default();
        let Some(batches) = self.progression.bottle_stock.get_mut(item_id) else {
            return;
        };
        let mut tracked = batches.iter().map(|batch| batch.count).sum::<u32>();
        for batch in batches.iter_mut() {
            if tracked <= held {
                break;
            }
            let excess = tracked - held;
            let dropped = batch.count.min(excess);
            batch.count -= dropped;
            tracked -= dropped;
        }
        batches.retain(|batch| batch.count > 0);
        if batches.is_empty() {
            self.progression.bottle_stock.remove(item_id);
        }
    }

    /// The batches this item really has, with anything the inventory no longer
    /// supports trimmed off. The read-only twin of `reconcile_bottle_stock`.
    fn live_batches(&self, item_id: &str) -> Vec<BottleBatchEntry> {
        let held = self.inventory.get(item_id).copied().unwrap_or_default();
        let mut batches = self
            .progression
            .bottle_stock
            .get(item_id)
            .cloned()
            .unwrap_or_default();
        let mut tracked = batches.iter().map(|batch| batch.count).sum::<u32>();
        for batch in batches.iter_mut() {
            if tracked <= held {
                break;
            }
            let dropped = batch.count.min(tracked - held);
            batch.count -= dropped;
            tracked -= dropped;
        }
        batches.retain(|batch| batch.count > 0);
        batches
    }

    /// Bottles not accounted for by any batch: bought, gifted, or granted at the
    /// start. They are plain examples of the item and are graded as such.
    fn untracked_bottles(&self, item_id: &str) -> u32 {
        let held = self.inventory.get(item_id).copied().unwrap_or_default();
        let tracked = self
            .live_batches(item_id)
            .iter()
            .map(|batch| batch.count)
            .sum::<u32>();
        held.saturating_sub(tracked)
    }

    /// Whether a plain, unbranded example of this item would satisfy the
    /// request. Shop stock has to be able to fill a request with no quality
    /// demands, or buying a bottle to hand in would stop working.
    fn plain_bottle_qualifies(&self, data: &GameData, quest: &QuestDefinition) -> bool {
        let Some(item) = data.item(&quest.required_item_id) else {
            return false;
        };
        let quality_ok = quest.minimum_quality_band.is_empty()
            || quality_band_rank(quality_band(item.quality))
                >= quality_band_rank(&quest.minimum_quality_band);
        let traits_ok =
            trait_requirement_target(quest) == 0 || trait_requirement_met(quest, &item.traits);
        quality_ok && traits_ok
    }

    fn batch_qualifies(quest: &QuestDefinition, batch: &BottleBatchEntry) -> bool {
        let quality_ok = quest.minimum_quality_band.is_empty()
            || quality_band_rank(&batch.quality_band)
                >= quality_band_rank(&quest.minimum_quality_band);
        quality_ok && trait_requirement_met(quest, &batch.traits)
    }

    /// How many held bottles would actually satisfy this request.
    pub(super) fn qualifying_bottle_count(&self, data: &GameData, quest: &QuestDefinition) -> u32 {
        let from_batches = self
            .live_batches(&quest.required_item_id)
            .iter()
            .filter(|batch| Self::batch_qualifies(quest, batch))
            .map(|batch| batch.count)
            .sum::<u32>();
        let from_plain = if self.plain_bottle_qualifies(data, quest) {
            self.untracked_bottles(&quest.required_item_id)
        } else {
            0
        };
        from_batches.saturating_add(from_plain)
    }

    /// Hand over `amount` bottles that meet the request, worst acceptable first
    /// — somebody who brewed one Masterwork and three merely Excellent bottles
    /// for an Excellent order should keep the Masterwork.
    pub(super) fn spend_bottles_for_quest(
        &mut self,
        data: &GameData,
        quest: &QuestDefinition,
        amount: u32,
    ) {
        self.reconcile_bottle_stock(&quest.required_item_id);

        // Plain stock goes first when it is good enough: it is the least
        // interesting thing on the shelf.
        let mut remaining = amount;
        if self.plain_bottle_qualifies(data, quest) {
            let plain = self
                .untracked_bottles(&quest.required_item_id)
                .min(remaining);
            remaining -= plain;
        }
        if remaining == 0 {
            return;
        }

        let quest = quest.clone();
        let Some(batches) = self
            .progression
            .bottle_stock
            .get_mut(&quest.required_item_id)
        else {
            return;
        };
        for batch in batches.iter_mut() {
            if remaining == 0 {
                break;
            }
            if !Self::batch_qualifies(&quest, batch) {
                continue;
            }
            let spent = batch.count.min(remaining);
            batch.count -= spent;
            remaining -= spent;
        }
        batches.retain(|batch| batch.count > 0);
        if batches.is_empty() {
            self.progression
                .bottle_stock
                .remove(&quest.required_item_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{BottleBatchEntry, CraftedItemProfileEntry, GameData, QuestDefinition};

    fn masterwork_request(data: &GameData) -> QuestDefinition {
        let mut quest = data
            .quests
            .iter()
            .find(|quest| quest.required_item_id == "healing_draught")
            .expect("some request should want a healing draught")
            .clone();
        quest.minimum_quality_band = "Masterwork".to_owned();
        quest.required_amount = 1;
        quest.required_trait = String::new();
        quest.required_traits = Vec::new();
        quest
    }

    fn bottle(band: &str, score: u32, count: u32) -> BottleBatchEntry {
        BottleBatchEntry {
            item_id: "healing_draught".to_owned(),
            quality_score: score,
            quality_band: band.to_owned(),
            traits: Vec::new(),
            count,
        }
    }

    /// The bug this whole module exists for: the gate read
    /// `crafted_item_profiles`, which is a best-ever record, so one Masterwork
    /// brew satisfied every later Masterwork request permanently — including
    /// ones handed a Crude bottle brewed afterwards.
    #[test]
    fn a_masterwork_record_does_not_qualify_a_crude_bottle() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let quest = masterwork_request(&data);

        // The player has brewed a Masterwork at some point in the past...
        state.progression.crafted_item_profiles.insert(
            "healing_draught".to_owned(),
            CraftedItemProfileEntry {
                item_id: "healing_draught".to_owned(),
                best_quality_score: 95,
                best_quality_band: "Masterwork".to_owned(),
                inherited_traits: Vec::new(),
                effect_kinds: vec!["restore".to_owned()],
            },
        );
        // ...but what is on the shelf right now is Crude.
        state.inventory.insert("healing_draught".to_owned(), 1);
        state
            .progression
            .bottle_stock
            .insert("healing_draught".to_owned(), vec![bottle("Crude", 10, 1)]);

        assert_eq!(state.qualifying_bottle_count(&data, &quest), 0);
        assert!(!state.quest_requirements_met(&data, &quest));
    }

    /// Handing over the worst bottle that still meets the request is the
    /// difference between a quality system and a tax on brewing well.
    #[test]
    fn delivering_spends_the_worst_bottle_that_still_qualifies() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let mut quest = masterwork_request(&data);
        quest.minimum_quality_band = "Fine".to_owned();

        state.inventory.insert("healing_draught".to_owned(), 3);
        state.progression.bottle_stock.insert(
            "healing_draught".to_owned(),
            vec![
                bottle("Crude", 10, 1),
                bottle("Fine", 55, 1),
                bottle("Masterwork", 95, 1),
            ],
        );

        assert_eq!(state.qualifying_bottle_count(&data, &quest), 2);
        state.spend_bottles_for_quest(&data, &quest, 1);
        *state.inventory.get_mut("healing_draught").expect("held") -= 1;

        let left = &state.progression.bottle_stock["healing_draught"];
        assert!(
            left.iter().any(|batch| batch.quality_band == "Masterwork"),
            "the best bottle should still be on the shelf"
        );
        assert!(
            !left.iter().any(|batch| batch.quality_band == "Fine"),
            "the Fine one was the worst that qualified and should have gone"
        );
    }

    /// Bottles leave the shelf a dozen ways that know nothing about batches —
    /// sold, drunk, imbued, fed to a planter. If a stale batch outlived the
    /// bottle it described, the next plain bottle bought to replace it would
    /// inherit that grade and the original bug would be back by another route.
    #[test]
    fn a_batch_cannot_outlive_the_bottle_it_described() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let quest = masterwork_request(&data);

        state.inventory.insert("healing_draught".to_owned(), 1);
        state.progression.bottle_stock.insert(
            "healing_draught".to_owned(),
            vec![bottle("Masterwork", 95, 1)],
        );
        assert_eq!(state.qualifying_bottle_count(&data, &quest), 1);

        // Sold or drunk — through `take_from_inventory`, which every removal
        // path now uses. Then a plain one is bought to replace it, leaving the
        // count exactly where it started: the case a lazy read-time trim cannot
        // see, because nothing about the totals looks wrong afterwards.
        state.take_from_inventory("healing_draught", 1);
        assert!(state.progression.bottle_stock.is_empty());
        state.inventory.insert("healing_draught".to_owned(), 1);

        assert_eq!(
            state.qualifying_bottle_count(&data, &quest),
            0,
            "a shop bottle inherited the grade of one that was already gone"
        );
    }
}
