use super::gameplay_quest_requirements::{
    effect_requirement_met, effect_requirement_summary, effect_requirement_target,
    trait_requirement_met, trait_requirement_summary, trait_requirement_target,
};
use super::gameplay_support::quality_band_rank;
use super::GameplayState;
use crate::content::ui_format;
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

        let profile = self
            .progression
            .crafted_item_profiles
            .get(&quest.required_item_id);
        let quality_ok = quest.minimum_quality_band.is_empty()
            || profile
                .map(|profile| {
                    quality_band_rank(&profile.best_quality_band)
                        >= quality_band_rank(&quest.minimum_quality_band)
                })
                .unwrap_or(false);
        let trait_ok = profile
            .map(|profile| trait_requirement_met(quest, &profile.inherited_traits))
            .unwrap_or_else(|| trait_requirement_target(quest) == 0);
        let effect_ok = effect_requirement_met(
            data,
            quest,
            profile.map(|profile| profile.effect_kinds.as_slice()),
        );

        quality_ok && trait_ok && effect_ok
    }

    pub(super) fn quest_is_available(&self, quest: &QuestDefinition) -> bool {
        quest
            .prerequisite_quests
            .iter()
            .all(|quest_id| self.progression.completed_quests.contains(quest_id))
            && (quest.required_unlocked_warp.is_empty()
                || self
                    .progression
                    .unlocked_warps
                    .contains(&quest.required_unlocked_warp))
            && self.progression.total_brews >= quest.minimum_total_brews
    }

    pub(super) fn quest_unlock_summary(&self, quest: &QuestDefinition) -> String {
        let mut reasons = Vec::new();
        let missing_prereqs = quest
            .prerequisite_quests
            .iter()
            .filter(|quest_id| !self.progression.completed_quests.contains(*quest_id))
            .cloned()
            .collect::<Vec<_>>();
        if !missing_prereqs.is_empty() {
            reasons.push(ui_format(
                "quests_unlock_finish",
                &[("quests", &missing_prereqs.join(", "))],
            ));
        }
        if !quest.required_unlocked_warp.is_empty()
            && !self
                .progression
                .unlocked_warps
                .contains(&quest.required_unlocked_warp)
        {
            reasons.push(ui_format("quests_unlock_greenhouse", &[]));
        }
        if self.progression.total_brews < quest.minimum_total_brews {
            reasons.push(ui_format(
                "quests_unlock_brews",
                &[("brews", &quest.minimum_total_brews.to_string())],
            ));
        }
        if reasons.is_empty() {
            ui_format("quests_unlock_closed", &[])
        } else {
            ui_format(
                "quests_unlock_after",
                &[("reasons", &reasons.join(" and "))],
            )
        }
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
            requirements.push(ui_format(
                "quests_requirement_carry",
                &[
                    ("carried", &carried.to_string()),
                    ("required", &quest.required_amount.to_string()),
                ],
            ));
        }
        if !quest.minimum_quality_band.is_empty() {
            let met = self
                .progression
                .crafted_item_profiles
                .get(&quest.required_item_id)
                .map(|profile| {
                    quality_band_rank(&profile.best_quality_band)
                        >= quality_band_rank(&quest.minimum_quality_band)
                })
                .unwrap_or(false);
            if !met {
                requirements.push(ui_format(
                    "quests_requirement_quality",
                    &[("band", &quest.minimum_quality_band)],
                ));
            }
        }
        if trait_requirement_target(quest) > 0 {
            let met = self
                .progression
                .crafted_item_profiles
                .get(&quest.required_item_id)
                .map(|profile| trait_requirement_met(quest, &profile.inherited_traits))
                .unwrap_or(false);
            if !met {
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
            ui_format("quests_requirement_ready", &[])
        } else {
            requirements.join(", ")
        }
    }
}
