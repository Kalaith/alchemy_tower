use super::GameplayState;
use crate::data::AreaDefinition;
use crate::ui::draw_phase1_story_flourishes_view;
use macroquad::prelude::Vec2;

impl GameplayState {
    pub(super) fn draw_phase1_story_flourishes(&self, area: &AreaDefinition, offset: Vec2) {
        let cultivation_for_brin_complete = self
            .progression
            .completed_quests
            .contains("cultivation_for_brin");
        draw_phase1_story_flourishes_view(
            area,
            offset,
            self.progression
                .completed_quests
                .contains("healing_for_mira"),
            self.progression.completed_quests.contains("glow_for_rowan"),
            self.has_journal_milestone("greenhouse_repaired") || cultivation_for_brin_complete,
            cultivation_for_brin_complete,
        );
    }
}
