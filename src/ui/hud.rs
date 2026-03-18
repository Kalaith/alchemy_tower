use super::*;
use crate::content::{ui_copy, ui_format};

struct HudLine {
    title: String,
    detail: String,
}

struct HudQuestView {
    title: String,
    location_hint: Option<String>,
}

struct HudToastView {
    text: String,
    color: Color,
    alpha: f32,
}

struct HudFeedbackView {
    position: Vec2,
    radius: f32,
    color: Color,
    sparkle_points: [Vec2; 4],
}

struct HudView {
    area_name: String,
    coins_text: String,
    vitality_text: String,
    time_text: String,
    progress_text: String,
    day_text: String,
    status_text: String,
    quest: Option<HudQuestView>,
    inventory: Vec<HudLine>,
    effects: Vec<HudLine>,
    potions: Vec<HudLine>,
    toasts: Vec<HudToastView>,
    feedbacks: Vec<HudFeedbackView>,
}

impl GameplayState {
    pub(super) fn draw_hud(&self, area: &AreaDefinition, data: &GameData) {
        let view = self.build_hud_view(area, data);
        draw_hud_view(&view);
    }

    fn build_hud_view(&self, area: &AreaDefinition, data: &GameData) -> HudView {
        let quest = self.active_quest_title(data).map(|title| HudQuestView {
            title: ui_format("hud_quest", &[("title", title)]),
            location_hint: self
                .active_quest_location_hint(data)
                .map(|hint| ui_format("hud_turn_in", &[("hint", hint.as_str())])),
        });

        let inventory = {
            let inventory_items = self.sorted_inventory_items(data);
            if inventory_items.is_empty() {
                vec![HudLine {
                    title: ui_copy("hud_inventory_empty").to_owned(),
                    detail: String::new(),
                }]
            } else {
                inventory_items
                    .into_iter()
                    .map(|item_id| {
                        let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                        let context = self.inventory_reference_summary(data, &item_id);
                        HudLine {
                            title: format!("{} x{}", data.item_name(&item_id), amount),
                            detail: if context.is_empty() {
                                ui_copy("hud_stocked").to_owned()
                            } else {
                                context
                            },
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
                title: ui_copy("hud_effects_empty").to_owned(),
                detail: String::new(),
            }]
        } else {
            self.runtime
                .active_effects
                .iter()
                .map(|effect| HudLine {
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
            coins_text: ui_format("hud_coins", &[("coins", &self.coins.to_string())]),
            vitality_text: ui_format("hud_vitality", &[("vitality", &format!("{:.0}", self.vitality))]),
            time_text: ui_format(
                "hud_time",
                &[("time", &clock_text(self.world.day_clock_seconds, data.config.day_length_seconds))],
            ),
            progress_text: ui_format("hud_progress", &[("brews", &self.progression.total_brews.min(10).to_string())]),
            day_text: ui_format(
                "hud_day",
                &[
                    ("season", self.current_season()),
                    ("weather", self.current_weather()),
                    ("day", &(self.world.day_index + 1).to_string()),
                ],
            ),
            status_text: self.runtime.status_text.clone(),
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
                    12.0 + t * 24.0
                } else {
                    10.0 + t * 16.0
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
                    let angle = t * 0.8 + index as f32 * std::f32::consts::FRAC_PI_2;
                    let sparkle = vec2(angle.cos(), angle.sin()) * (radius + 4.0);
                    screen_pos + sparkle
                });

                HudFeedbackView {
                    position: screen_pos,
                    radius,
                    color,
                    sparkle_points,
                }
            })
            .collect()
    }
}

fn draw_hud_view(view: &HudView) {
    draw_status_panel(view);
    draw_inventory_panel(view);
    draw_effects_panel(view);
    draw_potion_panel(view);
    draw_hud_toasts(&view.toasts);
    draw_hud_feedbacks(&view.feedbacks);
}

fn draw_status_panel(view: &HudView) {
    draw_panel(18.0, 18.0, 430.0, 176.0, ui_copy("hud_status_title"));
    draw_text(&view.area_name, 34.0, 58.0, 32.0, dark::TEXT_BRIGHT);
    draw_text(
        &view.coins_text,
        276.0,
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
    draw_text(&view.time_text, 34.0, 112.0, 24.0, dark::TEXT);
    draw_text(&view.progress_text, 34.0, 138.0, 22.0, dark::TEXT_DIM);
    draw_text(&view.day_text, 220.0, 112.0, 20.0, dark::TEXT_DIM);
    draw_text(&view.status_text, 34.0, 138.0, 20.0, dark::TEXT_DIM);
    if let Some(quest) = &view.quest {
        draw_text(
            &quest.title,
            34.0,
            160.0,
            20.0,
            Color::from_rgba(255, 230, 170, 255),
        );
        if let Some(location_hint) = &quest.location_hint {
            draw_text(location_hint, 34.0, 180.0, 18.0, dark::TEXT_DIM);
        }
    }
}

fn draw_inventory_panel(view: &HudView) {
    draw_panel(
        screen_width() - 320.0,
        18.0,
        302.0,
        222.0,
        ui_copy("hud_inventory_title"),
    );
    let mut y = 58.0;
    for line in &view.inventory {
        draw_text(&line.title, screen_width() - 302.0, y, 20.0, dark::TEXT);
        y += 18.0;
        if !line.detail.is_empty() {
            draw_text(&line.detail, screen_width() - 302.0, y, 16.0, dark::TEXT_DIM);
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
        draw_text(&line.title, screen_width() - 302.0, y, 22.0, dark::TEXT_BRIGHT);
        y += 22.0;
        if !line.detail.is_empty() {
            draw_text(&line.detail, screen_width() - 302.0, y, 18.0, dark::TEXT_DIM);
            y += 24.0;
        }
    }
}

fn draw_potion_panel(view: &HudView) {
    draw_panel(
        18.0,
        screen_height() - 166.0,
        560.0,
        148.0,
        ui_copy("hud_potion_belt_title"),
    );
    let mut y = screen_height() - 126.0;
    for line in &view.potions {
        draw_text(&line.title, 34.0, y, 22.0, dark::TEXT_BRIGHT);
        y += 22.0;
        if !line.detail.is_empty() {
            draw_text(&line.detail, 34.0, y, 18.0, dark::TEXT_DIM);
            y += 26.0;
        }
    }
    draw_text(
        ui_copy("hud_journal_hint"),
        418.0,
        screen_height() - 26.0,
        18.0,
        dark::TEXT_DIM,
    );
}

fn draw_hud_toasts(toasts: &[HudToastView]) {
    let start_x = screen_width() * 0.5 - 200.0;
    let mut y = 28.0;
    for toast in toasts {
        let bg = Color::new(18.0 / 255.0, 18.0 / 255.0, 24.0 / 255.0, toast.alpha * 0.9);
        let border = Color::new(toast.color.r, toast.color.g, toast.color.b, toast.alpha);
        let text = Color::new(toast.color.r, toast.color.g, toast.color.b, toast.alpha);
        draw_rectangle(start_x, y, 400.0, 28.0, bg);
        draw_rectangle_lines(start_x, y, 400.0, 28.0, 2.0, border);
        draw_text(&toast.text, start_x + 10.0, y + 19.0, 20.0, text);
        y += 34.0;
    }
}

fn draw_hud_feedbacks(feedbacks: &[HudFeedbackView]) {
    for feedback in feedbacks {
        draw_circle_lines(
            feedback.position.x,
            feedback.position.y,
            feedback.radius,
            2.0,
            feedback.color,
        );
        for sparkle in feedback.sparkle_points {
            draw_circle(sparkle.x, sparkle.y, 2.0, feedback.color);
        }
    }
}
