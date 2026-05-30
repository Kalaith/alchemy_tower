use super::GameplayState;
use crate::content::{ui_copy, ui_text};
use crate::data::GameData;
use crate::ui::{
    archive_footer_text, draw_archive_tabs, draw_overlay_backdrop, draw_overlay_footer,
    draw_overlay_subtitle, draw_panel,
};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn draw_archive_overlay(&self, data: &GameData) {
        draw_overlay_backdrop();
        let x = 150.0;
        let y = 70.0;
        let w = screen_width() - 300.0;
        let h = screen_height() - 140.0;
        draw_panel(x, y, w, h, ui_copy("overlay_archive_title"));
        draw_overlay_subtitle(x, y, &ui_text().overlays.archive_subtitle);
        draw_archive_tabs(self.archive_tabs(), self.ui.archive_tab, x, y, w);

        match self.archive_tab_id() {
            "timeline" => self.draw_archive_timeline_section(x, y, w, h),
            "experiments" => self.draw_archive_experiments_section(data, x, y, w, h),
            "mastery" => self.draw_archive_mastery_section(data, x, y, w, h),
            "morphs" => self.draw_archive_morphs_section(data, x, y, w, h),
            "disassembly" => self.draw_archive_disassembly_section(data, x, y, w, h),
            _ => self.draw_archive_duplication_section(data, x, y, w, h),
        }
        let footer = archive_footer_text(self.archive_tab_id());
        draw_overlay_footer(x, y, w, h, &footer);
    }
}
