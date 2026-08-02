use super::GameplayState;
use crate::data::AreaDefinition;
use crate::ui::draw_phase1_story_flourishes_view;
use macroquad::prelude::Vec2;

impl GameplayState {
    pub(super) fn draw_phase1_story_flourishes(&self, area: &AreaDefinition, offset: Vec2) {
        draw_phase1_story_flourishes_view(area, offset, &|id| self.flourish_is_earned(area, id));
    }

    /// Whether the town has changed in the way this flourish depicts. Any one
    /// of the listed beats is enough; a flourish that lists none is scenery
    /// that is simply always there.
    pub(super) fn flourish_is_earned(&self, area: &AreaDefinition, flourish_id: &str) -> bool {
        let Some(flourish) = area
            .flourishes
            .iter()
            .find(|flourish| flourish.id == flourish_id)
        else {
            return false;
        };
        if flourish.after_any_completed_quest.is_empty()
            && flourish.after_any_journal_milestone.is_empty()
        {
            return true;
        }
        flourish
            .after_any_completed_quest
            .iter()
            .any(|quest_id| self.progression.completed_quests.contains(quest_id))
            || flourish
                .after_any_journal_milestone
                .iter()
                .any(|milestone_id| self.has_journal_milestone(milestone_id))
    }
}
