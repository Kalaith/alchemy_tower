use super::GameplayState;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::data::{AreaDefinition, GameData, QuestDefinition};
use crate::ui::{draw_wrapped_text, truncate_text_to_width};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

#[path = "hud_atmosphere.rs"]
mod hud_atmosphere;
#[path = "hud_banner.rs"]
mod hud_banner;
#[path = "hud_belt.rs"]
mod hud_belt;
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
#[path = "hud_header.rs"]
mod hud_header;
#[path = "hud_primitives.rs"]
mod hud_primitives;
#[path = "hud_side.rs"]
mod hud_side;
#[path = "hud_text.rs"]
mod hud_text;

use self::hud_belt::*;
use self::hud_header::*;
use self::hud_side::*;
use self::hud_text::*;

const HOTBAR_SLOT_COUNT: usize = 8;

struct HudPotionSlot {
    key_label: &'static str,
    icon_id: Option<String>,
    amount: u32,
}

struct HudGoal {
    title: String,
    body: String,
    detail: String,
    action: String,
    icon_id: Option<String>,
    amount_text: String,
}

struct HudFeedbackView {
    position: Vec2,
    radius: f32,
    color: Color,
    sparkle_points: [Vec2; 8],
    burst_scale: f32,
}

struct HudView {
    vitality_value: String,
    coins_value: String,
    clock_text: String,
    season_weather_text: String,
    day_text: String,
    sleep_warning_text: Option<String>,
    goal_prefix: String,
    goal: HudGoal,
    status_text: String,
    area_label: String,
    potions: [HudPotionSlot; 3],
    inventory_count: u32,
    effect_count: usize,
    feedbacks: Vec<HudFeedbackView>,
}

impl GameplayState {
    pub(super) fn draw_hud(&self, area: &AreaDefinition, data: &GameData, art: &ArtAssets) {
        let view = self.build_hud_view(area, data);
        draw_hud_view(&view, art);
    }

    fn build_hud_view(&self, area: &AreaDefinition, data: &GameData) -> HudView {
        let quick = self.quick_potions(data);
        let potions = std::array::from_fn(|index| {
            if let Some(item_id) = quick.get(index) {
                HudPotionSlot {
                    key_label: ["Z", "X", "C"][index],
                    icon_id: Some(item_id.clone()),
                    amount: self.inventory.get(item_id).copied().unwrap_or_default(),
                }
            } else {
                HudPotionSlot {
                    key_label: ["Z", "X", "C"][index],
                    icon_id: None,
                    amount: 0,
                }
            }
        });

        let goal = self.hud_goal(data);

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
            goal,
            status_text: self.runtime.status_text.clone(),
            area_label: area.name.clone(),
            potions,
            inventory_count: self.inventory.values().copied().sum(),
            effect_count: self.runtime.active_effects.len(),
            feedbacks: self.build_hud_feedbacks(area),
        }
    }

    fn hud_goal(&self, data: &GameData) -> HudGoal {
        if let Some(goal) = self.active_hud_quest_goal(data) {
            return goal;
        }

        if self.can_reconstruct_archive() && !self.has_journal_milestone("archive_revelation") {
            return HudGoal {
                title: ui_copy("hud_goal_archive_title").to_owned(),
                body: ui_copy("goal_reconstruct_archive").to_owned(),
                detail: String::new(),
                action: String::new(),
                icon_id: None,
                amount_text: String::new(),
            };
        }

        if let Some(quest) = data
            .quests
            .iter()
            .filter(|quest| !self.progression.started_quests.contains(&quest.id))
            .filter(|quest| !self.progression.completed_quests.contains(&quest.id))
            .find(|quest| self.quest_is_available(quest))
        {
            let location = self.quest_hud_location(data, quest);
            return HudGoal {
                title: quest.title.clone(),
                body: quest.description.clone(),
                detail: ui_format("hud_goal_accept_from", &[("location", &location)]),
                action: ui_format(
                    "hud_goal_item_need",
                    &[
                        ("item", data.item_name(&quest.required_item_id)),
                        ("amount", &quest.required_amount.to_string()),
                    ],
                ),
                icon_id: Some(quest.required_item_id.clone()),
                amount_text: ui_format(
                    "hud_goal_item_amount",
                    &[("amount", &quest.required_amount.to_string())],
                ),
            };
        }

        if let Some(warp) = self.next_locked_warp(data) {
            return HudGoal {
                title: ui_copy("hud_goal_restore_title").to_owned(),
                body: warp.label.clone(),
                detail: ui_format(
                    "hud_goal_need",
                    &[("requirements", &self.warp_requirement_summary(data, warp))],
                ),
                action: String::new(),
                icon_id: None,
                amount_text: String::new(),
            };
        }

        HudGoal {
            title: ui_copy("hud_goal_work_title").to_owned(),
            body: ui_copy("goal_keep_working").to_owned(),
            detail: String::new(),
            action: String::new(),
            icon_id: None,
            amount_text: String::new(),
        }
    }

    fn active_hud_quest_goal(&self, data: &GameData) -> Option<HudGoal> {
        let quest = self
            .progression
            .started_quests
            .iter()
            .find_map(|quest_id| data.quest(quest_id))?;
        let location = self.quest_hud_location(data, quest);
        let requirements = self.quest_requirement_summary(data, quest);
        let action = if self.quest_requirements_met(data, quest) {
            ui_format("hud_goal_ready_to_deliver", &[("location", &location)])
        } else {
            ui_format(
                "hud_goal_item_need_with_requirements",
                &[
                    ("item", data.item_name(&quest.required_item_id)),
                    ("amount", &quest.required_amount.to_string()),
                    ("requirements", &requirements),
                ],
            )
        };

        Some(HudGoal {
            title: quest.title.clone(),
            body: quest.description.clone(),
            detail: ui_format("hud_goal_find", &[("location", &location)]),
            action,
            icon_id: Some(quest.required_item_id.clone()),
            amount_text: ui_format(
                "hud_goal_item_amount",
                &[("amount", &quest.required_amount.to_string())],
            ),
        })
    }

    fn quest_hud_location(&self, data: &GameData, quest: &QuestDefinition) -> String {
        let Some(npc) = data.npc(&quest.giver_npc_id) else {
            return ui_copy("hud_goal_request_board").to_owned();
        };
        let runtime = self.npc_runtime_state(data, npc);
        let area_name = data
            .area(&runtime.area_id)
            .map(|area| area.name.as_str())
            .unwrap_or(ui_copy("npc_hint_somewhere"));
        ui_format(
            "hud_goal_meet_npc",
            &[("npc", &npc.name), ("area", area_name)],
        )
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

fn draw_hud_view(view: &HudView, art: &ArtAssets) {
    draw_hud_vignette();
    draw_title_banner(view);
    draw_vitality_medallion(view);
    draw_coin_chip(view);
    draw_goal_note(view, art);
    draw_time_panel(view);
    draw_minimap_frame();
    draw_side_status_panel(view);
    draw_control_tags();
    draw_potion_belt(view, art);
    draw_status_strip(view);
    draw_hud_feedbacks(&view.feedbacks, art);
}

fn draw_hud_feedbacks(feedbacks: &[HudFeedbackView], art: &ArtAssets) {
    for feedback in feedbacks {
        draw_circle_lines(
            feedback.position.x,
            feedback.position.y,
            feedback.radius,
            if feedback.burst_scale > 1.5 { 3.0 } else { 2.0 },
            feedback.color,
        );
        if let Some(texture) = art.effect("gather_feedback_sparkle") {
            draw_texture_centered(
                texture,
                feedback.position,
                vec2(feedback.radius * 2.0, feedback.radius * 2.0),
                Color::new(
                    feedback.color.r,
                    feedback.color.g,
                    feedback.color.b,
                    feedback.color.a,
                ),
            );
        }
        draw_circle_lines(
            feedback.position.x,
            feedback.position.y,
            feedback.radius * 0.62,
            1.5,
            Color::new(
                feedback.color.r,
                feedback.color.g,
                feedback.color.b,
                feedback.color.a * 0.75,
            ),
        );
        for sparkle in feedback.sparkle_points {
            draw_circle(
                sparkle.x,
                sparkle.y,
                if feedback.burst_scale > 1.4 { 2.6 } else { 2.0 },
                feedback.color,
            );
        }
    }
}
