use super::{archive_tab_label, draw_overlay_tab};
use crate::content::{input_bindings, ui_copy, ui_format};
use macroquad::prelude::Rect;

pub(crate) fn draw_archive_tabs(tabs: &[&str], selected_index: usize, x: f32, y: f32, w: f32) {
    for (index, tab) in tabs.iter().enumerate() {
        let rect = Rect::new(x + 20.0 + index as f32 * 148.0, y + 54.0, 136.0, 30.0);
        draw_overlay_tab(rect, archive_tab_label(tab), selected_index == index);
        if rect.x + rect.w > x + w - 20.0 {
            break;
        }
    }
}

pub(crate) fn archive_footer_text(tab: &str) -> String {
    ui_format(match tab {
        "timeline" => "overlay_archive_footer_timeline",
        "disassembly" | "duplication" => "overlay_archive_footer_confirm",
        "experiments" => "overlay_archive_footer_filter",
        "mastery" | "morphs" => "overlay_archive_footer_browse",
        _ => "overlay_archive_footer_close",
    }, &[
        ("close", &input_bindings().global.cancel),
        ("confirm", &input_bindings().global.confirm),
        ("filter", &input_bindings().archive.filter),
        ("select", &input_bindings().navigation.select),
        ("switch", &input_bindings().navigation.switch),
    ])
}
