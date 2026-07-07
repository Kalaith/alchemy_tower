use crate::data::{GameData, QuestDefinition};

pub(super) fn quest_required_traits(quest: &QuestDefinition) -> Vec<&str> {
    let mut traits = quest
        .required_traits
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if !quest.required_trait.is_empty() && !traits.contains(&quest.required_trait.as_str()) {
        traits.push(&quest.required_trait);
    }
    traits
}

pub(super) fn quest_required_effect_kinds(quest: &QuestDefinition) -> Vec<&str> {
    let mut effects = quest
        .required_effect_kinds
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if !quest.required_effect_kind.is_empty()
        && !effects.contains(&quest.required_effect_kind.as_str())
    {
        effects.push(&quest.required_effect_kind);
    }
    effects
}

pub(super) fn requirement_target(requirement_count: usize, configured_minimum: u32) -> usize {
    if requirement_count == 0 {
        return 0;
    }
    if configured_minimum == 0 {
        return requirement_count;
    }
    (configured_minimum as usize).min(requirement_count)
}

pub(super) fn matching_requirement_count(required: &[&str], actual: &[String]) -> usize {
    required
        .iter()
        .filter(|required_value| {
            actual
                .iter()
                .any(|actual_value| actual_value == *required_value)
        })
        .count()
}

pub(super) fn trait_requirement_target(quest: &QuestDefinition) -> usize {
    requirement_target(
        quest_required_traits(quest).len(),
        quest.minimum_trait_matches,
    )
}

pub(super) fn effect_requirement_target(quest: &QuestDefinition) -> usize {
    requirement_target(
        quest_required_effect_kinds(quest).len(),
        quest.minimum_effect_matches,
    )
}

pub(super) fn trait_requirement_met(quest: &QuestDefinition, actual_traits: &[String]) -> bool {
    let required_traits = quest_required_traits(quest);
    let target = trait_requirement_target(quest);
    matching_requirement_count(&required_traits, actual_traits) >= target
}

pub(super) fn effect_requirement_met(
    data: &GameData,
    quest: &QuestDefinition,
    effect_kinds: Option<&[String]>,
) -> bool {
    let required_effects = quest_required_effect_kinds(quest);
    let target = effect_requirement_target(quest);
    if target == 0 {
        return true;
    }

    let owned_effects = effect_kinds
        .map(|effects| effects.to_vec())
        .or_else(|| {
            data.item(&quest.required_item_id).map(|item| {
                item.effects
                    .iter()
                    .map(|effect| effect.kind.to_string())
                    .collect::<Vec<_>>()
            })
        })
        .unwrap_or_default();

    matching_requirement_count(&required_effects, &owned_effects) >= target
}

#[cfg(test)]
mod tests {
    use super::trait_requirement_met;
    use crate::data::QuestDefinition;

    fn test_quest() -> QuestDefinition {
        QuestDefinition {
            id: "quest".to_owned(),
            title: "Quest".to_owned(),
            description: String::new(),
            required_item_id: "item".to_owned(),
            required_amount: 1,
            reward_coins: 1,
            giver_npc_id: "npc".to_owned(),
            minimum_quality_band: String::new(),
            required_trait: String::new(),
            required_traits: Vec::new(),
            minimum_trait_matches: 0,
            required_effect_kind: String::new(),
            required_effect_kinds: Vec::new(),
            minimum_effect_matches: 0,
            prerequisite_quests: Vec::new(),
            required_unlocked_warp: String::new(),
            minimum_total_brews: 0,
            completion_milestones: Vec::new(),
            repeatable: false,
            repeat_cooldown_days: 0,
        }
    }

    #[test]
    fn legacy_single_trait_requirement_still_works() {
        let mut quest = test_quest();
        quest.required_trait = "restorative".to_owned();

        assert!(trait_requirement_met(&quest, &["restorative".to_owned()]));
        assert!(!trait_requirement_met(&quest, &["luminous".to_owned()]));
    }

    #[test]
    fn multi_trait_requirement_supports_thresholds() {
        let mut quest = test_quest();
        quest.required_traits = vec![
            "restorative".to_owned(),
            "calm".to_owned(),
            "luminous".to_owned(),
        ];
        quest.minimum_trait_matches = 2;

        assert!(trait_requirement_met(
            &quest,
            &["restorative".to_owned(), "calm".to_owned()]
        ));
        assert!(!trait_requirement_met(&quest, &["restorative".to_owned()]));
    }
}
