use super::GameplayState;
use crate::data::{AreaDefinition, GameData};
use crate::ui::draw_interaction_prompt;

impl GameplayState {
    pub(super) fn draw_prompt(&self, area: &AreaDefinition, data: &GameData) {
        if let Some(prompt) = self.world_prompt_view(area, data) {
            draw_interaction_prompt(&prompt.text);
        }
    }
}
