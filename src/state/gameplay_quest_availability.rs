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
            && self.has_rapport_requirement(quest)
            && self.has_journal_requirement(quest)
    }

    /// Beat gate. The same field warps, stations and gather nodes already read,
    /// which is what lets a request wait on the ending without any new gating
    /// machinery — `observatory_ending` is a journal beat like any other.
    fn has_journal_requirement(&self, quest: &QuestDefinition) -> bool {
        quest.required_journal_milestone.is_empty()
            || self.has_journal_milestone(&quest.required_journal_milestone)
    }

    /// Standing gate. A townsperson who counts the player a confidant asks for
    /// things they would not mention to a stranger.
    fn has_rapport_requirement(&self, quest: &QuestDefinition) -> bool {
        quest.required_rapport_npc_id.is_empty()
            || self.rapport_value(&quest.required_rapport_npc_id) >= quest.required_rapport
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

        // Name the beat the way the journal did. A raw beat id here would be the
        // same leak the prerequisite titles above were fixed for.
        let missing_beat = if self.has_journal_requirement(quest) {
            String::new()
        } else {
            beat_title(data, &quest.required_journal_milestone)
        };

        quest_unlock_text::summary(QuestUnlockRequirements {
            missing_prereqs,
            missing_warp,
            missing_total_brews,
            minimum_total_brews: quest.minimum_total_brews,
            missing_mastery,
            missing_beat,
        })
    }
}

/// A beat's title, wherever it was authored: the narrative spine, a quest's
/// completion milestones, a recipe's discovery milestones, or an apply target.
/// Falls back to the id, which is better than an empty reason.
fn beat_title(data: &GameData, beat_id: &str) -> String {
    let spine = crate::content::narrative_text()
        .milestones
        .all()
        .into_iter()
        .find(|milestone| milestone.id == beat_id)
        .map(|milestone| milestone.title.clone());
    spine
        .or_else(|| {
            data.quests
                .iter()
                .flat_map(|quest| quest.completion_milestones.iter())
                .chain(
                    data.recipes
                        .iter()
                        .flat_map(|recipe| recipe.discovery_milestones.iter()),
                )
                .chain(
                    data.areas
                        .iter()
                        .flat_map(|area| area.apply_targets.iter())
                        .flat_map(|target| target.completion_milestones.iter()),
                )
                .find(|milestone| milestone.id == beat_id)
                .map(|milestone| milestone.title.clone())
        })
        .unwrap_or_else(|| beat_id.to_owned())
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

    /// The field is new, and a new gate field is exactly what this project keeps
    /// getting wrong — a key with no reader looks configured and opens nothing.
    /// So drive it: a request waiting on a beat must stay shut until the beat is
    /// recorded, must open the moment it is, and must name the beat by its
    /// *title* while shut rather than leaking the id.
    #[test]
    fn a_beat_gated_request_waits_for_the_beat() {
        let data = crate::data::load_embedded().expect("embedded game data should load");
        let quest = data
            .quests
            .iter()
            .find(|quest| !quest.required_journal_milestone.is_empty())
            .expect("some request should wait on a journal beat");
        let beat = quest.required_journal_milestone.clone();
        let title = super::beat_title(&data, &beat);
        assert_ne!(title, beat, "the beat should be authored with a title");

        let mut state = GameplayState::new(&data);
        state.progression.total_brews = 999;
        for prerequisite in &quest.prerequisite_quests {
            state
                .progression
                .completed_quests
                .insert(prerequisite.clone());
        }
        if !quest.required_mastered_recipe.is_empty() {
            state.progression.recipe_mastery.insert(
                quest.required_mastered_recipe.clone(),
                crate::alchemy::MASTERED_BREW_COUNT,
            );
        }

        assert!(
            !state.quest_is_available(quest),
            "{} is on offer before its beat is recorded",
            quest.id
        );
        let locked = state.quest_unlock_summary(&data, quest);
        assert!(
            locked.contains(&title),
            "a shut request should name the beat it waits on: {locked}"
        );

        state.push_journal_milestone(&beat, &title, "recorded by the test");
        assert!(
            state.quest_is_available(quest),
            "recording the beat should open it"
        );
    }
}
