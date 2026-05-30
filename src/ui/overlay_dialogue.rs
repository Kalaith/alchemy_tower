use super::GameplayState;
use crate::content::ui_format;
use crate::data::GameData;
use crate::ui::{draw_panel, draw_wrapped_text};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_dialogue_overlay(&self, data: &GameData) {
        let Some(npc_id) = self.dialogue_npc_id() else {
            return;
        };
        let Some(npc) = data.npc(npc_id) else {
            return;
        };
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 130),
        );
        let x = 180.0;
        let y = screen_height() - 286.0;
        let w = screen_width() - 360.0;
        let h = 226.0;
        draw_panel(x, y, w, h, &npc.name);
        draw_text(
            &ui_format("overlay_now", &[("text", &self.npc_now_hint(data, npc))]),
            x + 20.0,
            y + 34.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            &ui_format(
                "overlay_later",
                &[("text", &self.npc_later_hint(data, npc))],
            ),
            x + 20.0,
            y + 54.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_wrapped_text(
            &ui_format(
                "overlay_usually",
                &[("text", &self.npc_usual_hint(data, npc))],
            ),
            x + 20.0,
            y + 72.0,
            w - 40.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        let text = self.current_dialogue_text(data, npc);
        draw_wrapped_text(
            &text,
            x + 20.0,
            y + 104.0,
            w - 40.0,
            20.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_text(
            &self.current_dialogue_footer(data, npc),
            x + 20.0,
            y + h - 28.0,
            20.0,
            dark::TEXT_DIM,
        );
    }
}
