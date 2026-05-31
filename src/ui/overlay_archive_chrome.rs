use super::draw_overlay_tab;
use crate::archive_layout::archive_tab_rect_at;

pub(crate) fn draw_archive_tabs(tabs: &[&str], selected_index: usize, x: f32, y: f32, w: f32) {
    for (index, tab_label) in tabs.iter().enumerate() {
        let rect = archive_tab_rect_at(x, y, index);
        draw_overlay_tab(rect, tab_label, selected_index == index);
        if rect.x + rect.w > x + w - 20.0 {
            break;
        }
    }
}
