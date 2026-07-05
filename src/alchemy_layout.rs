use crate::ui_scale::{ui_h, ui_w};
use macroquad::prelude::Rect;

/// The alchemy bench needs a fixed amount of room for its columns of controls.
/// Rather than stretching the panel to the whole window (which scrambled and
/// overlapped the contents on small screens and left huge dead space on large
/// ones), we size the panel to this content box and center it.
pub(crate) const ALCHEMY_CONTENT_W: f32 = 964.0;
pub(crate) const ALCHEMY_CONTENT_H: f32 = 620.0;
const ALCHEMY_MAX_W: f32 = 1200.0;
const ALCHEMY_MAX_H: f32 = 780.0;
const ALCHEMY_MARGIN: f32 = 24.0;

// --- Internal layout grid (all offsets are relative to the panel origin) ------
// Two columns: a left column (materials / controls / formulae) and a right
// column (slots + process controls / actions / preview). Kept here as named
// constants so rendering and mouse hit-testing share one source of truth.

/// Left column x offset and width.
pub(crate) const AL_LX: f32 = 20.0;
pub(crate) const AL_LW: f32 = 300.0;
/// Right column x offset; its width is `panel_w - AL_RW_MARGIN`.
pub(crate) const AL_RX: f32 = 340.0;
pub(crate) const AL_RW_MARGIN: f32 = 360.0;
/// Gap kept between a bottom-anchored section box and the panel's bottom edge.
pub(crate) const AL_BOX_BOTTOM_MARGIN: f32 = 56.0;

// Left column bands.
pub(crate) const AL_MAT_TITLE_Y: f32 = 112.0;
pub(crate) const AL_MAT_BOX_Y: f32 = 120.0;
pub(crate) const AL_MAT_BOX_H: f32 = 172.0;
pub(crate) const AL_MAT_ROW_STEP: f32 = 42.0;
pub(crate) const AL_MAT_ROW_H: f32 = 38.0;
pub(crate) const AL_MAT_VISIBLE_ROWS: usize = 4;
pub(crate) const AL_FORM_TITLE_Y: f32 = 318.0;
pub(crate) const AL_FORM_BOX_Y: f32 = 326.0;

// Right column bands.
pub(crate) const AL_SLOT_TITLE_Y: f32 = 112.0;
pub(crate) const AL_SLOT_BOX_Y: f32 = 120.0;
pub(crate) const AL_SLOT_BOX_H: f32 = 164.0;
pub(crate) const AL_PROC_READOUT_Y: f32 = 140.0;
pub(crate) const AL_PREV_TITLE_Y: f32 = 348.0;
pub(crate) const AL_PREV_BOX_Y: f32 = 356.0;

/// Centered, clamped panel. Never shrinks below the space the controls need, so
/// the layout stays coherent instead of overlapping when the window is small;
/// caps its size on large monitors so the text stays readable.
pub(crate) fn alchemy_panel_rect() -> Rect {
    let avail_w = ui_w() - ALCHEMY_MARGIN * 2.0;
    let avail_h = ui_h() - ALCHEMY_MARGIN * 2.0;
    let w = avail_w.clamp(ALCHEMY_CONTENT_W, ALCHEMY_MAX_W);
    let h = avail_h.clamp(ALCHEMY_CONTENT_H, ALCHEMY_MAX_H);
    let x = ((ui_w() - w) * 0.5).max(0.0);
    let y = ((ui_h() - h) * 0.5).max(0.0);
    Rect::new(x, y, w, h)
}

/// Origin of the current panel. Both rendering and mouse hit-testing derive
/// their coordinates from this so a centered/clamped panel stays in sync.
fn panel_origin() -> (f32, f32) {
    let panel = alchemy_panel_rect();
    (panel.x, panel.y)
}

/// Width of the right column for the current panel width.
pub(crate) fn right_column_width(panel_w: f32) -> f32 {
    panel_w - AL_RW_MARGIN
}

pub(crate) fn material_row_rect(index: usize) -> Rect {
    let (x, y) = panel_origin();
    material_row_rect_at(x, y, index)
}

pub(crate) fn material_row_rect_at(x: f32, y: f32, index: usize) -> Rect {
    Rect::new(
        x + AL_LX,
        y + AL_MAT_BOX_Y + 8.0 + index as f32 * AL_MAT_ROW_STEP,
        AL_LW - 8.0,
        AL_MAT_ROW_H,
    )
}

pub(crate) fn alchemy_slot_rect(slot: usize) -> Rect {
    let (x, y) = panel_origin();
    alchemy_slot_rect_at(x, y, slot)
}

pub(crate) fn alchemy_slot_rect_at(x: f32, y: f32, slot: usize) -> Rect {
    Rect::new(
        x + AL_RX + 8.0 + slot as f32 * 126.0,
        y + 182.0,
        118.0,
        96.0,
    )
}

pub(crate) fn catalyst_rect() -> Rect {
    let (x, y) = panel_origin();
    catalyst_rect_at(x, y)
}

pub(crate) fn catalyst_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 400.0, y + 182.0, 150.0, 96.0)
}

pub(crate) fn heat_down_rect() -> Rect {
    let (x, y) = panel_origin();
    heat_down_rect_at(x, y)
}

pub(crate) fn heat_down_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 8.0, y + 150.0, 42.0, 26.0)
}

pub(crate) fn heat_up_rect() -> Rect {
    let (x, y) = panel_origin();
    heat_up_rect_at(x, y)
}

pub(crate) fn heat_up_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 54.0, y + 150.0, 42.0, 26.0)
}

pub(crate) fn stirs_rect() -> Rect {
    let (x, y) = panel_origin();
    stirs_rect_at(x, y)
}

pub(crate) fn stirs_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 104.0, y + 150.0, 100.0, 26.0)
}

pub(crate) fn timing_rect() -> Rect {
    let (x, y) = panel_origin();
    timing_rect_at(x, y)
}

pub(crate) fn timing_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 212.0, y + 150.0, 156.0, 26.0)
}

pub(crate) fn sort_rect() -> Rect {
    let (x, y) = panel_origin();
    sort_rect_at(x, y)
}

pub(crate) fn sort_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 8.0, y + 292.0, 82.0, 28.0)
}

pub(crate) fn clear_rect() -> Rect {
    let (x, y) = panel_origin();
    clear_rect_at(x, y)
}

pub(crate) fn clear_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 98.0, y + 292.0, 82.0, 28.0)
}

pub(crate) fn repeat_rect() -> Rect {
    let (x, y) = panel_origin();
    repeat_rect_at(x, y)
}

pub(crate) fn repeat_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 188.0, y + 292.0, 92.0, 28.0)
}

pub(crate) fn brew_rect() -> Rect {
    let (x, y) = panel_origin();
    brew_rect_at(x, y)
}

pub(crate) fn brew_rect_at(x: f32, y: f32) -> Rect {
    Rect::new(x + AL_RX + 290.0, y + 292.0, 110.0, 28.0)
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
