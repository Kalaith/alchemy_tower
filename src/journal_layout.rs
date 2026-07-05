use crate::ui_scale::{ui_h, ui_w};
use macroquad::prelude::Rect;

pub(crate) fn journal_panel_rect() -> Rect {
    Rect::new(120.0, 72.0, ui_w() - 240.0, ui_h() - 144.0)
}

pub(crate) fn journal_tab_rect(index: usize, tab_count: usize) -> Rect {
    let panel = journal_panel_rect();
    let tab_y = panel.y + 82.0;
    let tab_w = (panel.w - 40.0) / tab_count.max(1) as f32;
    Rect::new(
        panel.x + 20.0 + tab_w * index as f32,
        tab_y,
        tab_w - 8.0,
        30.0,
    )
}

pub(crate) fn journal_close_rect() -> Rect {
    let panel = journal_panel_rect();
    Rect::new(panel.x + panel.w - 112.0, panel.y + 16.0, 92.0, 28.0)
}
