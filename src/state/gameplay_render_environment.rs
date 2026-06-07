use super::GameplayState;
use crate::data::AreaDefinition;
use crate::ui::draw_environment_overlay_view;
use macroquad::prelude::Vec2;

impl GameplayState {
    pub(super) fn draw_environment_overlay(&self, area: &AreaDefinition, offset: Vec2) {
        draw_environment_overlay_view(
            area,
            offset,
            self.current_time_window(),
            self.current_weather(),
        );
    }
}
