use super::hud::{draw_beveled_rect, draw_beveled_rect_lines};
use super::draw_action_button;
use crate::alchemy_layout::{brew_rect_at, clear_rect_at, repeat_rect_at, sort_rect_at};
use crate::view_models::alchemy::AlchemyActionButtonsView;
use macroquad::prelude::Color;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub(crate) fn draw_alchemy_action_buttons(view: &AlchemyActionButtonsView, x: f32, y: f32) {
    draw_action_button(sort_rect_at(x, y), view.sort_label, 0.0);
    draw_action_button(clear_rect_at(x, y), view.clear_label, 0.0);
    draw_action_button(repeat_rect_at(x, y), view.repeat_label, 0.0);
    // Brew is the primary action: same bevel, but a warm green fill so it reads
    // as the confirm button rather than another neutral control.
    let brew = brew_rect_at(x, y);
    let bevel = 5.0;
    draw_beveled_rect(brew, bevel, Color::from_rgba(40, 62, 46, 236));
    draw_beveled_rect_lines(brew, bevel, 1.5, Color::from_rgba(188, 255, 220, 150));
    let label_w = measure_ui_text(view.brew_label, None, 18, 1.0).width;
    draw_ui_text(
        view.brew_label,
        brew.x + (brew.w - label_w) * 0.5,
        brew.y + 19.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}
