use crate::content::ui_copy;
use super::draw_panel;
use macroquad::prelude::*;

pub(crate) fn draw_sleep_flash_overlay_view(sleep_flash_seconds: f32) {
    if sleep_flash_seconds <= 0.0 {
        return;
    }

    let t = (sleep_flash_seconds / 1.2).clamp(0.0, 1.0);
    let pulse = ((get_time() as f32 * 16.0).sin() * 0.5 + 0.5) * t;
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(180, 22, 18, (100.0 + pulse * 110.0) as u8),
    );
    draw_panel(
        screen_width() * 0.5 - 260.0,
        screen_height() * 0.5 - 64.0,
        520.0,
        128.0,
        ui_copy("gameplay_sleep_flash_title"),
    );
    draw_text(
        ui_copy("gameplay_fainted_home"),
        screen_width() * 0.5 - 220.0,
        screen_height() * 0.5 + 10.0,
        28.0,
        Color::from_rgba(255, 236, 216, 255),
    );
}
