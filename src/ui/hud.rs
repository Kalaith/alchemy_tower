use crate::art::{draw_texture_centered, ArtAssets};
use crate::view_models::hud::{HudFeedbackView, HudPotionSlot, HudView, HOTBAR_SLOT_COUNT};
use macroquad::prelude::*;

#[path = "hud_atmosphere.rs"]
mod hud_atmosphere;
#[path = "hud_banner.rs"]
mod hud_banner;
#[path = "hud_banner_hardware.rs"]
mod hud_banner_hardware;
#[path = "hud_belt.rs"]
mod hud_belt;
#[path = "hud_belt_frame.rs"]
mod hud_belt_frame;
#[path = "hud_belt_slots.rs"]
mod hud_belt_slots;
#[path = "hud_botanical_icons.rs"]
mod hud_botanical_icons;
#[path = "hud_chrome.rs"]
mod hud_chrome;
#[path = "hud_chrome_coin.rs"]
mod hud_chrome_coin;
#[path = "hud_chrome_filigree.rs"]
mod hud_chrome_filigree;
#[path = "hud_chrome_goal.rs"]
mod hud_chrome_goal;
#[path = "hud_chrome_medallion.rs"]
mod hud_chrome_medallion;
#[path = "hud_chrome_plaque.rs"]
mod hud_chrome_plaque;
#[path = "hud_chrome_tag.rs"]
mod hud_chrome_tag;
#[path = "hud_compass.rs"]
mod hud_compass;
#[path = "hud_control_tags.rs"]
mod hud_control_tags;
#[path = "hud_decor.rs"]
mod hud_decor;
#[path = "hud_gem_icons.rs"]
mod hud_gem_icons;
#[path = "hud_gem_mount.rs"]
mod hud_gem_mount;
#[path = "hud_header.rs"]
mod hud_header;
#[path = "hud_icons.rs"]
mod hud_icons;
#[path = "hud_palette.rs"]
mod hud_palette;
#[path = "hud_primitives.rs"]
mod hud_primitives;
#[path = "hud_shapes.rs"]
mod hud_shapes;
#[path = "hud_side.rs"]
mod hud_side;
#[path = "hud_side_hardware.rs"]
mod hud_side_hardware;
#[path = "hud_status.rs"]
mod hud_status;
#[path = "hud_status_goal.rs"]
mod hud_status_goal;
#[path = "hud_status_time.rs"]
mod hud_status_time;
#[path = "hud_status_vitals.rs"]
mod hud_status_vitals;
#[path = "hud_text.rs"]
mod hud_text;

use self::hud_belt::*;
use self::hud_control_tags::*;
use self::hud_header::*;
use self::hud_side::*;
use self::hud_status::*;
use super::text::{draw_wrapped_text, truncate_text_to_width};

// Ornate chrome primitives shared with the overlay panels so screens like the
// alchemy bench match the HUD's beveled, gilded framing.
pub(crate) use self::hud_chrome_filigree::draw_panel_filigree;
pub(crate) use self::hud_palette::{brass_light, bright_ink, fill_slate};
pub(crate) use self::hud_shapes::{
    draw_beveled_rect, draw_beveled_rect_lines, draw_ornate_panel, draw_panel_texture,
};

pub(crate) fn draw_hud_view(view: &HudView, art: &ArtAssets) {
    draw_hud_vignette();
    draw_title_banner(view);
    draw_vitality_medallion(view);
    draw_coin_chip(view);
    draw_goal_note(view, art);
    draw_time_panel(view);
    draw_minimap_frame(view);
    draw_side_status_panel(view);
    draw_control_tags(view);
    draw_potion_belt(view, art);
    draw_status_strip(view);
    draw_hud_feedbacks(&view.feedbacks, art);
}

fn draw_hud_feedbacks(feedbacks: &[HudFeedbackView], art: &ArtAssets) {
    for feedback in feedbacks {
        let position = hud_point(feedback.position);
        let color = hud_color(feedback.color);
        draw_circle_lines(
            position.x,
            position.y,
            feedback.radius,
            if feedback.burst_scale > 1.5 { 3.0 } else { 2.0 },
            color,
        );
        if let Some(texture) = art.effect("gather_feedback_sparkle") {
            draw_texture_centered(
                texture,
                position,
                vec2(feedback.radius * 2.0, feedback.radius * 2.0),
                color,
            );
        }
        draw_circle_lines(
            position.x,
            position.y,
            feedback.radius * 0.62,
            1.5,
            Color::new(color.r, color.g, color.b, color.a * 0.75),
        );
        for sparkle in feedback.sparkle_points {
            let sparkle = hud_point(sparkle);
            draw_circle(
                sparkle.x,
                sparkle.y,
                if feedback.burst_scale > 1.4 { 2.6 } else { 2.0 },
                color,
            );
        }
    }
}

fn hud_color(color: [f32; 4]) -> Color {
    Color::new(color[0], color[1], color[2], color[3])
}

fn hud_point(point: [f32; 2]) -> Vec2 {
    vec2(point[0], point[1])
}
