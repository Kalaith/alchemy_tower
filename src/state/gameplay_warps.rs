use super::GameplayState;
use crate::data::{GameData, WarpDefinition};

#[path = "gameplay_warp_requirement_text.rs"]
mod requirement_text;

use self::requirement_text::WarpRequirementProgress;

impl GameplayState {
    pub(super) fn warp_is_unlocked(&self, warp: &WarpDefinition) -> bool {
        self.progression.unlocked_warps.contains(&warp.id)
            || (warp.required_total_brews == 0
                && warp.required_coins == 0
                && warp.required_item_id.is_empty()
                && warp.required_journal_milestone.is_empty()
                && warp.required_mastered_recipe.is_empty())
    }

    pub(super) fn can_unlock_warp(&self, warp: &WarpDefinition) -> bool {
        self.progression.total_brews >= warp.required_total_brews
            && self.coins >= warp.required_coins
            && (warp.required_item_id.is_empty()
                || self
                    .inventory
                    .get(&warp.required_item_id)
                    .copied()
                    .unwrap_or_default()
                    >= warp.required_item_amount)
            && (warp.required_journal_milestone.is_empty()
                || self.has_journal_milestone(&warp.required_journal_milestone))
            && self.mastery_requirement_met(warp)
    }

    /// True when the warp has no mastery gate, or the named recipe has reached
    /// the mastered stage.
    pub(super) fn mastery_requirement_met(&self, warp: &WarpDefinition) -> bool {
        warp.required_mastered_recipe.is_empty()
            || self.recipe_mastery_brews(&warp.required_mastered_recipe)
                >= crate::alchemy::MASTERED_BREW_COUNT
    }

    pub(super) fn pay_warp_costs(&mut self, warp: &WarpDefinition) {
        self.coins = self.coins.saturating_sub(warp.required_coins);
        if !warp.required_item_id.is_empty() {
            if let Some(amount) = self.inventory.get_mut(&warp.required_item_id) {
                *amount = amount.saturating_sub(warp.required_item_amount);
            }
            self.inventory.retain(|_, amount| *amount > 0);
        }
    }

    pub(super) fn locked_warps<'a>(&self, data: &'a GameData) -> Vec<&'a WarpDefinition> {
        data.areas
            .iter()
            .flat_map(|area| area.warps.iter())
            .filter(|warp| !self.warp_is_unlocked(warp))
            .collect()
    }

    pub(super) fn next_locked_warp<'a>(&self, data: &'a GameData) -> Option<&'a WarpDefinition> {
        self.locked_warps(data)
            .into_iter()
            .min_by_key(|warp| self.warp_progress_score(data, warp))
    }

    pub(super) fn warp_progress_score(&self, _data: &GameData, warp: &WarpDefinition) -> u32 {
        let owned = self
            .inventory
            .get(&warp.required_item_id)
            .copied()
            .unwrap_or_default();
        let item_missing = warp.required_item_amount.saturating_sub(owned);
        let milestone_missing = u32::from(
            !warp.required_journal_milestone.is_empty()
                && !self.has_journal_milestone(&warp.required_journal_milestone),
        );
        let mastery_missing = u32::from(!self.mastery_requirement_met(warp));

        warp.required_total_brews
            .saturating_sub(self.progression.total_brews)
            .saturating_mul(100)
            .saturating_add(warp.required_coins.saturating_sub(self.coins))
            .saturating_add(item_missing.saturating_mul(25))
            .saturating_add(milestone_missing.saturating_mul(150))
            .saturating_add(mastery_missing.saturating_mul(150))
    }

    pub(super) fn warp_requirement_summary(
        &self,
        data: &GameData,
        warp: &WarpDefinition,
    ) -> String {
        requirement_text::warp_requirement_summary(data, warp, self.warp_requirement_progress(warp))
    }

    pub(super) fn warp_lock_text(&self, data: &GameData, warp: &WarpDefinition) -> String {
        let requirement_summary = self.warp_requirement_summary(data, warp);
        requirement_text::warp_lock_text(warp, requirement_summary)
    }

    fn warp_requirement_progress(&self, warp: &WarpDefinition) -> WarpRequirementProgress {
        let owned_required_item = self
            .inventory
            .get(&warp.required_item_id)
            .copied()
            .unwrap_or_default();
        let missing_journal_milestone = !warp.required_journal_milestone.is_empty()
            && !self.has_journal_milestone(&warp.required_journal_milestone);
        let mastered_recipe_brews = if warp.required_mastered_recipe.is_empty() {
            0
        } else {
            self.recipe_mastery_brews(&warp.required_mastered_recipe)
        };

        WarpRequirementProgress::new(
            self.progression.total_brews,
            self.coins,
            owned_required_item,
            missing_journal_milestone,
            mastered_recipe_brews,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;

    #[test]
    fn mastery_gate_blocks_until_recipe_is_mastered() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);

        let warp = data
            .areas
            .iter()
            .flat_map(|area| area.warps.iter())
            .find(|warp| warp.id == "containment_to_rune_workshop")
            .expect("rune workshop warp should exist")
            .clone();
        assert_eq!(warp.required_mastered_recipe, "glow_potion_recipe");

        // Pay the coin cost but leave the recipe unmastered: still locked.
        state.coins = warp.required_coins + 10;
        assert!(!state.can_unlock_warp(&warp));
        assert!(!state.warp_is_unlocked(&warp));

        // Reaching the mastered threshold opens the gate.
        state.progression.recipe_mastery.insert(
            "glow_potion_recipe".to_owned(),
            crate::alchemy::MASTERED_BREW_COUNT,
        );
        assert!(state.can_unlock_warp(&warp));
    }
}
