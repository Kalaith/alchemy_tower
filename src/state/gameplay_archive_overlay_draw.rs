use super::GameplayState;
use crate::archive_layout::archive_panel_rect;
use crate::data::GameData;
use crate::ui::{
    draw_archive_tabs, draw_overlay_backdrop, draw_overlay_footer, draw_overlay_subtitle,
    draw_panel,
};

impl GameplayState {
    pub(super) fn draw_archive_overlay(&self, data: &GameData) {
        draw_overlay_backdrop();
        let panel = archive_panel_rect();
        let x = panel.x;
        let y = panel.y;
        let w = panel.w;
        let h = panel.h;
        let chrome = self.archive_chrome_view();
        draw_panel(x, y, w, h, chrome.title);
        draw_overlay_subtitle(x, y, &chrome.subtitle);
        draw_archive_tabs(&chrome.tabs, self.ui.archive_tab, x, y, w);

        match self.archive_tab_id() {
            "timeline" => self.draw_archive_timeline_section(data, x, y, w, h),
            "experiments" => self.draw_archive_experiments_section(data, x, y, w, h),
            "mastery" => self.draw_archive_mastery_section(data, x, y, w, h),
            "morphs" => self.draw_archive_morphs_section(data, x, y, w, h),
            "disassembly" => self.draw_archive_disassembly_section(data, x, y, w, h),
            _ => self.draw_archive_duplication_section(data, x, y, w, h),
        }
        draw_overlay_footer(x, y, w, h, &chrome.footer_text);
    }
}
