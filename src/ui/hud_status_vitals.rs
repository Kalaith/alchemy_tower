use super::hud_chrome::*;
use super::hud_primitives::*;
use super::HudView;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub(super) fn draw_vitality_medallion(view: &HudView) {
    let center = vec2(86.0, 88.0);
    let radius = 60.0;
    draw_medallion_backplate(center, radius);
    draw_circle(center.x + 5.0, center.y + 8.0, radius + 6.0, shadow());
    draw_circle(
        center.x,
        center.y,
        radius,
        Color::from_rgba(22, 45, 34, 232),
    );
    draw_circle_lines(center.x, center.y, radius + 2.0, 4.0, brass());
    draw_circle_lines(
        center.x,
        center.y,
        radius - 8.0,
        2.0,
        Color::from_rgba(229, 184, 92, 150),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius - 17.0,
        1.0,
        Color::from_rgba(89, 207, 171, 132),
    );
    draw_medallion_ticks(center, radius);
    draw_circle_arc(
        center,
        radius - 5.0,
        -125.0,
        80.0,
        4.0,
        Color::from_rgba(103, 190, 116, 220),
    );
    draw_circle_arc(
        center,
        radius - 5.0,
        104.0,
        42.0,
        3.0,
        Color::from_rgba(242, 205, 126, 180),
    );
    draw_leaf_cluster(center + vec2(-45.0, 42.0), false);
    draw_leaf_cluster(center + vec2(42.0, -44.0), true);
    draw_flower(vec2(center.x + 48.0, center.y - 40.0), 0.8);
    draw_flower(vec2(center.x - 52.0, center.y + 38.0), 0.58);
    draw_centered_text_shadowed(
        &view.vitality_label,
        center.x - radius,
        center.y - 12.0,
        radius * 2.0,
        18.0,
        Color::from_rgba(210, 244, 183, 255),
    );
    draw_centered_text_shadowed(
        &view.vitality_text,
        center.x - radius,
        center.y + 26.0,
        radius * 2.0,
        30.0,
        bright_ink(),
    );
}

pub(super) fn draw_coin_chip(view: &HudView) {
    let rect = Rect::new(176.0, 86.0, 132.0, 54.0);
    draw_coin_chip_backplate(rect);
    draw_coin_chip_connector(rect);
    draw_ornate_panel(rect, fill_slate(), 0.9);
    draw_panel_filigree(rect, 0.46);
    draw_coin_face(vec2(rect.x + 30.0, rect.y + 27.0));
    draw_ui_text(
        &view.coins_label,
        rect.x + 54.0,
        rect.y + 22.0,
        15.0,
        brass_light(),
    );
    draw_text_shadowed(
        &view.coins_value,
        rect.x + 64.0,
        rect.y + 43.0,
        22.0,
        bright_ink(),
    );
}
