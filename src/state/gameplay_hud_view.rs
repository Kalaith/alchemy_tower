use crate::view_models::hud::{HudFeedbackView, HudPotionSlot, HudView};
use super::GameplayState;
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::{AreaDefinition, GameData};
use macroquad::prelude::*;

impl GameplayState {
    pub(super) fn build_hud_view(&self, area: &AreaDefinition, data: &GameData) -> HudView {
        let quick = self.quick_potions(data);
        let potions = std::array::from_fn(|index| {
            if let Some(item_id) = quick.get(index) {
                HudPotionSlot {
                    key_label: quick_potion_key_label(index),
                    icon_id: Some(item_id.clone()),
                    amount: self.inventory.get(item_id).copied().unwrap_or_default(),
                }
            } else {
                HudPotionSlot {
                    key_label: quick_potion_key_label(index),
                    icon_id: None,
                    amount: 0,
                }
            }
        });

        HudView {
            vitality_value: format!("{:.0}", self.vitality),
            coins_value: self.coins.to_string(),
            clock_text: clock_text_12h(
                self.world.day_clock_seconds,
                data.config.day_length_seconds,
            ),
            season_weather_text: format!(
                "{} / {}",
                title_case_label(self.current_season()),
                title_case_label(self.current_weather())
            ),
            day_text: ui_format(
                "hud_day_count",
                &[("day", &(self.world.day_index + 1).to_string())],
            ),
            sleep_warning_text: if self.current_clock_minutes() < 60.0 {
                Some(ui_copy("hud_sleep_warning").to_owned())
            } else {
                None
            },
            goal_prefix: ui_copy("hud_current_goal").to_owned(),
            goal: self.hud_goal(data),
            status_text: self.runtime.status_text.clone(),
            area_label: area.name.clone(),
            potions,
            inventory_count: self.inventory.values().copied().sum(),
            effect_count: self.runtime.active_effects.len(),
            feedbacks: self.build_hud_feedbacks(area),
        }
    }

    fn build_hud_feedbacks(&self, area: &AreaDefinition) -> Vec<HudFeedbackView> {
        let offset = self.camera_offset(area);
        self.runtime
            .gather_feedbacks
            .iter()
            .map(|feedback| {
                let life = feedback.remaining_seconds;
                let t = 1.0
                    - if feedback.emphasis {
                        life / 0.8
                    } else {
                        life / 0.45
                    };
                let radius = if feedback.emphasis {
                    (12.0 + t * 24.0) * feedback.burst_scale
                } else {
                    (10.0 + t * 16.0) * feedback.burst_scale
                };
                let alpha = (1.0 - t).clamp(0.0, 1.0);
                let color = Color::new(feedback.color.r, feedback.color.g, feedback.color.b, alpha);
                let screen_pos = offset + feedback.position;
                let sparkle_points = std::array::from_fn(|index| {
                    let angle = t * 1.1 + index as f32 * (std::f32::consts::TAU / 8.0);
                    let sparkle =
                        vec2(angle.cos(), angle.sin()) * (radius + 4.0 + index as f32 * 1.6);
                    screen_pos + sparkle
                });

                HudFeedbackView {
                    position: screen_pos,
                    radius,
                    color,
                    sparkle_points,
                    burst_scale: feedback.burst_scale,
                }
            })
            .collect()
    }
}

fn clock_text_12h(day_clock_seconds: f32, full_day_seconds: f32) -> String {
    let total_minutes = ((day_clock_seconds / full_day_seconds) * 24.0 * 60.0) as i32;
    let hour_24 = (total_minutes / 60).rem_euclid(24);
    let minute = total_minutes.rem_euclid(60);
    let period = if hour_24 < 12 {
        ui_copy("hud_time_period_am")
    } else {
        ui_copy("hud_time_period_pm")
    };
    let hour_12 = match hour_24 % 12 {
        0 => 12,
        hour => hour,
    };
    format!("{hour_12:02}:{minute:02} {period}")
}

fn title_case_label(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}

fn quick_potion_key_label(index: usize) -> &'static str {
    input_bindings()
        .global
        .quick_potions
        .get(index)
        .map(String::as_str)
        .unwrap_or("")
}
