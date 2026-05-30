use std::sync::atomic::{AtomicBool, Ordering};

use macroquad::prelude::set_fullscreen;

static FULLSCREEN_ENABLED: AtomicBool = AtomicBool::new(false);

pub(super) fn saved_fullscreen_enabled() -> bool {
    FULLSCREEN_ENABLED.load(Ordering::Relaxed)
}

pub(super) fn apply_fullscreen_enabled(enabled: bool) {
    FULLSCREEN_ENABLED.store(enabled, Ordering::Relaxed);
    set_fullscreen(enabled);
}
