use super::*;
use crate::art::{draw_texture_centered, toast_icon_for_text, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::ui::{draw_wrapped_text, truncate_text_to_width};

struct HudLine {
    icon_id: Option<String>,
    title: String,
    detail: String,
}

struct HudQuestView {
    title: String,
    requirements: String,
    location_hint: Option<String>,
}

struct HudToastView {
    text: String,
    color: Color,
    alpha: f32,
    icon_key: &'static str,
}

struct HudFeedbackView {
    position: Vec2,
    radius: f32,
    color: Color,
    sparkle_points: [Vec2; 8],
    burst_scale: f32,
}

struct HudView {
    area_name: String,
    inventory_title: String,
    coins_text: String,
    vitality_text: String,
    time_text: String,
    time_color: Color,
    sleep_warning_text: Option<String>,
    progress_text: String,
    day_text: String,
    goal_text: String,
    status_text: String,
    controls_text: String,
    quest: Option<HudQuestView>,
    inventory: Vec<HudLine>,
    effects: Vec<HudLine>,
    potions: Vec<HudLine>,
    toasts: Vec<HudToastView>,
    feedbacks: Vec<HudFeedbackView>,
}

impl GameplayState {
    pub(super) fn draw_hud(&self, area: &AreaDefinition, data: &GameData, art: &ArtAssets) {
        let view = self.build_hud_view(area, data);
        draw_hud_view(&view, art);
    }

    fn build_hud_view(&self, area: &AreaDefinition, data: &GameData) -> HudView {
        let quest = self.active_quest_title(data).map(|title| HudQuestView {
            title: ui_format("hud_quest", &[("title", title)]),
            requirements: self
                .progression
                .started_quests
                .iter()
                .find_map(|quest_id| data.quest(quest_id))
                .map(|quest| {
                    ui_format(
                        "hud_quest_requirements",
                        &[("requirements", &self.quest_requirement_summary(data, quest))],
                    )
                })
                .unwrap_or_default(),
            location_hint: self
                .active_quest_location_hint(data)
                .map(|hint| ui_format("hud_turn_in", &[("hint", hint.as_str())])),
        });

        let inventory = {
            let inventory_items = self.sorted_inventory_items(data);
            if inventory_items.is_empty() {
                vec![HudLine {
                    icon_id: None,
                    title: ui_copy("hud_inventory_empty").to_owned(),
                    detail: String::new(),
                }]
            } else {
                inventory_items
                    .into_iter()
                    .map(|item_id| {
                        let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                        HudLine {
                            icon_id: Some(item_id.clone()),
                            title: format!("{} x{}", data.item_name(&item_id), amount),
                            detail: String::new(),
                        }
                    })
                    .take_while({
                        let mut y = 58.0;
                        move |_| {
                            let keep = y <= 214.0;
                            y += 44.0;
                            keep
                        }
                    })
                    .collect()
            }
        };

        let effects = if self.runtime.active_effects.is_empty() {
            vec![HudLine {
                icon_id: None,
                title: ui_copy("hud_effects_empty").to_owned(),
                detail: String::new(),
            }]
        } else {
            self.runtime
                .active_effects
                .iter()
                .map(|effect| HudLine {
                    icon_id: None,
                    title: format!(
                        "{} {:.0}s",
                        effect_name(effect.kind),
                        effect.remaining_seconds.ceil()
                    ),
                    detail: effect.description.clone(),
                })
                .collect()
        };

        let potions = {
            let quick = self.quick_potions(data);
            if quick.is_empty() {
                vec![HudLine {
                    icon_id: None,
                    title: ui_copy("hud_potions_empty").to_owned(),
                    detail: String::new(),
                }]
            } else {
                quick.iter()
                    .take(3)
                    .enumerate()
                    .map(|(index, item_id)| {
                        let amount = self.inventory.get(item_id).copied().unwrap_or_default();
                        let detail = data
                            .item(item_id)
                            .map(|item| {
                                item.effects
                                    .iter()
                                    .map(|effect| effect.description.as_str())
                                    .collect::<Vec<_>>()
                                    .join(" / ")
                            })
                            .unwrap_or_default();
                        HudLine {
                            icon_id: Some(item_id.clone()),
                            title: format!(
                                "{}: {} x{}",
                                ["Z", "X", "C"][index],
                                data.item_name(item_id),
                                amount
                            ),
                            detail,
                        }
                    })
                    .collect()
            }
        };

        HudView {
            area_name: area.name.clone(),
            inventory_title: format!(
                "{} [{}]",
                ui_copy("hud_inventory_title"),
                self.inventory_sort_label()
            ),
            coins_text: ui_format("hud_coins", &[("coins", &self.coins.to_string())]),
            vitality_text: ui_format("hud_vitality", &[("vitality", &format!("{:.0}", self.vitality))]),
            time_text: ui_format(
                "hud_time",
                &[("time", &clock_text(self.world.day_clock_seconds, data.config.day_length_seconds))],
            ),
            time_color: if self.current_clock_minutes() < 60.0 {
                Color::from_rgba(255, 214, 132, 255)
            } else {
                dark::TEXT
            },
            sleep_warning_text: if self.current_clock_minutes() < 60.0 {
                Some(ui_copy("hud_sleep_warning").to_owned())
            } else {
                None
            },
            progress_text: ui_format("hud_progress", &[("brews", &self.progression.total_brews.to_string())]),
            day_text: ui_format(
                "hud_day",
                &[
                    ("season", self.current_season()),
                    ("weather", self.current_weather()),
                    ("day", &(self.world.day_index + 1).to_string()),
                ],
            ),
            goal_text: ui_format("hud_goal", &[("goal", &self.next_goal_summary(data))]),
            status_text: self.runtime.status_text.clone(),
            controls_text: ui_format(
                "hud_controls_hint",
                &[
                    ("interact", &input_bindings().global.interact),
                    ("journal", &input_bindings().global.journal),
                    ("cancel", &input_bindings().global.cancel),
                ],
            ),
            quest,
            inventory,
            effects,
            potions,
            toasts: self.build_hud_toasts(),
            feedbacks: self.build_hud_feedbacks(area),
        }
    }

    fn build_hud_toasts(&self) -> Vec<HudToastView> {
        self.runtime
            .gather_toasts
            .iter()
            .take(3)
            .rev()
            .map(|toast| HudToastView {
                text: toast.text.clone(),
                color: toast.color,
                alpha: (toast.remaining_seconds / 2.2).clamp(0.0, 1.0),
                icon_key: toast_icon_for_text(&toast.text),
            })
            .collect()
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
                let color = Color::new(
                    feedback.color.r,
                    feedback.color.g,
                    feedback.color.b,
                    alpha,
                );
                let screen_pos = offset + feedback.position;
                let sparkle_points = std::array::from_fn(|index| {
                    let angle = t * 1.1 + index as f32 * (std::f32::consts::TAU / 8.0);
                    let sparkle = vec2(angle.cos(), angle.sin()) * (radius + 4.0 + index as f32 * 1.6);
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
    draw_status_panel(view);
    draw_inventory_panel(view, art);
    draw_effects_panel(view);
    draw_potion_panel(view, art);
    draw_hud_toasts(&view.toasts, art);
    draw_hud_feedbacks(&view.feedbacks, art);
}

fn draw_status_panel(view: &HudView) {
    draw_panel(18.0, 18.0, 560.0, 276.0, ui_copy("hud_status_title"));
    draw_text(&view.area_name, 34.0, 58.0, 32.0, dark::TEXT_BRIGHT);
    draw_text(
        &view.coins_text,
        392.0,
        58.0,
        28.0,
        Color::from_rgba(255, 214, 132, 255),
    );
    draw_text(
        &view.vitality_text,
        34.0,
        86.0,
        24.0,
        Color::from_rgba(126, 220, 158, 255),
    );
    draw_text(&view.time_text, 34.0, 112.0, 24.0, view.time_color);
    draw_text(&view.progress_text, 34.0, 136.0, 20.0, dark::TEXT_DIM);
    draw_text(&view.day_text, 220.0, 112.0, 20.0, dark::TEXT_DIM);
    if let Some(text) = &view.sleep_warning_text {
        draw_wrapped_text(
            text,
            292.0,
            132.0,
            250.0,
            16.0,
            16.0,
            Color::from_rgba(255, 214, 132, 255),
        );
    }
    draw_wrapped_text(&view.goal_text, 34.0, 158.0, 236.0, 18.0, 18.0, dark::TEXT_DIM);
    draw_wrapped_text(&view.status_text, 34.0, 194.0, 236.0, 20.0, 20.0, dark::TEXT);
    draw_wrapped_text(&view.controls_text, 34.0, 250.0, 236.0, 18.0, 18.0, dark::TEXT_DIM);
    if let Some(quest) = &view.quest {
        draw_wrapped_text(
            &quest.title,
            292.0,
            172.0,
            250.0,
            20.0,
            20.0,
            Color::from_rgba(255, 230, 170, 255),
        );
        draw_wrapped_text(&quest.requirements, 292.0, 212.0, 250.0, 18.0, 18.0, dark::TEXT_DIM);
        if let Some(location_hint) = &quest.location_hint {
            draw_wrapped_text(location_hint, 292.0, 248.0, 250.0, 18.0, 18.0, dark::TEXT_DIM);
        }
    }
}

fn draw_inventory_panel(view: &HudView, art: &ArtAssets) {
    draw_panel(
        screen_width() - 320.0,
        18.0,
        302.0,
        222.0,
        &view.inventory_title,
    );
    let mut y = 58.0;
    for line in &view.inventory {
        if let Some(icon_id) = &line.icon_id {
            if let Some(texture) = art.item_icon(icon_id) {
                draw_texture_centered(texture, vec2(screen_width() - 284.0, y + 6.0), vec2(26.0, 26.0), WHITE);
            }
        }
        draw_text(
            &truncate_text_to_width(&line.title, 214.0, 18.0),
            screen_width() - 266.0,
            y,
            18.0,
            dark::TEXT,
        );
        y += 18.0;
        if !line.detail.is_empty() {
            draw_text(
                &truncate_text_to_width(&line.detail, 214.0, 16.0),
                screen_width() - 266.0,
                y,
                16.0,
                dark::TEXT_DIM,
            );
        }
        y += 26.0;
    }
}

fn draw_effects_panel(view: &HudView) {
    draw_panel(
        screen_width() - 320.0,
        252.0,
        302.0,
        154.0,
        ui_copy("hud_effects_title"),
    );
    let mut y = 292.0;
    for line in &view.effects {
        draw_text(
            &truncate_text_to_width(&line.title, 270.0, 20.0),
            screen_width() - 302.0,
            y,
            20.0,
            dark::TEXT_BRIGHT,
        );
        y += 22.0;
        if !line.detail.is_empty() {
            draw_wrapped_text(&line.detail, screen_width() - 302.0, y, 270.0, 16.0, 16.0, dark::TEXT_DIM);
            y += 24.0;
        }
    }
}

fn draw_potion_panel(view: &HudView, art: &ArtAssets) {
    draw_panel(
        18.0,
        screen_height() - 166.0,
        560.0,
        148.0,
        ui_copy("hud_potion_belt_title"),
    );
    let mut y = screen_height() - 126.0;
    for line in &view.potions {
        if let Some(icon_id) = &line.icon_id {
            if let Some(texture) = art.item_icon(icon_id) {
                draw_texture_centered(texture, vec2(54.0, y + 2.0), vec2(30.0, 30.0), WHITE);
            }
        }
        draw_text(
            &truncate_text_to_width(&line.title, 272.0, 20.0),
            76.0,
            y,
            20.0,
            dark::TEXT_BRIGHT,
        );
        y += 22.0;
        if !line.detail.is_empty() {
            draw_wrapped_text(&line.detail, 76.0, y, 272.0, 16.0, 16.0, dark::TEXT_DIM);
            y += 26.0;
        }
    }
    draw_wrapped_text(ui_copy("hud_journal_hint"), 370.0, screen_height() - 44.0, 180.0, 18.0, 18.0, dark::TEXT_DIM);
}

fn draw_hud_toasts(toasts: &[HudToastView], art: &ArtAssets) {
    let start_x = screen_width() * 0.5 - 200.0;
    let mut y = 28.0;
    for toast in toasts {
        let bg = Color::new(18.0 / 255.0, 18.0 / 255.0, 24.0 / 255.0, toast.alpha * 0.9);
        let border = Color::new(toast.color.r, toast.color.g, toast.color.b, toast.alpha);
        let text = Color::new(toast.color.r, toast.color.g, toast.color.b, toast.alpha);
        draw_rectangle(start_x, y, 400.0, 36.0, bg);
        draw_rectangle_lines(start_x, y, 400.0, 36.0, 2.0, border);
        if let Some(texture) = art.toast_icon(toast.icon_key) {
            draw_texture_centered(texture, vec2(start_x + 18.0, y + 18.0), vec2(20.0, 20.0), text);
        }
        draw_text(
            &truncate_text_to_width(&toast.text, 350.0, 18.0),
            start_x + 34.0,
            y + 24.0,
            18.0,
            text,
        );
        y += 42.0;
    }
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
                Color::new(feedback.color.r, feedback.color.g, feedback.color.b, feedback.color.a),
            );
        }
        draw_circle_lines(
            feedback.position.x,
            feedback.position.y,
            feedback.radius * 0.62,
            1.5,
            Color::new(feedback.color.r, feedback.color.g, feedback.color.b, feedback.color.a * 0.75),
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
