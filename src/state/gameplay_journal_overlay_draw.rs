use super::GameplayState;
use crate::art::ArtAssets;
use crate::content::ui_copy;
use crate::data::GameData;
use crate::ui::{
    draw_journal_backdrop, draw_journal_close_button, draw_journal_current_conditions,
    draw_journal_footer, draw_journal_tabs, draw_panel,
};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_field_journal(&self, data: &GameData, art: &ArtAssets) {
        draw_journal_backdrop();
        let x = 120.0;
        let y = 72.0;
        let w = screen_width() - 240.0;
        let h = screen_height() - 144.0;
        draw_panel(x, y, w, h, ui_copy("overlay_journal_title"));
        draw_journal_close_button(self.journal_close_rect());
        draw_journal_current_conditions(self.current_season(), self.current_weather(), x, y);
        let tabs = self.journal_tabs();
        let tab_rects: Vec<Rect> = (0..tabs.len())
            .map(|index| self.journal_tab_rect(index, tabs.len()))
            .collect();
        draw_journal_tabs(&tabs, self.journal_tab_index(), &tab_rects, art);

        match self.journal_tab_index() {
            0 => self.draw_journal_routes_tab(data, x, y, w, h),
            1 => self.draw_journal_notes_tab(data, x, y, w, h),
            2 => self.draw_journal_brews_tab(data, x, y, w, h),
            3 if self.greenhouse_journal_unlocked() => {
                self.draw_journal_greenhouse_tab(data, x, y, w, h)
            }
            _ => self.draw_journal_rapport_tab(data, x, y, w, h),
        }
        draw_journal_footer(x, y, h);
    }
}
