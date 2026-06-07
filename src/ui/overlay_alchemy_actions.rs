use super::draw_action_button;
use crate::alchemy_layout::{brew_rect_at, clear_rect_at, repeat_rect_at, sort_rect_at};
use crate::view_models::alchemy::AlchemyActionButtonsView;
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, draw_text, Color};
use macroquad_toolkit::colors::dark;

pub(crate) fn draw_alchemy_action_buttons(view: &AlchemyActionButtonsView, x: f32, y: f32) {
    draw_action_button(sort_rect_at(x, y), view.sort_label, 0.0);
    draw_action_button(clear_rect_at(x, y), view.clear_label, 0.0);
    draw_action_button(repeat_rect_at(x, y), view.repeat_label, 0.0);
    let brew = brew_rect_at(x, y);
    draw_rectangle(
        brew.x,
        brew.y,
        brew.w,
        brew.h,
        Color::from_rgba(38, 58, 46, 210),
    );
    draw_rectangle_lines(
        brew.x,
        brew.y,
        brew.w,
        brew.h,
        1.5,
        Color::from_rgba(188, 255, 220, 96),
    );
    draw_text(
        view.brew_label,
        brew.x + 28.0,
        brew.y + 20.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
}
