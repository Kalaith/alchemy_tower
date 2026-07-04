use macroquad::prelude::{screen_height, screen_width, Rect};

/// The alchemy bench needs a fixed amount of room for its columns of controls.
/// Rather than stretching the panel to the whole window (which scrambled and
/// overlapped the contents on small screens and left huge dead space on large
/// ones), we size the panel to this content box and center it.
pub(crate) const ALCHEMY_CONTENT_W: f32 = 964.0;
pub(crate) const ALCHEMY_CONTENT_H: f32 = 620.0;
const ALCHEMY_MAX_W: f32 = 1200.0;
const ALCHEMY_MAX_H: f32 = 780.0;
const ALCHEMY_MARGIN: f32 = 24.0;

/// Centered, clamped panel. Never shrinks below the space the controls need, so
/// the layout stays coherent instead of overlapping when the window is small;
/// caps its size on large monitors so the text stays readable.
pub(crate) fn alchemy_panel_rect() -> Rect {
    let avail_w = screen_width() - ALCHEMY_MARGIN * 2.0;
    let avail_h = screen_height() - ALCHEMY_MARGIN * 2.0;
    let w = avail_w.clamp(ALCHEMY_CONTENT_W, ALCHEMY_MAX_W);
    let h = avail_h.clamp(ALCHEMY_CONTENT_H, ALCHEMY_MAX_H);
    let x = ((screen_width() - w) * 0.5).max(0.0);
    let y = ((screen_height() - h) * 0.5).max(0.0);
    Rect::new(x, y, w, h)
}

/// Origin of the current panel. Both rendering and mouse hit-testing derive
/// their coordinates from this so a centered/clamped panel stays in sync.
fn panel_origin() -> (f32, f32) {
    let panel = alchemy_panel_rect();
    (panel.x, panel.y)
}

pub(crate) fn material_row_rect(index: usize) -> Rect {
    let (x, y) = panel_origin();
    material_row_rect_at(x, y, index)
}

pub(crate) fn material_row_rect_at(x: f32, y: f32, index: usize) -> Rect {
    Rect::new(x + 18.0, y + 58.0 + index as f32 * 58.0, 286.0, 52.0)
}

pub(crate) fn alchemy_slot_rect(slot: usize) -> Rect {
    let (x, y) = panel_origin();
    alchemy_slot_rect_at(x, y, slot)
}

pub(crate) fn alchemy_slot_rect_at(x: f32, y: f32, slot: usize) -> Rect {
    Rect::new(x + 340.0 + slot as f32 * 140.0, y + 120.0, 120.0, 100.0)
}

pub(crate) fn catalyst_rect() -> Rect {
    let (x, y) = panel_origin();
    catalyst_rect_at(x, y)
}

pub(crate) fn catalyst_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 760.0, y + 120.0, 160.0, 100.0)
}

pub(crate) fn heat_down_rect() -> Rect {
    let (x, y) = panel_origin();
    heat_down_rect_at(x, y)
}

pub(crate) fn heat_down_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 520.0, y + 88.0, 28.0, 24.0)
}

pub(crate) fn heat_up_rect() -> Rect {
    let (x, y) = panel_origin();
    heat_up_rect_at(x, y)
}

pub(crate) fn heat_up_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 552.0, y + 88.0, 28.0, 24.0)
}

pub(crate) fn stirs_rect() -> Rect {
    let (x, y) = panel_origin();
    stirs_rect_at(x, y)
}

pub(crate) fn stirs_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 612.0, y + 88.0, 92.0, 24.0)
}

pub(crate) fn timing_rect() -> Rect {
    let (x, y) = panel_origin();
    timing_rect_at(x, y)
}

pub(crate) fn timing_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 716.0, y + 88.0, 156.0, 24.0)
}

pub(crate) fn sort_rect() -> Rect {
    let (x, y) = panel_origin();
    sort_rect_at(x, y)
}

pub(crate) fn sort_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 20.0, y + 368.0, 82.0, 28.0)
}

pub(crate) fn clear_rect() -> Rect {
    let (x, y) = panel_origin();
    clear_rect_at(x, y)
}

pub(crate) fn clear_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 114.0, y + 368.0, 82.0, 28.0)
}

pub(crate) fn repeat_rect() -> Rect {
    let (x, y) = panel_origin();
    repeat_rect_at(x, y)
}

pub(crate) fn repeat_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 208.0, y + 368.0, 90.0, 28.0)
}

pub(crate) fn brew_rect() -> Rect {
    let (x, y) = panel_origin();
    brew_rect_at(x, y)
}

pub(crate) fn brew_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + 310.0, y + 368.0, 90.0, 28.0)
}

/// Explicit close control in the panel's top-right corner. The overlay also
/// closes on Esc, but the feedback asked for a visible exit affordance.
pub(crate) fn alchemy_close_rect() -> Rect {
    let panel = alchemy_panel_rect();
    alchemy_close_rect_at(panel.x, panel.y, panel.w)
}

pub(crate) fn alchemy_close_rect_at(x: f32, y: f32, w: f32) -> Rect {
    Rect::new(x + w - 116.0, y + 16.0, 96.0, 30.0)
}
