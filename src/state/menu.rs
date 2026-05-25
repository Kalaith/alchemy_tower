//! Title screen state.

use std::sync::atomic::{AtomicBool, Ordering};

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::input::was_clicked;

use crate::art::{draw_texture_cover, ArtAssets};
use crate::content::ui_copy;
use crate::data::GameData;
use crate::save::SaveRepository;
use crate::state::{GameplayState, StateTransition};
use crate::ui::{draw_action_button, draw_wrapped_text, truncate_text_to_width};

static FULLSCREEN_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TitleMode {
    Actions,
    Settings,
}

pub struct MenuState {
    mode: TitleMode,
    status_text: String,
    fullscreen_enabled: bool,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            mode: TitleMode::Actions,
            status_text: String::new(),
            fullscreen_enabled: saved_fullscreen_enabled(),
        }
    }

    pub fn update(&mut self, data: &GameData) -> Option<StateTransition> {
        if self.mode == TitleMode::Settings {
            self.update_settings();
            return None;
        }

        if is_key_pressed(KeyCode::Enter) || title_button_clicked(0) {
            return Some(StateTransition::EnterGameplay(GameplayState::new(data)));
        }

        if title_button_clicked(1) {
            if !SaveRepository::exists() {
                self.status_text = ui_copy("menu_load_unavailable").to_owned();
                return None;
            }
            let mut gameplay = GameplayState::new(data);
            if gameplay.load_progress(data) {
                return Some(StateTransition::EnterGameplay(gameplay));
            }
            self.status_text = gameplay.pause_status_text().to_owned();
        }

        if title_button_clicked(2) {
            self.mode = TitleMode::Settings;
            self.status_text.clear();
        }

        None
    }

    fn update_settings(&mut self) {
        if is_key_pressed(KeyCode::Escape) || settings_back_clicked() {
            self.mode = TitleMode::Actions;
            return;
        }

        if is_key_pressed(KeyCode::F11) || fullscreen_toggle_clicked() {
            self.fullscreen_enabled = !self.fullscreen_enabled;
            remember_fullscreen_enabled(self.fullscreen_enabled);
            set_fullscreen(self.fullscreen_enabled);
            self.status_text = ui_copy(if self.fullscreen_enabled {
                "menu_fullscreen_on_status"
            } else {
                "menu_fullscreen_off_status"
            })
            .to_owned();
        }
    }

    pub fn draw(&self, data: &GameData, art: &ArtAssets) {
        let has_title_screen = draw_title_background(data, art);

        draw_title_vignette(has_title_screen);
        draw_title_text();
        if self.mode == TitleMode::Settings {
            draw_settings(self.fullscreen_enabled);
        } else {
            draw_title_buttons();
        }
        draw_title_status(&self.status_text);
    }
}

fn draw_title_background(data: &GameData, art: &ArtAssets) -> bool {
    if let Some(texture) = art.title_screen("main") {
        draw_texture_cover(
            texture,
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            WHITE,
        );
        return true;
    }

    if let Some(texture) = art.background(&data.config.starting_area) {
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            Color::from_rgba(255, 255, 255, 215),
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
    false
}

fn draw_title_vignette(has_title_screen: bool) {
    let base_alpha = if has_title_screen { 62 } else { 126 };
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(8, 10, 16, base_alpha),
    );

    let band_height = screen_height() * 0.42;
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        band_height,
        Color::from_rgba(8, 10, 16, 58),
    );
    draw_rectangle(
        0.0,
        screen_height() - band_height * 0.7,
        screen_width(),
        band_height * 0.7,
        Color::from_rgba(8, 10, 16, 86),
    );
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

fn title_button_clicked(index: usize) -> bool {
    let rect = title_button_rect(index);
    was_clicked(rect.x, rect.y, rect.w, rect.h)
}

fn title_button_rect(index: usize) -> Rect {
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

fn status_y() -> f32 {
    let last_button = title_button_rect(2);
    (last_button.y + last_button.h + 28.0).min(screen_height() - 28.0)
}

fn settings_rect() -> Rect {
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

fn fullscreen_toggle_rect() -> Rect {
    let rect = settings_rect();
    Rect::new(rect.x + 24.0, rect.y + 122.0, rect.w - 48.0, 44.0)
}

fn settings_back_rect() -> Rect {
    let rect = settings_rect();
    Rect::new(rect.x + 24.0, rect.y + 178.0, rect.w - 48.0, 38.0)
}

fn fullscreen_toggle_clicked() -> bool {
    let rect = fullscreen_toggle_rect();
    was_clicked(rect.x, rect.y, rect.w, rect.h)
}

fn settings_back_clicked() -> bool {
    let rect = settings_back_rect();
    was_clicked(rect.x, rect.y, rect.w, rect.h)
}

fn saved_fullscreen_enabled() -> bool {
    FULLSCREEN_ENABLED.load(Ordering::Relaxed)
}

fn remember_fullscreen_enabled(enabled: bool) {
    FULLSCREEN_ENABLED.store(enabled, Ordering::Relaxed);
}
