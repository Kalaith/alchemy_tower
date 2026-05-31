use macroquad::prelude::{screen_height, screen_width, Rect};

pub(crate) const ALCHEMY_PANEL_X: f32 = 80.0;
pub(crate) const ALCHEMY_PANEL_Y: f32 = 64.0;

pub(crate) fn alchemy_panel_rect() -> Rect {
    Rect::new(
        ALCHEMY_PANEL_X,
        ALCHEMY_PANEL_Y,
        screen_width() - ALCHEMY_PANEL_X * 2.0,
        screen_height() - ALCHEMY_PANEL_Y * 2.0,
    )
}

pub(crate) fn material_row_rect(index: usize) -> Rect {
    material_row_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y, index)
}

pub(crate) fn material_row_rect_at(x: f32, y: f32, index: usize) -> Rect {
    Rect::new(x + 18.0, y + 58.0 + index as f32 * 58.0, 286.0, 52.0)
}

pub(crate) fn alchemy_slot_rect(slot: usize) -> Rect {
    alchemy_slot_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y, slot)
}

pub(crate) fn alchemy_slot_rect_at(x: f32, y: f32, slot: usize) -> Rect {
    Rect::new(x + 340.0 + slot as f32 * 140.0, y + 120.0, 120.0, 100.0)
}

pub(crate) fn catalyst_rect() -> Rect {
    catalyst_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn catalyst_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 760.0, y + 120.0, 160.0, 100.0)
}

pub(crate) fn heat_down_rect() -> Rect {
    heat_down_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn heat_down_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 520.0, y + 88.0, 28.0, 24.0)
}

pub(crate) fn heat_up_rect() -> Rect {
    heat_up_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn heat_up_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 552.0, y + 88.0, 28.0, 24.0)
}

pub(crate) fn stirs_rect() -> Rect {
    stirs_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn stirs_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 612.0, y + 88.0, 92.0, 24.0)
}

pub(crate) fn timing_rect() -> Rect {
    timing_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn timing_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 716.0, y + 88.0, 156.0, 24.0)
}

pub(crate) fn sort_rect() -> Rect {
    sort_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn sort_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 20.0, y + 368.0, 82.0, 28.0)
}

pub(crate) fn clear_rect() -> Rect {
    clear_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn clear_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 114.0, y + 368.0, 82.0, 28.0)
}

pub(crate) fn repeat_rect() -> Rect {
    repeat_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn repeat_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 208.0, y + 368.0, 90.0, 28.0)
}

pub(crate) fn brew_rect() -> Rect {
    brew_rect_at(ALCHEMY_PANEL_X, ALCHEMY_PANEL_Y)
}

pub(crate) fn brew_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 310.0, y + 368.0, 90.0, 28.0)
}
