use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub fn draw_panel(x: f32, y: f32, width: f32, height: f32, title: &str) {
    draw_rectangle(x, y, width, height, dark::PANEL);
    draw_rectangle(x, y, width, 28.0, dark::PANEL_HEADER);
    draw_rectangle_lines(x, y, width, height, 2.0, dark::ACCENT);
    draw_text(title, x + 12.0, y + 20.0, 22.0, dark::TEXT_BRIGHT);
}

pub fn centered_panel_rect(width: f32, height: f32) -> Rect {
    Rect::new(
        screen_width() * 0.5 - width * 0.5,
        screen_height() * 0.5 - height * 0.5,
        width,
        height,
    )
}

pub fn inset_rect(panel: Rect, offset_x: f32, offset_y: f32, width: f32, height: f32) -> Rect {
    Rect::new(panel.x + offset_x, panel.y + offset_y, width, height)
}

pub fn draw_panel_frame(panel: Rect) {
    draw_rectangle(panel.x, panel.y, panel.w, panel.h, dark::PANEL);
    draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 2.0, dark::ACCENT);
}

pub fn draw_overlay_backdrop() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 150),
    );
}

pub fn draw_overlay_subtitle(x: f32, y: f32, text: &str) {
    draw_text(text, x + 20.0, y + 52.0, 24.0, dark::TEXT_DIM);
}

pub fn draw_overlay_footer(x: f32, y: f32, w: f32, h: f32, text: &str) {
    draw_rectangle(
        x + 16.0,
        y + h - 38.0,
        w - 32.0,
        24.0,
        Color::from_rgba(24, 26, 34, 255),
    );
    draw_text(text, x + 24.0, y + h - 20.0, 18.0, dark::TEXT_DIM);
}
