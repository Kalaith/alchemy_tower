use super::menu_background::{draw_title_background, draw_title_vignette};
use crate::menu_layout::{
    fullscreen_toggle_rect, settings_back_rect, settings_rect, status_y, title_button_rect,
};
use super::{draw_action_button, draw_wrapped_text, truncate_text_to_width};
use crate::art::ArtAssets;
use crate::content::ui_copy;
use crate::data::GameData;
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_menu_screen(
    data: &GameData,
    art: &ArtAssets,
    showing_settings: bool,
    fullscreen_enabled: bool,
    status_text: &str,
) {
    let has_title_screen = draw_title_background(data, art);
    draw_title_vignette(has_title_screen);
    draw_title_text();
    if showing_settings {
        draw_settings(fullscreen_enabled);
    } else {
        draw_title_buttons();
    }
    draw_title_status(status_text);
}

fn draw_title_text() {
    let title_size = if screen_width() < 760.0 { 48.0 } else { 72.0 };
    let subtitle_size = if screen_width() < 760.0 { 21.0 } else { 26.0 };
    let title_y = if screen_height() < 500.0 { 78.0 } else { 128.0 };

    draw_centered_shadow_text(
        ui_copy("menu_title"),
        title_y,
        title_size,
        dark::TEXT_BRIGHT,
    );
    draw_centered_shadow_text(
        ui_copy("menu_subtitle"),
        title_y + title_size * 0.74,
        subtitle_size,
        Color::from_rgba(244, 230, 194, 255),
    );
}

fn draw_title_buttons() {
    for (index, label) in [
        ui_copy("menu_new_game"),
        ui_copy("menu_load_game"),
        ui_copy("menu_settings"),
    ]
    .iter()
    .enumerate()
    {
        draw_action_button(title_button_rect(index), label, 24.0);
    }
}

fn draw_settings(fullscreen_enabled: bool) {
    let rect = settings_rect();
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(10, 12, 18, 172),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        Color::from_rgba(240, 218, 168, 118),
    );
    draw_text(
        ui_copy("menu_settings_title"),
        rect.x + 24.0,
        rect.y + 42.0,
        30.0,
        dark::TEXT_BRIGHT,
    );
    draw_wrapped_text(
        ui_copy("menu_settings_hint"),
        rect.x + 24.0,
        rect.y + 74.0,
        rect.w - 48.0,
        18.0,
        19.0,
        Color::from_rgba(238, 231, 214, 224),
    );

    draw_action_button(
        fullscreen_toggle_rect(),
        if fullscreen_enabled {
            ui_copy("menu_fullscreen_on")
        } else {
            ui_copy("menu_fullscreen_off")
        },
        24.0,
    );
    draw_action_button(settings_back_rect(), ui_copy("menu_settings_back"), 24.0);
}

fn draw_title_status(status_text: &str) {
    if status_text.is_empty() {
        return;
    }

    let safe = truncate_text_to_width(status_text, screen_width() - 72.0, 20.0);
    let measured = measure_text(&safe, None, 20, 1.0);
    let x = screen_width() * 0.5 - measured.width * 0.5;
    let y = status_y();

    draw_text(
        &safe,
        x + 1.0,
        y + 1.0,
        20.0,
        Color::from_rgba(0, 0, 0, 190),
    );
    draw_text(&safe, x, y, 20.0, Color::from_rgba(238, 231, 214, 230));
}

fn draw_centered_shadow_text(text: &str, y: f32, font_size: f32, color: Color) {
    let safe = truncate_text_to_width(text, screen_width() - 64.0, font_size);
    let measured = measure_text(&safe, None, font_size as u16, 1.0);
    let x = screen_width() * 0.5 - measured.width * 0.5;

    for (offset_x, offset_y) in [(-2.0, 2.0), (2.0, 2.0), (0.0, 4.0)] {
        draw_text(
            &safe,
            x + offset_x,
            y + offset_y,
            font_size,
            Color::from_rgba(0, 0, 0, 180),
        );
    }
    draw_text(&safe, x, y, font_size, color);
}
