use crate::ui_scale::{ui_h, ui_w};
use macroquad::prelude::Rect;

pub(super) fn standard_overlay_panel_rect() -> Rect {
    Rect::new(180.0, 90.0, ui_w() - 360.0, ui_h() - 180.0)
}
