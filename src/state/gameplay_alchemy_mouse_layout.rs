use macroquad::prelude::Rect;

const ALCHEMY_PANEL_X: f32 = 80.0;
const ALCHEMY_PANEL_Y: f32 = 64.0;

pub(super) fn material_row_rect(index: usize) -> Rect {
    Rect::new(
        ALCHEMY_PANEL_X + 18.0,
        ALCHEMY_PANEL_Y + 58.0 + index as f32 * 58.0,
        286.0,
        52.0,
    )
}

pub(super) fn alchemy_slot_rect(slot: usize) -> Rect {
    Rect::new(
        ALCHEMY_PANEL_X + 340.0 + slot as f32 * 140.0,
        ALCHEMY_PANEL_Y + 120.0,
        120.0,
        100.0,
    )
}

pub(super) fn catalyst_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 760.0, ALCHEMY_PANEL_Y + 120.0, 160.0, 100.0)
}

pub(super) fn heat_down_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 520.0, ALCHEMY_PANEL_Y + 88.0, 28.0, 24.0)
}

pub(super) fn heat_up_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 552.0, ALCHEMY_PANEL_Y + 88.0, 28.0, 24.0)
}

pub(super) fn stirs_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 612.0, ALCHEMY_PANEL_Y + 88.0, 92.0, 24.0)
}

pub(super) fn timing_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 716.0, ALCHEMY_PANEL_Y + 88.0, 156.0, 24.0)
}

pub(super) fn sort_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 20.0, ALCHEMY_PANEL_Y + 368.0, 82.0, 28.0)
}

pub(super) fn clear_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 114.0, ALCHEMY_PANEL_Y + 368.0, 82.0, 28.0)
}

pub(super) fn repeat_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 208.0, ALCHEMY_PANEL_Y + 368.0, 90.0, 28.0)
}

pub(super) fn brew_rect() -> Rect {
    Rect::new(ALCHEMY_PANEL_X + 310.0, ALCHEMY_PANEL_Y + 368.0, 90.0, 28.0)
}
