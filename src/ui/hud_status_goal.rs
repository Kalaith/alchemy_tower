use super::hud_chrome::*;
use super::hud_primitives::*;
use super::HudView;
use crate::art::ArtAssets;
use super::{draw_wrapped_text, truncate_text_to_width};
use macroquad::prelude::*;

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
        &view.truncation_suffix,
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
            &view.truncation_suffix,
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
            &view.truncation_suffix,
        );
    }
}
