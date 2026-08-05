use super::gameplay_quest_requirement_summary::{
    carry_requirement_summary, effect_requirement_summary, quality_requirement_summary,
    ready_requirement_summary, trait_requirement_summary,
};
use super::gameplay_quest_requirements::{
    effect_requirement_met, effect_requirement_target, trait_requirement_target,
};
use super::GameplayState;
use crate::data::{GameData, QuestDefinition};

impl GameplayState {
    pub(super) fn quest_requirements_met(&self, data: &GameData, quest: &QuestDefinition) -> bool {
        let carried = self
            .inventory
            .get(&quest.required_item_id)
            .copied()
            .unwrap_or_default();
        if carried < quest.required_amount {
            return false;
        }

        // A commission is funded, not merely filled.
        if self.coins < quest.coin_cost {
            return false;
        }

        // Quality and traits are asked of the bottles actually on the shelf,
        // not of `crafted_item_profiles`, which is a best-ever record: one
        // Masterwork used to satisfy every later Masterwork request forever,
        // including ones filled with Crude bottles brewed after it.
        if self.qualifying_bottle_count(data, quest) < quest.required_amount {
            return false;
        }

        // Effects belong to the item rather than the bottle, so they are still
        // read from the definition.
        let profile = self
            .progression
            .crafted_item_profiles
            .get(&quest.required_item_id);
        effect_requirement_met(
            data,
            quest,
            profile.map(|profile| profile.effect_kinds.as_slice()),
        )
    }

    pub(super) fn quest_requirement_summary(
        &self,
        data: &GameData,
        quest: &QuestDefinition,
    ) -> String {
        let carried = self
            .inventory
            .get(&quest.required_item_id)
            .copied()
            .unwrap_or_default();
        let mut requirements = Vec::new();
        if carried < quest.required_amount {
            requirements.push(carry_requirement_summary(carried, quest.required_amount));
        }
        let qualifying = self.qualifying_bottle_count(data, quest);
        let quality_short = !quest.minimum_quality_band.is_empty()
            && self.bottles_meeting_quality_count(data, quest) < quest.required_amount;
        let trait_short = trait_requirement_target(quest) > 0
            && self.bottles_meeting_trait_count(data, quest) < quest.required_amount;
        if qualifying < quest.required_amount {
            if quality_short {
                requirements.push(quality_requirement_summary(&quest.minimum_quality_band));
            }
            if trait_short {
                requirements.push(trait_requirement_summary(quest));
            }
            // Two separate bottles can each satisfy half of a combined
            // specification. If neither individual count is short, name both
            // constraints so the text never says the delivery is ready.
            if !quality_short
                && !trait_short
                && !quest.minimum_quality_band.is_empty()
                && trait_requirement_target(quest) > 0
            {
                requirements.push(quality_requirement_summary(&quest.minimum_quality_band));
                requirements.push(trait_requirement_summary(quest));
            }
        }
        if effect_requirement_target(quest) > 0 {
            let met = effect_requirement_met(
                data,
                quest,
                self.progression
                    .crafted_item_profiles
                    .get(&quest.required_item_id)
                    .map(|profile| profile.effect_kinds.as_slice()),
            );
            if !met {
                requirements.push(effect_requirement_summary(quest));
            }
        }

        if requirements.is_empty() {
            ready_requirement_summary()
        } else {
            requirements.join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameplayState;
    use crate::data::{BottleBatchEntry, CraftedItemProfileEntry};

    #[test]
    fn quest_text_describes_the_bottles_held_now_not_the_best_ever_made() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let mut state = GameplayState::new(&data);
        let mut quest = data
            .quests
            .iter()
            .find(|quest| quest.required_item_id == "healing_draught")
            .expect("some request should want a healing draught")
            .clone();
        quest.required_amount = 1;
        quest.minimum_quality_band = "Masterwork".to_owned();
        quest.required_trait = "restorative".to_owned();
        quest.required_traits.clear();
        quest.required_effect_kind.clear();
        quest.required_effect_kinds.clear();

        state.progression.crafted_item_profiles.insert(
            quest.required_item_id.clone(),
            CraftedItemProfileEntry {
                item_id: quest.required_item_id.clone(),
                best_quality_score: 95,
                best_quality_band: "Masterwork".to_owned(),
                inherited_traits: vec!["restorative".to_owned()],
                effect_kinds: Vec::new(),
            },
        );
        state.inventory.insert(quest.required_item_id.clone(), 1);
        state.progression.bottle_stock.insert(
            quest.required_item_id.clone(),
            vec![BottleBatchEntry {
                item_id: quest.required_item_id.clone(),
                quality_score: 10,
                quality_band: "Crude".to_owned(),
                traits: Vec::new(),
                count: 1,
            }],
        );

        let summary = state.quest_requirement_summary(&data, &quest);
        assert!(
            summary.contains("Masterwork"),
            "quality was hidden: {summary}"
        );
        assert!(
            summary.contains("restorative"),
            "traits were hidden: {summary}"
        );
        assert_ne!(summary, super::ready_requirement_summary());
    }
}
