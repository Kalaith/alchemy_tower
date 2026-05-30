use macroquad::prelude::{screen_height, screen_width, Rect};

const PAUSE_PANEL_WIDTH: f32 = 380.0;
const PAUSE_PANEL_HEIGHT: f32 = 300.0;

pub(crate) fn pause_panel_rect() -> Rect {
    Rect::new(
        screen_width() * 0.5 - PAUSE_PANEL_WIDTH * 0.5,
        screen_height() * 0.5 - PAUSE_PANEL_HEIGHT * 0.5,
        PAUSE_PANEL_WIDTH,
        PAUSE_PANEL_HEIGHT,
    )
}

pub(crate) fn resume_pause_button_rect() -> Rect {
    inset_rect(pause_panel_rect(), 24.0, 88.0, 150.0, 42.0)
}

pub(crate) fn save_pause_button_rect() -> Rect {
    inset_rect(pause_panel_rect(), 206.0, 88.0, 150.0, 42.0)
}

pub(crate) fn load_pause_button_rect() -> Rect {
    inset_rect(pause_panel_rect(), 24.0, 144.0, 150.0, 42.0)
}

pub(crate) fn pause_menu_button_rect() -> Rect {
    inset_rect(pause_panel_rect(), 206.0, 144.0, 150.0, 42.0)
}

fn inset_rect(panel: Rect, offset_x: f32, offset_y: f32, width: f32, height: f32) -> Rect {
    Rect::new(panel.x + offset_x, panel.y + offset_y, width, height)
}
