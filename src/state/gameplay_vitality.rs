//! What a day of work costs, and what a restorative is for.
//!
//! Vitality shipped as a number that only ever went up. `apply_effect` raised
//! it and the HUD drew it, and nothing anywhere took any away — so `Restore`,
//! which 51 of the game's 110 authored effect blocks use, healed a wound the
//! game could not inflict. Half the potions in the tower were flavour text.
//!
//! It is the working day now. Standing over a cauldron costs; walking a route
//! and bending down costs less. Run out and you wake up at home having lost the
//! morning, the same collapse the small hours already cause. Sleeping in a bed
//! by choice gives the day back in full; being carried home does not quite.
//!
//! That makes a restorative the thing that buys another hour at the bench,
//! which is what a potion called Healing Draught should be for.

use super::GameplayState;
use crate::data::GameData;

impl GameplayState {
    /// Spend some of the day. Stops at zero rather than going negative — the
    /// collapse is handled by `handle_sleep_pressure`, so that running out
    /// mid-brew finishes the brew first and drags the player home afterwards
    /// rather than interrupting itself.
    pub(super) fn spend_vitality(&mut self, amount: f32) {
        self.vitality = (self.vitality - amount).clamp(0.0, 100.0);
    }

    /// Nothing left in the day.
    pub(super) fn is_exhausted(&self) -> bool {
        self.vitality <= 0.0
    }

    pub(super) fn restore_vitality_to(&mut self, value: f32) {
        self.vitality = value.clamp(0.0, 100.0);
    }

    pub(super) fn spend_brewing_vitality(&mut self, data: &GameData) {
        self.spend_vitality(data.config.balance.vitality.brew_cost);
    }

    pub(super) fn spend_gathering_vitality(&mut self, data: &GameData) {
        self.spend_vitality(data.config.balance.vitality.gather_cost);
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    /// The whole point: a restorative has to be able to buy back work that was
    /// actually spent. Before this, drinking one at full vitality did nothing
    /// and there was no other state to be in.
    #[test]
    fn a_restorative_buys_back_a_day_that_was_spent() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        assert_eq!(state.vitality, 100.0);

        for _ in 0..5 {
            state.spend_brewing_vitality(&data);
        }
        let worked = state.vitality;
        assert!(worked < 100.0, "brewing cost nothing");

        let draught = data
            .item("healing_draught")
            .expect("the healing draught should exist");
        let restore = draught
            .effects
            .iter()
            .find(|effect| effect.kind == crate::data::EffectKind::Restore)
            .expect("a healing draught should restore");
        state.apply_effect(restore);
        assert!(
            state.vitality > worked,
            "drinking a restorative gave nothing back: {} vs {worked}",
            state.vitality
        );
    }

    /// Vitality is a day, not a pit. Spending past the end stops at empty, so
    /// the collapse is a state the player can be in rather than a number that
    /// keeps falling while they carry on.
    #[test]
    fn a_day_runs_out_rather_than_going_negative() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        for _ in 0..200 {
            state.spend_brewing_vitality(&data);
        }
        assert_eq!(state.vitality, 0.0);
        assert!(state.is_exhausted());
    }

    /// Going to bed is better than running yourself into the ground, or there
    /// is no reason to ever stop working.
    #[test]
    fn a_chosen_night_is_worth_more_than_being_carried_home() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let tuning = &data.config.balance.vitality;
        assert!(
            tuning.sleep_restores > tuning.collapse_restores,
            "collapsing is as good as sleeping, so nothing is at stake"
        );
    }

    /// The collapse, end to end: work until there is nothing left, and the next
    /// tick carries you home having lost the morning. This is what gives the
    /// drain teeth — without it, running out would be a number sitting at zero
    /// while the player carried on regardless.
    #[test]
    fn running_out_carries_you_home_and_costs_the_morning() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        state.set_clock_minutes(15.0 * 60.0);
        let day = state.world.day_index;

        while !state.is_exhausted() {
            state.spend_brewing_vitality(&data);
        }
        state.handle_sleep_pressure(&data);

        assert_eq!(state.world.day_index, day + 1, "the day should have turned");
        assert_eq!(state.current_clock_minutes(), 10.0 * 60.0);
        assert_eq!(
            state.vitality, data.config.balance.vitality.collapse_restores,
            "being carried home should give back less than a chosen night"
        );
        assert!(!state.is_exhausted(), "you wake up able to work");
    }

    /// A restorative taken in time is the difference between finishing the day
    /// and losing the next morning to it.
    #[test]
    fn drinking_in_time_keeps_the_day() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        state.set_clock_minutes(15.0 * 60.0);
        let day = state.world.day_index;

        while state.vitality > data.config.balance.vitality.brew_cost {
            state.spend_brewing_vitality(&data);
        }
        state.inventory.insert("healing_draught".to_owned(), 1);
        state.consume_potion(&data, "healing_draught");
        state.spend_brewing_vitality(&data);
        state.handle_sleep_pressure(&data);

        assert!(!state.is_exhausted());
        assert_eq!(state.world.day_index, day, "the day should not have turned");
    }
}
