use super::*;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{ui_copy, ui_format};
use crate::ui::{draw_wrapped_text, truncate_text_to_width};

struct HudLine {
    icon_id: Option<String>,
    title: String,
    detail: String,
}

struct HudPotionSlot {
    key_label: &'static str,
    icon_id: Option<String>,
    amount: u32,
    item_name: String,
}

struct HudDrawerView {
    expanded: bool,
    buttons: [HudDockButton; 3],
    sort_text: String,
    inventory: Vec<HudLine>,
    effects: Vec<HudLine>,
}

struct HudDockButton {
    title: String,
    value: String,
    color: Color,
}

struct HudFeedbackView {
    position: Vec2,
    radius: f32,
    color: Color,
    sparkle_points: [Vec2; 8],
    burst_scale: f32,
}

struct HudView {
    vitality_text: String,
    coins_text: String,
    time_text: String,
    time_color: Color,
    day_text: String,
    sleep_warning_text: Option<String>,
    goal_prefix: String,
    goal_text: String,
    status_text: String,
    area_banner_label: String,
    area_banner_alpha: f32,
    potion_empty_text: String,
    potions: [HudPotionSlot; 3],
    drawer: HudDrawerView,
    feedbacks: Vec<HudFeedbackView>,
}

impl GameplayState {
    pub(super) fn draw_hud(&self, area: &AreaDefinition, data: &GameData, art: &ArtAssets) {
        let view = self.build_hud_view(area, data);
        draw_hud_view(&view, art);
    }

    fn build_hud_view(&self, area: &AreaDefinition, data: &GameData) -> HudView {
        let inventory_preview = self
            .sorted_inventory_items(data)
            .into_iter()
            .take(5)
            .map(|item_id| {
                let amount = self.inventory.get(&item_id).copied().unwrap_or_default();
                HudLine {
                    icon_id: Some(item_id.clone()),
                    title: format!("{} x{}", data.item_name(&item_id), amount),
                    detail: String::new(),
                }
            })
            .collect();

        let effects_preview = self
            .runtime
            .active_effects
            .iter()
            .take(4)
            .map(|effect| HudLine {
                icon_id: None,
                title: format!(
                    "{} {:.0}s",
                    effect_name(effect.kind),
                    effect.remaining_seconds.ceil()
                ),
                detail: effect.description.clone(),
            })
            .collect();

        let quick = self.quick_potions(data);
        let potions = std::array::from_fn(|index| {
            if let Some(item_id) = quick.get(index) {
                HudPotionSlot {
                    key_label: ["Z", "X", "C"][index],
                    icon_id: Some(item_id.clone()),
                    amount: self.inventory.get(item_id).copied().unwrap_or_default(),
                    item_name: data.item_name(item_id).to_owned(),
                }
            } else {
                HudPotionSlot {
                    key_label: ["Z", "X", "C"][index],
                    icon_id: None,
                    amount: 0,
                    item_name: ui_copy("hud_potion_empty_slot").to_owned(),
                }
            }
        });

        let inventory_count: u32 = self.inventory.values().copied().sum();
        let (mouse_x, _) = mouse_position();
        let drawer_hovered = mouse_x >= screen_width() - 124.0;
        let goal_text = self
            .active_quest_location_hint(data)
            .map(|hint| hint.to_owned())
            .or_else(|| self.active_quest_title(data).map(ToOwned::to_owned))
            .unwrap_or_else(|| self.next_goal_summary(data));

        HudView {
            vitality_text: ui_format(
                "hud_vitality",
                &[("vitality", &format!("{:.0}", self.vitality))],
            ),
            coins_text: ui_format("hud_coins", &[("coins", &self.coins.to_string())]),
            time_text: ui_format(
                "hud_time",
                &[(
                    "time",
                    &clock_text(self.world.day_clock_seconds, data.config.day_length_seconds),
                )],
            ),
            time_color: if self.current_clock_minutes() < 60.0 {
                Color::from_rgba(255, 214, 132, 255)
            } else {
                dark::TEXT_BRIGHT
            },
            day_text: ui_format(
                "hud_day",
                &[
                    ("season", self.current_season()),
                    ("weather", self.current_weather()),
                    ("day", &(self.world.day_index + 1).to_string()),
                ],
            ),
            sleep_warning_text: if self.current_clock_minutes() < 60.0 {
                Some(ui_copy("hud_sleep_warning").to_owned())
            } else {
                None
            },
            goal_prefix: ui_copy("hud_goal_prefix").to_owned(),
            goal_text,
            status_text: self.runtime.status_text.clone(),
            area_banner_label: self.runtime.area_banner_label.clone(),
            area_banner_alpha: area_banner_alpha(self.runtime.area_banner_seconds),
            potion_empty_text: ui_copy("hud_potion_empty_slot").to_owned(),
            potions,
            drawer: HudDrawerView {
                expanded: drawer_hovered,
                buttons: [
                    HudDockButton {
                        title: ui_copy("hud_drawer_inventory").to_owned(),
                        value: inventory_count.to_string(),
                        color: Color::from_rgba(255, 230, 170, 255),
                    },
                    HudDockButton {
                        title: ui_copy("hud_drawer_effects").to_owned(),
                        value: self.runtime.active_effects.len().to_string(),
                        color: Color::from_rgba(188, 255, 220, 255),
                    },
                    HudDockButton {
                        title: ui_copy("hud_drawer_journal").to_owned(),
                        value: "J".to_owned(),
                        color: Color::from_rgba(176, 226, 255, 255),
                    },
                ],
                sort_text: ui_format("hud_drawer_sort", &[("mode", self.inventory_sort_label())]),
                inventory: inventory_preview,
                effects: effects_preview,
            },
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

fn draw_hud_view(view: &HudView, art: &ArtAssets) {
    draw_status_cards(view);
    draw_goal_tracker(view);
    draw_area_banner(view);
    draw_right_drawer(view, art);
    draw_potion_belt(view, art);
    draw_status_strip(view);
    draw_hud_feedbacks(&view.feedbacks, art);
}

fn draw_status_cards(view: &HudView) {
    let left = Rect::new(18.0, 18.0, 220.0, 70.0);
    let right = Rect::new(screen_width() - 278.0, 18.0, 260.0, 70.0);
    draw_glass_card(left, Color::from_rgba(126, 220, 158, 66), 176);
    draw_glass_card(right, Color::from_rgba(176, 226, 255, 66), 176);

    draw_text(
        &view.vitality_text,
        left.x + 16.0,
        left.y + 28.0,
        24.0,
        Color::from_rgba(126, 220, 158, 255),
    );
    draw_text(
        &view.coins_text,
        left.x + 16.0,
        left.y + 54.0,
        22.0,
        Color::from_rgba(255, 214, 132, 255),
    );

    draw_text(
        &view.time_text,
        right.x + 16.0,
        right.y + 28.0,
        24.0,
        view.time_color,
    );
    draw_text(
        &truncate_text_to_width(&view.day_text, right.w - 32.0, 18.0),
        right.x + 16.0,
        right.y + 54.0,
        18.0,
        dark::TEXT_DIM,
    );

    if let Some(text) = &view.sleep_warning_text {
        let warning = Rect::new(right.x, right.y + right.h + 8.0, right.w, 38.0);
        draw_glass_card(warning, Color::from_rgba(255, 214, 132, 100), 186);
        draw_wrapped_text(
            text,
            warning.x + 12.0,
            warning.y + 15.0,
            warning.w - 24.0,
            14.0,
            14.0,
            Color::from_rgba(255, 224, 168, 255),
        );
    }
}

fn draw_goal_tracker(view: &HudView) {
    let rect = Rect::new(18.0, 102.0, 220.0, 54.0);
    draw_glass_card(rect, Color::from_rgba(255, 230, 170, 62), 150);
    draw_text(
        &view.goal_prefix,
        rect.x + 14.0,
        rect.y + 22.0,
        15.0,
        dark::TEXT_DIM,
    );
    draw_text(
        &truncate_text_to_width(&view.goal_text, rect.w - 28.0, 18.0),
        rect.x + 14.0,
        rect.y + 42.0,
        18.0,
        Color::from_rgba(255, 238, 196, 255),
    );
}

fn draw_area_banner(view: &HudView) {
    if view.area_banner_alpha <= 0.0 || view.area_banner_label.is_empty() {
        return;
    }

    let width = 360.0;
    let height = 54.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = 22.0 + (1.0 - view.area_banner_alpha) * 12.0;
    let fill = (170.0 * view.area_banner_alpha) as u8;
    let border = Color::new(
        176.0 / 255.0,
        226.0 / 255.0,
        255.0 / 255.0,
        view.area_banner_alpha,
    );
    let text = Color::new(1.0, 0.97, 0.9, view.area_banner_alpha);

    draw_rectangle(x, y, width, height, Color::from_rgba(18, 20, 28, fill));
    draw_rectangle_lines(x, y, width, height, 2.0, border);
    draw_text(
        &truncate_text_to_width(&view.area_banner_label, width - 32.0, 28.0),
        x + 16.0,
        y + 35.0,
        28.0,
        text,
    );
}

fn draw_right_drawer(view: &HudView, art: &ArtAssets) {
    let collapsed_width = 96.0;
    let expanded_width = 300.0;
    let width = if view.drawer.expanded {
        expanded_width
    } else {
        collapsed_width
    };
    let x = screen_width() - width - 18.0;
    let y = 102.0;
    let height = if view.drawer.expanded { 274.0 } else { 180.0 };
    let rect = Rect::new(x, y, width, height);
    draw_glass_card(rect, Color::from_rgba(186, 210, 220, 56), 156);

    if !view.drawer.expanded {
        draw_drawer_stub(rect, &view.drawer);
        return;
    }

    let mut cursor_y = rect.y + 22.0;
    draw_drawer_header(rect, cursor_y, &view.drawer);
    cursor_y += 34.0;

    draw_drawer_section(
        rect.x + 14.0,
        cursor_y,
        rect.w - 28.0,
        &view.drawer.buttons[0].title,
        &view.drawer.inventory,
        ui_copy("hud_drawer_inventory_empty"),
        art,
    );
    cursor_y += 106.0;

    draw_drawer_section(
        rect.x + 14.0,
        cursor_y,
        rect.w - 28.0,
        &view.drawer.buttons[1].title,
        &view.drawer.effects,
        ui_copy("hud_drawer_effects_empty"),
        art,
    );

    draw_text(
        &truncate_text_to_width(&view.drawer.sort_text, rect.w - 28.0, 16.0),
        rect.x + 14.0,
        rect.y + rect.h - 18.0,
        16.0,
        dark::TEXT_DIM,
    );
}

fn draw_drawer_stub(rect: Rect, drawer: &HudDrawerView) {
    for (index, button) in drawer.buttons.iter().enumerate() {
        let top = rect.y + 12.0 + index as f32 * 54.0;
        draw_rectangle(
            rect.x + 8.0,
            top,
            rect.w - 16.0,
            46.0,
            Color::from_rgba(20, 22, 30, 170),
        );
        draw_text(
            &truncate_text_to_width(&button.title, rect.w - 20.0, 14.0),
            rect.x + 16.0,
            top + 18.0,
            14.0,
            dark::TEXT_DIM,
        );
        draw_text(&button.value, rect.x + 16.0, top + 36.0, 18.0, button.color);
    }
}

fn draw_drawer_header(rect: Rect, y: f32, drawer: &HudDrawerView) {
    let inventory_summary = format!("{} {}", drawer.buttons[0].title, drawer.buttons[0].value);
    let effects_summary = format!("{} {}", drawer.buttons[1].title, drawer.buttons[1].value);
    draw_text(
        &truncate_text_to_width(&inventory_summary, 122.0, 18.0),
        rect.x + 14.0,
        y,
        18.0,
        drawer.buttons[0].color,
    );
    draw_text(
        &truncate_text_to_width(&effects_summary, 104.0, 18.0),
        rect.x + 132.0,
        y,
        18.0,
        drawer.buttons[1].color,
    );
    draw_text(
        &truncate_text_to_width(&drawer.buttons[2].title, 70.0, 18.0),
        rect.x + rect.w - 82.0,
        y,
        18.0,
        drawer.buttons[2].color,
    );
}

fn draw_drawer_section(
    x: f32,
    y: f32,
    width: f32,
    title: &str,
    lines: &[HudLine],
    empty_text: &str,
    art: &ArtAssets,
) {
    draw_text(title, x, y, 16.0, dark::TEXT_DIM);
    draw_rectangle(x, y + 8.0, width, 82.0, Color::from_rgba(18, 20, 28, 148));

    if lines.is_empty() {
        draw_wrapped_text(
            empty_text,
            x + 10.0,
            y + 30.0,
            width - 20.0,
            16.0,
            16.0,
            dark::TEXT_DIM,
        );
        return;
    }

    let mut cursor_y = y + 28.0;
    for line in lines.iter().take(3) {
        if let Some(icon_id) = &line.icon_id {
            if let Some(texture) = art.item_icon(icon_id) {
                draw_texture_centered(
                    texture,
                    vec2(x + 13.0, cursor_y - 6.0),
                    vec2(18.0, 18.0),
                    WHITE,
                );
            }
        }
        let text_x = if line.icon_id.is_some() {
            x + 28.0
        } else {
            x + 10.0
        };
        draw_text(
            &truncate_text_to_width(&line.title, width - (text_x - x) - 10.0, 16.0),
            text_x,
            cursor_y,
            16.0,
            dark::TEXT_BRIGHT,
        );
        if !line.detail.is_empty() {
            draw_text(
                &truncate_text_to_width(&line.detail, width - (text_x - x) - 10.0, 14.0),
                text_x,
                cursor_y + 14.0,
                14.0,
                dark::TEXT_DIM,
            );
            cursor_y += 24.0;
        } else {
            cursor_y += 20.0;
        }
    }
}

fn draw_potion_belt(view: &HudView, art: &ArtAssets) {
    let slot_size = 74.0;
    let gap = 14.0;
    let width = slot_size * 3.0 + gap * 2.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 112.0;

    for (index, slot) in view.potions.iter().enumerate() {
        let slot_x = x + index as f32 * (slot_size + gap);
        let rect = Rect::new(slot_x, y, slot_size, slot_size);
        let border = if slot.icon_id.is_some() {
            Color::from_rgba(176, 226, 255, 112)
        } else {
            Color::from_rgba(114, 122, 140, 92)
        };
        draw_glass_card(rect, border, 176);
        draw_text(
            slot.key_label,
            rect.x + 10.0,
            rect.y + 18.0,
            16.0,
            dark::TEXT_DIM,
        );

        if let Some(icon_id) = &slot.icon_id {
            if let Some(texture) = art.item_icon(icon_id) {
                draw_texture_centered(
                    texture,
                    vec2(rect.x + rect.w * 0.5, rect.y + 35.0),
                    vec2(34.0, 34.0),
                    WHITE,
                );
            }
            draw_text(
                &truncate_text_to_width(&slot.item_name, rect.w - 16.0, 14.0),
                rect.x + 8.0,
                rect.y + 58.0,
                14.0,
                dark::TEXT_BRIGHT,
            );
            draw_text(
                &format!("x{}", slot.amount),
                rect.x + rect.w - 24.0,
                rect.y + 18.0,
                16.0,
                Color::from_rgba(255, 214, 132, 255),
            );
        } else {
            draw_text(
                &view.potion_empty_text,
                rect.x + 8.0,
                rect.y + 42.0,
                16.0,
                dark::TEXT_DIM,
            );
        }
    }
}

fn draw_status_strip(view: &HudView) {
    if view.status_text.is_empty() {
        return;
    }

    let width = (screen_width() - 240.0).min(560.0);
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 152.0;
    let rect = Rect::new(x, y, width, 34.0);
    draw_glass_card(rect, Color::from_rgba(160, 170, 190, 48), 132);
    draw_text(
        &truncate_text_to_width(&view.status_text, width - 24.0, 18.0),
        x + 12.0,
        y + 22.0,
        18.0,
        dark::TEXT_DIM,
    );
}

fn draw_glass_card(rect: Rect, border: Color, alpha: u8) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(16, 18, 26, alpha),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.5, border);
}

fn area_banner_alpha(seconds: f32) -> f32 {
    if seconds <= 0.0 {
        return 0.0;
    }
    if seconds > 2.0 {
        ((2.6 - seconds) / 0.6).clamp(0.0, 1.0)
    } else {
        (seconds / 0.6).clamp(0.0, 1.0)
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
