use super::GameplayState;
use crate::data::AreaDefinition;
use macroquad::prelude::{
    draw_circle, draw_circle_lines, draw_poly_lines, get_time, vec2, Color, Vec2,
};

impl GameplayState {
    /// Untreated targets get a slow pulsing ring so they read as something to
    /// act on rather than scenery. Drawn from primitives on purpose: the whole
    /// point of a target is that it is a place in the world, and it needs to be
    /// legible before there is art for it.
    pub(super) fn draw_area_apply_targets(&self, area: &AreaDefinition, offset: Vec2) {
        for target in &area.apply_targets {
            if self.target_is_treated(target) {
                continue;
            }
            let center = vec2(offset.x + target.position[0], offset.y + target.position[1]);
            let color = Color::from_rgba(
                target.color[0],
                target.color[1],
                target.color[2],
                target.color[3],
            );
            let pulse = 0.5 + 0.5 * ((get_time() as f32) * 1.6).sin();
            let ring = target.radius * (0.72 + 0.18 * pulse);

            draw_circle(center.x, center.y, target.radius * 0.34, color);
            draw_circle_lines(center.x, center.y, ring, 2.0, color);
            // Three marks around it, so an untreated target is distinguishable
            // from a gather node at a glance rather than only by colour.
            draw_poly_lines(
                center.x,
                center.y,
                3,
                target.radius * 0.55,
                (get_time() as f32) * 18.0,
                1.5,
                color,
            );
        }
    }
}
