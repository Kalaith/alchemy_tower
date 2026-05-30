use super::GameplayState;
use crate::data::{AreaDefinition, GameData};
use crate::ui::draw_interaction_prompt;
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_prompt(&self, area: &AreaDefinition, offset: Vec2, data: &GameData) {
        if let Some(prompt) = self.world_prompt_view(area, offset, data) {
            draw_interaction_prompt(prompt.position, &prompt.text);
        }
    }
}
