//! Uniform down-scaling of the on-screen UI (HUD + modal overlays) for windows
//! smaller than the reference resolution.
//!
//! The HUD and overlays are authored for [`UI_DESIGN_W`] x [`UI_DESIGN_H`]. When
//! the window is at least that big everything uses real screen coordinates and
//! this module is a no-op (so large/normal windows are unaffected). When the
//! window is smaller, the UI is laid out at the design size and rendered into a
//! centered, uniformly scaled viewport (letterbox) so its fixed-size panels stay
//! readable and never overlap. Mouse input for overlays is transformed back
//! through the same scale so clicks still land on the right controls.

use macroquad::prelude::*;
use std::cell::Cell;

pub(crate) const UI_DESIGN_W: f32 = 1280.0;
pub(crate) const UI_DESIGN_H: f32 = 720.0;

thread_local! {
    // True while overlay mouse coordinates are being read, so the transform only
    // applies to gameplay overlays (never the menu/pause screens, which lay out
    // at the real window size).
    static OVERLAY_MOUSE: Cell<bool> = const { Cell::new(false) };
}

/// `(offset_x, offset_y, scale)` of the centered letterbox viewport, or `None`
/// when the window is at least design size and no scaling is applied.
fn params() -> Option<(f32, f32, f32)> {
    let sw = screen_width();
    let sh = screen_height();
    if sw >= UI_DESIGN_W && sh >= UI_DESIGN_H {
        return None;
    }
    let scale = (sw / UI_DESIGN_W).min(sh / UI_DESIGN_H).max(0.1);
    let vw = UI_DESIGN_W * scale;
    let vh = UI_DESIGN_H * scale;
    Some(((sw - vw) * 0.5, (sh - vh) * 0.5, scale))
}

pub(crate) fn is_scaling() -> bool {
    params().is_some()
}

/// Layout width the UI anchors to: the design width when scaling, else the real
/// window width.
pub(crate) fn ui_w() -> f32 {
    if params().is_some() {
        UI_DESIGN_W
    } else {
        screen_width()
    }
}

pub(crate) fn ui_h() -> f32 {
    if params().is_some() {
        UI_DESIGN_H
    } else {
        screen_height()
    }
}

/// Install the scaled UI camera. Returns whether a camera was set (i.e. whether
/// scaling is active); pass that value to [`end_ui_camera`].
pub(crate) fn begin_ui_camera() -> bool {
    let Some((ox, oy, scale)) = params() else {
        return false;
    };
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, UI_DESIGN_W, UI_DESIGN_H));
    // `from_display_rect` orients for the default framebuffer; attaching a
    // viewport makes macroquad treat it as an offscreen target, which flips Y.
    // Undo that so the scaled UI keeps a top-left origin.
    camera.zoom.y = -camera.zoom.y;
    camera.viewport = Some((
        ox as i32,
        oy as i32,
        (UI_DESIGN_W * scale) as i32,
        (UI_DESIGN_H * scale) as i32,
    ));
    set_camera(&camera);
    true
}

pub(crate) fn end_ui_camera(active: bool) {
    if active {
        set_default_camera();
    }
}

/// Marks whether subsequent mouse reads belong to a gameplay overlay (and so
/// should be transformed into design space when scaling).
pub(crate) fn set_overlay_mouse(active: bool) {
    OVERLAY_MOUSE.with(|flag| flag.set(active));
}

/// Transforms a raw window-space mouse position into UI design space. Identity
/// unless an overlay is being read *and* the UI is currently scaled.
pub(crate) fn transform_mouse(point: [f32; 2]) -> [f32; 2] {
    if !OVERLAY_MOUSE.with(|flag| flag.get()) {
        return point;
    }
    match params() {
        Some((ox, oy, scale)) => [(point[0] - ox) / scale, (point[1] - oy) / scale],
        None => point,
    }
}
