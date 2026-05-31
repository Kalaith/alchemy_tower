use super::gameplay_render_color::render_color;
use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::{AreaDefinition, GameData};
use crate::ui::draw_npc_world_marker;
use macroquad::prelude::{vec2, Vec2};

impl GameplayState {
    pub(super) fn draw_area_npcs(
        &self,
        area: &AreaDefinition,
        offset: Vec2,
        data: &GameData,
        art: &ArtAssets,
    ) {
        for npc in self.visible_npcs(data) {
            let runtime = self.npc_runtime_state(data, npc);
            if runtime.area_id != area.id {
                continue;
            }
            let pos = self.npc_draw_position(npc, &runtime);
            let center = vec2(offset.x + pos.x, offset.y + pos.y);
            let priority = self.npc_world_label(data, npc);
            let facing = if runtime.direction.length_squared() > 0.0 {
                runtime.direction
            } else {
                vec2(0.0, 1.0)
            };
            let show_name =
                priority.is_some()
                    || self.world.player.position.distance(pos) <= npc.interaction_radius + 54.0;
            draw_npc_world_marker(
                npc,
                center,
                facing,
                runtime.moving,
                render_color(npc.color),
                show_name,
                priority
                    .as_ref()
                    .map(|(label, tone)| (label.as_str(), render_color(tone.color()))),
                art,
            );
        }
    }
}
