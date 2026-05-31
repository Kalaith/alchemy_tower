use macroquad::prelude::{screen_height, screen_width, Rect};

pub(super) fn standard_overlay_panel_rect() -> Rect {
    Rect::new(180.0, 90.0, screen_width() - 360.0, screen_height() - 180.0)
}
