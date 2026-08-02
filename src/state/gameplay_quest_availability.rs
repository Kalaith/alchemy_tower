use super::GameplayState;
use crate::data::{GameData, QuestDefinition};

#[path = "gameplay_quest_unlock_text.rs"]
mod quest_unlock_text;

use self::quest_unlock_text::QuestUnlockRequirements;

impl GameplayState {
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
            && self.has_mastered_requirement(quest)
    }

    /// Mastery is seven clean brews of one formula. `total_brews` measures how
    /// busy somebody has been; this measures whether they can make a particular
    /// thing the same way twice.
    fn has_mastered_requirement(&self, quest: &QuestDefinition) -> bool {
        quest.required_mastered_recipe.is_empty()
            || self.recipe_mastery_brews(&quest.required_mastered_recipe)
                >= crate::alchemy::MASTERED_BREW_COUNT
    }

    pub(super) fn quest_unlock_summary(&self, data: &GameData, quest: &QuestDefinition) -> String {
        // Name the blocking request the way the player saw it. Raw quest ids
        // used to leak straight into this line, and a chain multiplies them.
        let missing_prereqs = quest
            .prerequisite_quests
            .iter()
            .filter(|quest_id| !self.progression.completed_quests.contains(*quest_id))
            .map(|quest_id| {
                data.quest(quest_id)
                    .map(|blocking| blocking.title.clone())
                    .unwrap_or_else(|| quest_id.clone())
            })
            .collect::<Vec<_>>();
        let missing_warp = !quest.required_unlocked_warp.is_empty()
            && !self
                .progression
                .unlocked_warps
                .contains(&quest.required_unlocked_warp);
        let missing_total_brews = self.progression.total_brews < quest.minimum_total_brews;
        // Name the formula the way the bench does, not by its id.
        let missing_mastery = if self.has_mastered_requirement(quest) {
            String::new()
        } else {
            data.recipe(&quest.required_mastered_recipe)
                .map(|recipe| recipe.name.clone())
                .unwrap_or_else(|| quest.required_mastered_recipe.clone())
        };

        quest_unlock_text::summary(QuestUnlockRequirements {
            missing_prereqs,
            missing_warp,
            missing_total_brews,
            minimum_total_brews: quest.minimum_total_brews,
            missing_mastery,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::alchemy::MASTERED_BREW_COUNT;
    use crate::state::gameplay::GameplayState;

    /// The gate is the point of the field, so drive it rather than trusting the
    /// expression: a request wanting a mastered formula must stay shut at six
    /// clean brews and open at seven, and must say which formula while shut.
    #[test]
    fn a_mastery_gated_request_waits_for_the_seventh_brew() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let quest = data
            .quests
            .iter()
            .find(|quest| !quest.required_mastered_recipe.is_empty())
            .expect("some request should ask for a mastered formula");
        let recipe_id = quest.required_mastered_recipe.clone();
        let recipe_name = data
            .recipe(&recipe_id)
            .expect("the guard checks this resolves")
            .name
            .clone();

        let mut state = GameplayState::new(&data);
        state.progression.total_brews = 999;
        for prerequisite in &quest.prerequisite_quests {
            state
                .progression
                .completed_quests
                .insert(prerequisite.clone());
        }

        state
            .progression
            .recipe_mastery
            .insert(recipe_id.clone(), MASTERED_BREW_COUNT - 1);
        assert!(
            !state.quest_is_available(quest),
            "six clean brews is not mastery"
        );
        let locked = state.quest_unlock_summary(&data, quest);
        assert!(
            locked.contains(&recipe_name),
            "a shut request should name the formula it is waiting on: {locked}"
        );

        state
            .progression
            .recipe_mastery
            .insert(recipe_id, MASTERED_BREW_COUNT);
        assert!(
            state.quest_is_available(quest),
            "the seventh brew should open it"
        );
    }
}
