use macroquad::prelude::{screen_height, screen_width, Rect};

pub(crate) fn archive_panel_rect() -> Rect {
    Rect::new(150.0, 70.0, screen_width() - 300.0, screen_height() - 140.0)
}

pub(crate) fn archive_tab_rect_at(x: f32, y: f32, index: usize) -> Rect {
    Rect::new(x + 20.0 + index as f32 * 148.0, y + 54.0, 136.0, 30.0)
}
