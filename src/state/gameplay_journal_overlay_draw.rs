use super::GameplayState;
use crate::art::ArtAssets;
use crate::data::GameData;
use crate::journal_layout::{journal_close_rect, journal_panel_rect, journal_tab_rect};
use crate::ui::{
    draw_journal_backdrop, draw_journal_close_button, draw_journal_current_conditions,
    draw_journal_footer, draw_journal_tabs, draw_panel,
};

impl GameplayState {
    pub(super) fn draw_field_journal(&self, data: &GameData, art: &ArtAssets) {
        draw_journal_backdrop();
        let panel = journal_panel_rect();
        let x = panel.x;
        let y = panel.y;
        let w = panel.w;
        let h = panel.h;
        let chrome = self.journal_chrome_view();
        draw_panel(x, y, w, h, chrome.title);
        draw_journal_close_button(journal_close_rect(), chrome.close_label);
        draw_journal_current_conditions(&chrome.current_conditions_text, x, y);
        let tab_rects: Vec<_> = (0..chrome.tabs.len())
            .map(|index| journal_tab_rect(index, chrome.tabs.len()))
            .collect();
        draw_journal_tabs(&chrome.tabs, self.journal_tab_index(), &tab_rects, art);

        match self.journal_tab_index() {
            0 => self.draw_journal_routes_tab(data, x, y, w, h),
            1 => self.draw_journal_notes_tab(data, x, y, w, h),
            2 => self.draw_journal_brews_tab(data, x, y, w, h),
            3 if self.greenhouse_journal_unlocked() => {
                self.draw_journal_greenhouse_tab(data, x, y, w, h)
            }
            _ => self.draw_journal_rapport_tab(data, x, y, w, h),
        }
        draw_journal_footer(&chrome.footer_text, x, y, h);
    }
}
