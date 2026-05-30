use super::{draw_overlay_section_box, draw_overlay_section_title, GameplayState};
use crate::content::{input_bindings, ui_copy, ui_format, ui_text};
use crate::data::GameData;
use crate::ui::{
    draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle, draw_panel,
    draw_selection_card, draw_state_banner, draw_wrapped_text,
};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

impl GameplayState {
    pub(super) fn draw_quest_board_overlay(&self, data: &GameData) {
        draw_overlay_backdrop();
        let x = 180.0;
        let y = 90.0;
        let w = screen_width() - 360.0;
        let h = screen_height() - 180.0;
        draw_panel(x, y, w, h, ui_copy("overlay_quest_board_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.quest_board_subtitle);
        let available = self.available_board_quests(data);
        draw_overlay_section_title(
            x + 20.0,
            y + 122.0,
            ui_copy("overlay_quest_available"),
            None,
        );
        draw_overlay_section_box(x + 20.0, y + 136.0, w - 40.0, 232.0);
        let mut row_y = y + 168.0;
        if available.is_empty() {
            draw_state_banner(
                x + 32.0,
                row_y - 16.0,
                w - 64.0,
                &self.unavailable_state_text(ui_copy("overlay_quest_none_available")),
                false,
            );
        } else {
            for (index, quest_id) in available.iter().enumerate() {
                let selected = index == self.ui.shop_index;
                if let Some(quest) = data.quest(quest_id) {
                    let giver_hint = self.quest_location_hint(data, quest);
                    draw_selection_card(
                        x + 32.0,
                        row_y - 24.0,
                        w - 64.0,
                        58.0,
                        selected,
                        true,
                        &quest.title,
                        &giver_hint,
                        &ui_format(
                            "overlay_reward",
                            &[("coins", &quest.reward_coins.to_string())],
                        ),
                    );
                }
                row_y += 64.0;
            }
        }
        draw_text(
            ui_copy("overlay_quest_locked"),
            x + 20.0,
            y + h - 200.0,
            22.0,
            dark::TEXT_BRIGHT,
        );
        draw_overlay_section_box(x + 20.0, y + h - 186.0, w - 40.0, 54.0);
        let locked = data
            .quests
            .iter()
            .filter(|quest| quest.giver_npc_id == "quest_board")
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .filter(|quest| !self.quest_is_available(quest))
            .collect::<Vec<_>>();
        let locked_text = if locked.is_empty() {
            ui_copy("overlay_none").to_owned()
        } else {
            locked
                .iter()
                .map(|quest| {
                    format!(
                        "{}: {}",
                        quest.title,
                        self.locked_state_text(&self.quest_unlock_summary(quest))
                    )
                })
                .collect::<Vec<_>>()
                .join("  ")
        };
        draw_wrapped_text(
            &locked_text,
            x + 32.0,
            y + h - 164.0,
            w - 64.0,
            16.0,
            18.0,
            dark::TEXT_DIM,
        );
        draw_text(
            ui_copy("overlay_quest_active"),
            x + 20.0,
            y + h - 120.0,
            24.0,
            dark::TEXT_BRIGHT,
        );
        draw_overlay_section_box(x + 20.0, y + h - 106.0, w - 40.0, 52.0);
        let active_orders = self
            .progression
            .started_quests
            .iter()
            .filter_map(|quest_id| data.quest(quest_id))
            .map(|quest| quest.title.clone())
            .collect::<Vec<_>>();
        let active_text = if active_orders.is_empty() {
            ui_copy("overlay_none").to_owned()
        } else {
            active_orders.join(", ")
        };
        draw_wrapped_text(
            &active_text,
            x + 32.0,
            y + h - 84.0,
            w - 64.0,
            18.0,
            20.0,
            dark::TEXT_DIM,
        );
        draw_overlay_footer(
            x,
            y,
            w,
            h,
            &ui_copy("overlay_rune_footer")
                .replace("{select}", &input_bindings().navigation.select)
                .replace("{confirm}", &input_bindings().global.confirm)
                .replace("{close}", &input_bindings().global.cancel),
        );
    }


}
