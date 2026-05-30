use super::hud_banner::*;
use super::hud_chrome::*;
use super::hud_compass::*;
use super::hud_primitives::*;
use super::HudView;
use crate::art::ArtAssets;
use crate::content::{ui_copy, ui_format};
use crate::ui::draw_wrapped_text;
use macroquad::prelude::*;

pub(super) fn draw_hud_vignette() {
    let sw = screen_width();
    let sh = screen_height();
    draw_hud_atmosphere(sw, sh);

    for band in 0..6 {
        let t = band as f32 / 5.0;
        let alpha = (95.0 * (1.0 - t)) as u8;
        let inset = band as f32 * 12.0;
        draw_rectangle(0.0, inset, sw, 12.0, Color::from_rgba(0, 0, 0, alpha));
        draw_rectangle(
            0.0,
            sh - inset - 12.0,
            sw,
            12.0,
            Color::from_rgba(0, 0, 0, alpha),
        );
        draw_rectangle(
            inset,
            0.0,
            12.0,
            sh,
            Color::from_rgba(0, 0, 0, alpha.saturating_sub(18)),
        );
        draw_rectangle(
            sw - inset - 12.0,
            0.0,
            12.0,
            sh,
            Color::from_rgba(0, 0, 0, alpha.saturating_sub(18)),
        );
    }

    draw_edge_foliage(sw, sh);
}

pub(super) fn draw_title_banner(view: &HudView) {
    let width = 500.0;
    let height = 62.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = 12.0;
    let top_plaque = Rect::new(x + 130.0, y - 8.0, width - 260.0, 24.0);
    let main = Rect::new(x, y + 14.0, width, height);

    draw_banner_backplate(main);
    draw_flourish_line(x - 72.0, y + 28.0, x + 44.0, y + 28.0, true);
    draw_flourish_line(
        x + width - 44.0,
        y + 28.0,
        x + width + 72.0,
        y + 28.0,
        false,
    );
    draw_title_vines(x, y, width);
    draw_ornate_panel(top_plaque, fill_slate(), 0.82);
    draw_title_plaque_caps(top_plaque);
    draw_centered_text_shadowed(
        ui_copy("menu_title"),
        top_plaque.x,
        top_plaque.y + 17.0,
        top_plaque.w,
        16.0,
        parchment(),
    );

    draw_ornate_panel(main, fill_slate(), 0.96);
    draw_banner_inner_hardware(main);
    draw_small_diamond(vec2(main.x + 11.0, main.y + main.h * 0.5), brass_light());
    draw_small_diamond(
        vec2(main.x + main.w - 11.0, main.y + main.h * 0.5),
        brass_light(),
    );
    draw_centered_text_shadowed(
        &truncate_text_to_width(&view.area_label, main.w - 56.0, 32.0),
        main.x,
        main.y + 43.0,
        main.w,
        34.0,
        bright_ink(),
    );
    draw_gem_mount(vec2(main.x + main.w * 0.5, main.y + main.h + 3.0));
    draw_gem(vec2(main.x + main.w * 0.5, main.y + main.h + 3.0), 13.0);
}

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
        ui_copy("hud_vitality_label"),
        center.x - radius,
        center.y - 12.0,
        radius * 2.0,
        18.0,
        Color::from_rgba(210, 244, 183, 255),
    );
    draw_centered_text_shadowed(
        &format!("{}/100", view.vitality_value),
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
    draw_text(
        ui_copy("hud_coins_label"),
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

pub(super) fn draw_goal_note(view: &HudView, art: &ArtAssets) {
    let rect = Rect::new(20.0, 188.0, 302.0, 220.0);
    let has_icon = view.goal.icon_id.is_some();
    let body_width = rect.w - if has_icon { 98.0 } else { 40.0 };
    draw_journal_note_backplate(rect);
    draw_ornate_panel(rect, Color::from_rgba(24, 25, 24, 226), 0.9);
    draw_panel_filigree(rect, 0.82);
    draw_goal_note_hardware(rect, has_icon);
    let header = Rect::new(rect.x + 13.0, rect.y + 11.0, rect.w - 26.0, 34.0);
    draw_beveled_rect(header, 7.0, Color::from_rgba(67, 51, 32, 130));
    draw_panel_texture(header, 7.0, Color::from_rgba(67, 51, 32, 130), 0.68);
    draw_beveled_rect_lines(header, 7.0, 1.0, Color::from_rgba(240, 198, 122, 96));
    draw_leaf_cluster_scaled(vec2(rect.x + rect.w - 18.0, rect.y + 17.0), true, 0.48);
    draw_small_diamond(vec2(rect.x + 18.0, rect.y + 23.0), brass_light());
    draw_text_shadowed(
        &view.goal_prefix,
        rect.x + 34.0,
        rect.y + 30.0,
        17.0,
        brass_light(),
    );
    draw_small_diamond(vec2(rect.x + 20.0, rect.y + 58.0), bright_ink());
    draw_text_shadowed(
        &truncate_text_to_width(&view.goal.title, rect.w - 56.0, 18.0),
        rect.x + 34.0,
        rect.y + 64.0,
        18.0,
        bright_ink(),
    );
    draw_ornate_divider(rect.x + 18.0, rect.y + 80.0, rect.w - 36.0, 0.68);
    draw_wrapped_text_limited(
        &view.goal.body,
        rect.x + 20.0,
        rect.y + 105.0,
        body_width,
        14.0,
        17.0,
        Color::from_rgba(218, 205, 178, 255),
        4,
    );

    if let Some(icon_id) = &view.goal.icon_id {
        draw_goal_item_badge(rect, icon_id, &view.goal.amount_text, art);
    }

    if !view.goal.detail.is_empty() {
        draw_wrapped_text_limited(
            &view.goal.detail,
            rect.x + 22.0,
            rect.y + 174.0,
            rect.w - 44.0,
            13.0,
            16.0,
            muted_ink(),
            1,
        );
    }

    if !view.goal.action.is_empty() {
        draw_goal_action_strip(rect);
        draw_small_diamond(vec2(rect.x + 23.0, rect.y + 203.0), parchment());
        draw_wrapped_text_limited(
            &view.goal.action,
            rect.x + 38.0,
            rect.y + 207.0,
            rect.w - 58.0,
            14.0,
            16.0,
            bright_ink(),
            1,
        );
    }
}

pub(super) fn draw_time_panel(view: &HudView) {
    let rect = Rect::new(screen_width() - 326.0, 22.0, 218.0, 94.0);
    draw_small_plaque_backplate(rect);
    draw_ornate_panel(rect, fill_slate(), 0.9);
    draw_panel_filigree(rect, 0.56);
    draw_time_panel_hardware(rect);
    let text_width = rect.w - 62.0;
    draw_centered_text(
        &view.season_weather_text,
        rect.x + 10.0,
        rect.y + 27.0,
        text_width,
        18.0,
        parchment(),
    );
    draw_centered_text_shadowed(
        &view.clock_text,
        rect.x + 10.0,
        rect.y + 62.0,
        text_width,
        30.0,
        bright_ink(),
    );
    draw_centered_text_shadowed(
        &view.day_text,
        rect.x + 10.0,
        rect.y + 86.0,
        text_width,
        18.0,
        bright_ink(),
    );
    draw_sun_icon(vec2(rect.x + rect.w - 34.0, rect.y + 32.0), 13.0);

    if let Some(text) = &view.sleep_warning_text {
        let warning = Rect::new(rect.x, rect.y + rect.h + 8.0, rect.w, 38.0);
        draw_ornate_panel(warning, Color::from_rgba(74, 42, 31, 214), 0.82);
        draw_wrapped_text(
            text,
            warning.x + 12.0,
            warning.y + 23.0,
            warning.w - 24.0,
            14.0,
            15.0,
            Color::from_rgba(255, 224, 168, 255),
        );
    }
}

pub(super) fn draw_minimap_frame() {
    let center = vec2(screen_width() - 62.0, 82.0);
    let radius = 62.0;
    draw_compass_backplate(center, radius);
    draw_circle(center.x + 5.0, center.y + 8.0, radius, shadow());
    draw_circle(
        center.x,
        center.y,
        radius,
        Color::from_rgba(72, 56, 42, 178),
    );
    draw_circle_lines(center.x, center.y, radius, 4.0, brass());
    draw_circle_lines(
        center.x,
        center.y,
        radius - 11.0,
        1.5,
        Color::from_rgba(230, 204, 150, 132),
    );
    draw_line(
        center.x - 42.0,
        center.y,
        center.x + 42.0,
        center.y,
        1.0,
        Color::from_rgba(230, 204, 150, 54),
    );
    draw_line(
        center.x,
        center.y - 42.0,
        center.x,
        center.y + 42.0,
        1.0,
        Color::from_rgba(230, 204, 150, 54),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius - 25.0,
        1.0,
        Color::from_rgba(230, 204, 150, 42),
    );
    draw_compass_ticks(center, radius);
    draw_compass_map_texture(center, radius);
    draw_line(
        center.x - 30.0,
        center.y - 30.0,
        center.x + 30.0,
        center.y + 30.0,
        1.0,
        Color::from_rgba(230, 204, 150, 36),
    );
    draw_line(
        center.x - 30.0,
        center.y + 30.0,
        center.x + 30.0,
        center.y - 30.0,
        1.0,
        Color::from_rgba(230, 204, 150, 36),
    );
    for marker in [
        vec2(center.x, center.y - radius + 6.0),
        vec2(center.x + radius - 6.0, center.y),
        vec2(center.x, center.y + radius - 6.0),
        vec2(center.x - radius + 6.0, center.y),
    ] {
        draw_small_diamond(marker, brass_light());
    }
    draw_text(
        ui_copy("hud_minimap_north"),
        center.x - 6.0,
        center.y - radius + 17.0,
        17.0,
        parchment(),
    );
    draw_compass_rosette(center);
    draw_triangle(
        vec2(center.x, center.y - 8.0),
        vec2(center.x - 7.0, center.y + 12.0),
        vec2(center.x + 7.0, center.y + 12.0),
        bright_ink(),
    );
    draw_leaf_cluster_scaled(center + vec2(38.0, 47.0), true, 0.72);
}
