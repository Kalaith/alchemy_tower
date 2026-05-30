use macroquad::prelude::{screen_height, screen_width, Rect};

pub(crate) fn title_button_rect(index: usize) -> Rect {
    let button_width = if screen_width() < 760.0 { 250.0 } else { 320.0 };
    let button_height = if screen_height() < 500.0 { 40.0 } else { 48.0 };
    let gap = if screen_height() < 500.0 { 8.0 } else { 12.0 };
    let button_count = 3.0;
    let total_height = button_height * button_count + gap * (button_count - 1.0);
    let min_y: f32 = if screen_height() < 500.0 {
        152.0
    } else {
        248.0
    };
    let max_y = (screen_height() - total_height - 58.0).max(24.0);
    let start_y = (screen_height() * 0.5).clamp(min_y.min(max_y), max_y.max(min_y));

    Rect::new(
        screen_width() * 0.5 - button_width * 0.5,
        start_y + index as f32 * (button_height + gap),
        button_width,
        button_height,
    )
}

pub(crate) fn status_y() -> f32 {
    let last_button = title_button_rect(2);
    (last_button.y + last_button.h + 28.0).min(screen_height() - 28.0)
}

pub(crate) fn settings_rect() -> Rect {
    let target_width: f32 = if screen_width() < 760.0 { 320.0 } else { 420.0 };
    let width = target_width.min(screen_width() - 48.0);
    let height = 238.0_f32.min(screen_height() - 48.0);
    Rect::new(
        screen_width() * 0.5 - width * 0.5,
        screen_height() * 0.5 - height * 0.5 + 42.0,
        width,
        height,
    )
}

pub(crate) fn fullscreen_toggle_rect() -> Rect {
    let rect = settings_rect();
    Rect::new(rect.x + 24.0, rect.y + 122.0, rect.w - 48.0, 44.0)
}

pub(crate) fn settings_back_rect() -> Rect {
    let rect = settings_rect();
    Rect::new(rect.x + 24.0, rect.y + 178.0, rect.w - 48.0, 38.0)
}
