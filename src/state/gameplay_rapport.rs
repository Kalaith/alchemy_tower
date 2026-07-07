use super::GameplayState;
use crate::content::{ui_copy, ui_format};
use crate::data::{GameData, NpcDefinition};

#[path = "gameplay_rapport_text.rs"]
mod rapport_text;

/// Rapport at which a townsperson counts you a friend and hands over their
/// one-time thank-you gift. Reachable by seeing one of their errands through
/// (+1 on accept, +2 on completion in `gameplay_dialogue`).
pub(super) const FRIEND_RAPPORT: i32 = 3;
const CONFIDANT_RAPPORT: i32 = 6;

impl GameplayState {
    pub(super) fn rapport_value(&self, npc_id: &str) -> i32 {
        self.progression
            .relationships
            .get(npc_id)
            .copied()
            .unwrap_or_default()
    }

    /// Human-readable standing for the journal rapport tab.
    pub(super) fn rapport_tier_label(&self, rapport: i32) -> &'static str {
        if rapport >= CONFIDANT_RAPPORT {
            ui_copy("rapport_tier_confidant")
        } else if rapport >= FRIEND_RAPPORT {
            ui_copy("rapport_tier_friend")
        } else if rapport >= 1 {
            ui_copy("rapport_tier_acquaintance")
        } else {
            ui_copy("rapport_tier_stranger")
        }
    }

    fn friendship_milestone_id(npc_id: &str) -> String {
        format!("rapport_friend_{npc_id}")
    }

    pub(super) fn has_reached_friendship(&self, npc_id: &str) -> bool {
        self.has_journal_milestone(&Self::friendship_milestone_id(npc_id))
    }

    /// If this townsperson has just crossed into the friend tier and their gift
    /// has not been handed over yet, grant it once: coins/item reward, a warm
    /// line, and a permanent journal beat. Returns true when a gift was given,
    /// so the caller can surface it instead of the normal dialogue flow.
    pub(super) fn try_grant_friendship_gift(
        &mut self,
        data: &GameData,
        npc: &NpcDefinition,
    ) -> bool {
        // The Crow is a guide, not a townsperson, and has no friendship arc.
        if npc.id == "crow_guide" || npc.friendship_line.is_empty() {
            return false;
        }
        if self.rapport_value(&npc.id) < FRIEND_RAPPORT || self.has_reached_friendship(&npc.id) {
            return false;
        }

        self.coins += npc.friendship_reward_coins;
        if !npc.friendship_reward_item_id.is_empty() && npc.friendship_reward_amount > 0 {
            *self
                .inventory
                .entry(npc.friendship_reward_item_id.clone())
                .or_insert(0) += npc.friendship_reward_amount;
        }

        // Record the beat permanently (also raises a journal-note toast).
        self.push_journal_milestone(
            &Self::friendship_milestone_id(&npc.id),
            &ui_format("rapport_friend_title", &[("name", &npc.name)]),
            &npc.friendship_line,
        );
        self.trigger_quest_complete_feedback(rapport_text::friendship_toast(&npc.name));
        self.runtime.status_text = rapport_text::friendship_status(data, npc);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{GameplayState, FRIEND_RAPPORT};

    #[test]
    fn friendship_gift_granted_once_at_friend_tier() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let npc = data
            .npc("mira_apothecary")
            .expect("mira should exist")
            .clone();
        assert!(!npc.friendship_line.is_empty());
        assert!(npc.friendship_reward_coins > 0);

        // Below the friend tier: no gift.
        assert!(!state.try_grant_friendship_gift(&data, &npc));

        // At the friend tier: gift is handed over exactly once.
        state
            .progression
            .relationships
            .insert(npc.id.clone(), FRIEND_RAPPORT);
        let coins_before = state.coins;
        assert!(state.try_grant_friendship_gift(&data, &npc));
        assert_eq!(state.coins, coins_before + npc.friendship_reward_coins);
        assert!(state.has_reached_friendship(&npc.id));
        assert_eq!(
            state
                .inventory
                .get(&npc.friendship_reward_item_id)
                .copied()
                .unwrap_or_default(),
            npc.friendship_reward_amount
        );

        // Not repeatable.
        assert!(!state.try_grant_friendship_gift(&data, &npc));
    }

    #[test]
    fn rapport_tiers_track_standing() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let state = GameplayState::new(&data);
        assert_eq!(state.rapport_tier_label(0), "Stranger");
        assert_eq!(state.rapport_tier_label(1), "Acquaintance");
        assert_eq!(state.rapport_tier_label(FRIEND_RAPPORT), "Friend");
        assert_eq!(state.rapport_tier_label(6), "Confidant");
    }
}
