use super::*;
use crate::art::{draw_texture_centered, ArtAssets};
use crate::content::{input_bindings, ui_copy, ui_format};
use crate::ui::{draw_wrapped_text, truncate_text_to_width};

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

fn draw_hud_vignette() {
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

fn draw_title_banner(view: &HudView) {
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

fn draw_vitality_medallion(view: &HudView) {
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

fn draw_coin_chip(view: &HudView) {
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

fn draw_goal_note(view: &HudView, art: &ArtAssets) {
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

fn draw_time_panel(view: &HudView) {
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

fn draw_minimap_frame() {
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

fn draw_side_status_panel(view: &HudView) {
    let rect = Rect::new(screen_width() - 104.0, 214.0, 84.0, 238.0);
    draw_vertical_plaque_backplate(rect);
    draw_ornate_panel(rect, Color::from_rgba(27, 25, 20, 218), 0.9);
    draw_panel_filigree(rect, 0.58);
    draw_side_status_hardware(rect);
    draw_centered_text(
        ui_copy("hud_drawer_inventory"),
        rect.x,
        rect.y + 25.0,
        rect.w,
        15.0,
        brass_light(),
    );
    draw_status_icon_medallion(
        vec2(rect.x + 30.0, rect.y + 53.0),
        Color::from_rgba(222, 174, 112, 84),
    );
    draw_bag_icon(vec2(rect.x + 29.0, rect.y + 52.0), 0.82);
    draw_text(
        &view.inventory_count.to_string(),
        rect.x + 48.0,
        rect.y + 58.0,
        18.0,
        bright_ink(),
    );
    draw_side_status_divider(rect, rect.y + 78.0);
    draw_centered_text(
        ui_copy("hud_drawer_effects"),
        rect.x,
        rect.y + 103.0,
        rect.w,
        15.0,
        brass_light(),
    );
    if view.effect_count > 0 {
        draw_status_icon_medallion(
            vec2(rect.x + 31.0, rect.y + 128.0),
            Color::from_rgba(85, 222, 207, 82),
        );
        draw_spark_icon(vec2(rect.x + 31.0, rect.y + 128.0), 0.9);
        draw_centered_text_shadowed(
            &view.effect_count.to_string(),
            rect.x,
            rect.y + 127.0,
            rect.w,
            20.0,
            bright_ink(),
        );
    } else {
        draw_centered_text(
            ui_copy("overlay_none"),
            rect.x,
            rect.y + 129.0,
            rect.w,
            17.0,
            bright_ink(),
        );
    }
    draw_side_status_divider(rect, rect.y + 156.0);
    draw_centered_text(
        ui_copy("hud_drawer_journal"),
        rect.x,
        rect.y + 188.0,
        rect.w,
        16.0,
        brass_light(),
    );
    draw_status_icon_medallion(
        vec2(rect.x + 30.0, rect.y + 213.0),
        Color::from_rgba(84, 124, 110, 88),
    );
    draw_book_icon(vec2(rect.x + 21.0, rect.y + 212.0), 0.8);
    draw_keycap(
        Rect::new(rect.x + 28.0, rect.y + 197.0, 30.0, 30.0),
        "J",
        true,
    );
}

fn draw_side_status_hardware(rect: Rect) {
    let rail = Color::from_rgba(238, 196, 119, 116);
    let dark = Color::from_rgba(0, 0, 0, 82);
    for x in [rect.x - 5.0, rect.x + rect.w + 5.0] {
        draw_line(
            x + 1.0,
            rect.y + 25.0,
            x + 1.0,
            rect.y + rect.h - 24.0,
            2.0,
            dark,
        );
        draw_line(x, rect.y + 25.0, x, rect.y + rect.h - 24.0, 1.1, rail);
        draw_circle_lines(x, rect.y + 22.0, 5.0, 1.0, rail);
        draw_circle_lines(x, rect.y + rect.h - 21.0, 5.0, 1.0, rail);
    }

    for point in [
        vec2(rect.x + rect.w * 0.5, rect.y + 4.0),
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h - 4.0),
    ] {
        draw_poly(
            point.x + 1.0,
            point.y + 2.0,
            4,
            7.0,
            45.0,
            Color::from_rgba(0, 0, 0, 72),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            7.0,
            45.0,
            Color::from_rgba(242, 205, 126, 174),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            4.0,
            45.0,
            Color::from_rgba(52, 130, 124, 148),
        );
    }

    for point in [
        vec2(rect.x + 13.0, rect.y + 13.0),
        vec2(rect.x + rect.w - 13.0, rect.y + 13.0),
        vec2(rect.x + rect.w - 13.0, rect.y + rect.h - 13.0),
        vec2(rect.x + 13.0, rect.y + rect.h - 13.0),
    ] {
        draw_circle(point.x, point.y, 2.2, Color::from_rgba(242, 205, 126, 146));
        draw_circle(
            point.x - 0.7,
            point.y - 0.7,
            0.8,
            Color::from_rgba(255, 238, 182, 160),
        );
    }
}

fn draw_status_icon_medallion(center: Vec2, tint: Color) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        16.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_circle(center.x, center.y, 15.0, Color::from_rgba(91, 62, 36, 166));
    draw_circle(center.x, center.y, 11.0, tint);
    draw_circle_lines(
        center.x,
        center.y,
        15.0,
        1.1,
        Color::from_rgba(242, 205, 126, 146),
    );
    draw_circle_lines(
        center.x,
        center.y,
        9.0,
        0.8,
        Color::from_rgba(255, 238, 181, 72),
    );
}

fn draw_side_status_divider(rect: Rect, y: f32) {
    let color = Color::from_rgba(221, 174, 91, 92);
    let center = rect.x + rect.w * 0.5;
    draw_line(rect.x + 13.0, y, center - 7.0, y, 1.0, color);
    draw_line(center + 7.0, y, rect.x + rect.w - 13.0, y, 1.0, color);
    draw_small_diamond(vec2(center, y), Color::from_rgba(242, 205, 126, 128));
    draw_circle_lines(center - 18.0, y, 3.0, 0.8, color);
    draw_circle_lines(center + 18.0, y, 3.0, 0.8, color);
}

fn draw_control_tags() {
    let x = 22.0;
    let y = screen_height() - 184.0;
    let rows = [
        (
            input_bindings().alchemy.open.as_str(),
            ui_copy("hud_control_alchemy"),
        ),
        (
            input_bindings().global.journal.as_str(),
            ui_copy("hud_drawer_journal"),
        ),
        ("V", ui_copy("hud_control_sort")),
        (
            input_bindings().global.cancel.as_str(),
            ui_copy("hud_control_pause"),
        ),
    ];
    for (index, (key, label)) in rows.iter().enumerate() {
        draw_control_tag(
            Rect::new(x, y + index as f32 * 40.0, 158.0, 32.0),
            key,
            label,
        );
    }
}

fn draw_control_tag(rect: Rect, key: &str, label: &str) {
    draw_tag_panel(rect);
    draw_keycap(
        Rect::new(rect.x + 12.0, rect.y + 5.0, 40.0, 22.0),
        key,
        false,
    );
    draw_text(
        label,
        rect.x + 64.0,
        rect.y + 22.0,
        19.0,
        Color::from_rgba(44, 34, 26, 255),
    );
    draw_small_diamond(vec2(rect.x + rect.w - 9.0, rect.y + rect.h * 0.5), brass());
}

fn draw_potion_belt(view: &HudView, art: &ArtAssets) {
    let slot_size = 58.0;
    let gap = 12.0;
    let width = 40.0 + slot_size * HOTBAR_SLOT_COUNT as f32 + gap * 7.0 + 40.0;
    let height = 96.0;
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 108.0;
    let rect = Rect::new(x, y, width, height);
    draw_belt_backplate(rect);
    draw_ornate_panel(rect, Color::from_rgba(43, 35, 28, 224), 0.92);
    draw_flourish_line(
        rect.x - 26.0,
        rect.y + rect.h * 0.5,
        rect.x + 20.0,
        rect.y + rect.h * 0.5,
        true,
    );
    draw_flourish_line(
        rect.x + rect.w - 20.0,
        rect.y + rect.h * 0.5,
        rect.x + rect.w + 26.0,
        rect.y + rect.h * 0.5,
        false,
    );
    draw_line(
        rect.x + 28.0,
        rect.y + 13.0,
        rect.x + rect.w - 28.0,
        rect.y + 13.0,
        1.0,
        Color::from_rgba(255, 236, 180, 48),
    );
    draw_line(
        rect.x + 28.0,
        rect.y + rect.h - 18.0,
        rect.x + rect.w - 28.0,
        rect.y + rect.h - 18.0,
        1.0,
        Color::from_rgba(0, 0, 0, 92),
    );
    draw_belt_hardware(rect, slot_size, gap);
    draw_gem(vec2(rect.x + rect.w * 0.5, rect.y + rect.h + 2.0), 9.0);

    for index in 0..HOTBAR_SLOT_COUNT {
        let slot_rect = Rect::new(
            x + 40.0 + index as f32 * (slot_size + gap),
            y + 16.0,
            slot_size,
            slot_size,
        );
        if let Some(slot) = view.potions.get(index) {
            draw_hotbar_slot(slot_rect, slot, art, index);
        } else {
            draw_empty_hotbar_slot(slot_rect, index);
        }
        draw_centered_text_shadowed(
            &(index + 1).to_string(),
            slot_rect.x,
            slot_rect.y + 85.0,
            slot_rect.w,
            20.0,
            bright_ink(),
        );
    }
}

fn draw_hotbar_slot(rect: Rect, slot: &HudPotionSlot, art: &ArtAssets, index: usize) {
    draw_empty_hotbar_slot(rect, index);
    draw_keycap(
        Rect::new(rect.x + 5.0, rect.y + 5.0, 20.0, 18.0),
        slot.key_label,
        true,
    );
    if let Some(icon_id) = &slot.icon_id {
        draw_glass_potion_bottle(rect, slot_glow(index), 0.28);
        if let Some(texture) = art.item_icon(icon_id) {
            draw_texture_centered(
                texture,
                vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.55),
                vec2(46.0, 46.0),
                WHITE,
            );
        }
        draw_slot_sparkle(rect, slot_glow(index), 0.42);
        draw_text(
            &slot.amount.to_string(),
            rect.x + rect.w - 17.0,
            rect.y + rect.h - 9.0,
            18.0,
            bright_ink(),
        );
    } else {
        draw_glass_potion_bottle(rect, slot_glow(index), 0.34);
    }
}

fn draw_empty_hotbar_slot(rect: Rect, index: usize) {
    let glow = slot_glow(index);
    let bevel = 7.0;
    draw_beveled_rect(
        Rect::new(rect.x + 3.0, rect.y + 5.0, rect.w, rect.h),
        bevel,
        Color::from_rgba(4, 4, 6, 72),
    );
    draw_beveled_rect(rect, bevel, Color::from_rgba(38, 32, 27, 202));
    draw_panel_texture(rect, bevel, Color::from_rgba(38, 32, 27, 202), 0.7);
    draw_beveled_rect_lines(rect, bevel, 1.5, Color::from_rgba(223, 184, 111, 154));
    draw_beveled_rect_lines(
        Rect::new(rect.x + 2.0, rect.y + 2.0, rect.w - 4.0, rect.h - 4.0),
        5.0,
        1.0,
        Color::new(glow.r, glow.g, glow.b, 0.25),
    );
    draw_beveled_rect_lines(
        Rect::new(rect.x + 5.0, rect.y + 5.0, rect.w - 10.0, rect.h - 10.0),
        4.0,
        1.0,
        Color::from_rgba(255, 236, 180, 42),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.52,
        18.0,
        Color::new(glow.r, glow.g, glow.b, 0.08),
    );
    draw_line(
        rect.x + 12.0,
        rect.y + 12.0,
        rect.x + rect.w - 12.0,
        rect.y + 8.0,
        1.0,
        Color::new(glow.r, glow.g, glow.b, 0.2),
    );
    draw_slot_corner_dots(rect, glow);
    draw_glass_potion_bottle(rect, glow, 0.22);
}

fn draw_slot_corner_dots(rect: Rect, glow: Color) {
    let color = Color::new(glow.r, glow.g, glow.b, 0.34);
    for point in [
        vec2(rect.x + 7.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 7.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 7.0, rect.y + rect.h - 7.0),
        vec2(rect.x + 7.0, rect.y + rect.h - 7.0),
    ] {
        draw_circle(point.x, point.y, 1.5, color);
    }
}

fn draw_bottle_silhouette(rect: Rect, alpha: f32) {
    let color = Color::new(parchment().r, parchment().g, parchment().b, alpha);
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 17.0;
    draw_rectangle(cx - 5.0, top, 10.0, 13.0, color);
    draw_rectangle(cx - 12.0, top + 13.0, 24.0, 23.0, color);
    draw_circle(cx, top + 37.0, 12.0, color);
    draw_rectangle_lines(
        cx - 12.0,
        top + 13.0,
        24.0,
        24.0,
        1.0,
        Color::new(
            brass_light().r,
            brass_light().g,
            brass_light().b,
            alpha * 0.8,
        ),
    );
}

fn draw_glass_potion_bottle(rect: Rect, liquid: Color, alpha: f32) {
    let cx = rect.x + rect.w * 0.5;
    let top = rect.y + 16.0;
    let glass = Color::new(parchment().r, parchment().g, parchment().b, alpha * 0.9);
    let outline = Color::new(
        brass_light().r,
        brass_light().g,
        brass_light().b,
        alpha * 0.8,
    );
    let liquid_color = Color::new(liquid.r, liquid.g, liquid.b, alpha);

    draw_rectangle(cx - 5.0, top, 10.0, 13.0, glass);
    draw_rectangle(
        cx - 8.0,
        top - 3.0,
        16.0,
        5.0,
        Color::new(0.44, 0.33, 0.24, alpha),
    );
    draw_rectangle(cx - 13.0, top + 15.0, 26.0, 21.0, glass);
    draw_circle(cx, top + 38.0, 14.0, glass);
    draw_rectangle(cx - 10.0, top + 29.0, 20.0, 11.0, liquid_color);
    draw_circle(cx, top + 38.0, 10.5, liquid_color);
    draw_circle(
        cx - 5.0,
        top + 26.0,
        3.0,
        Color::new(1.0, 1.0, 1.0, alpha * 0.72),
    );
    draw_line(cx - 9.0, top + 17.0, cx - 12.0, top + 35.0, 1.2, outline);
    draw_line(cx + 9.0, top + 17.0, cx + 12.0, top + 35.0, 1.2, outline);
    draw_circle_lines(cx, top + 38.0, 14.0, 1.2, outline);
}

fn draw_slot_sparkle(rect: Rect, color: Color, alpha: f32) {
    let center = vec2(rect.x + rect.w - 12.0, rect.y + 15.0);
    let tint = Color::new(color.r, color.g, color.b, alpha);
    draw_line(
        center.x - 5.0,
        center.y,
        center.x + 5.0,
        center.y,
        1.0,
        tint,
    );
    draw_line(
        center.x,
        center.y - 5.0,
        center.x,
        center.y + 5.0,
        1.0,
        tint,
    );
    draw_circle(center.x, center.y, 1.5, bright_ink());
}

fn draw_status_strip(view: &HudView) {
    if view.status_text.is_empty() {
        return;
    }

    let width = (screen_width() - 320.0).min(560.0);
    let x = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - 148.0;
    let rect = Rect::new(x, y, width, 34.0);
    draw_ornate_panel(rect, Color::from_rgba(17, 17, 19, 168), 0.66);
    draw_text(
        &truncate_text_to_width(&view.status_text, width - 24.0, 17.0),
        x + 12.0,
        y + 22.0,
        17.0,
        muted_ink(),
    );
}

fn draw_goal_item_badge(rect: Rect, icon_id: &str, amount_text: &str, art: &ArtAssets) {
    let badge = Rect::new(rect.x + rect.w - 76.0, rect.y + 100.0, 48.0, 58.0);
    draw_beveled_rect(
        Rect::new(badge.x + 3.0, badge.y + 4.0, badge.w, badge.h),
        7.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_beveled_rect(badge, 7.0, Color::from_rgba(42, 35, 29, 204));
    draw_beveled_rect_lines(badge, 7.0, 1.2, Color::from_rgba(223, 184, 111, 150));

    if let Some(texture) = art.item_icon(icon_id) {
        draw_texture_centered(
            texture,
            vec2(badge.x + badge.w * 0.5, badge.y + 25.0),
            vec2(34.0, 34.0),
            WHITE,
        );
    } else {
        draw_bottle_silhouette(Rect::new(badge.x + 7.0, badge.y + 6.0, 34.0, 34.0), 0.44);
    }

    draw_centered_text(
        amount_text,
        badge.x,
        badge.y + badge.h - 8.0,
        badge.w,
        13.0,
        parchment(),
    );
}

fn draw_tag_panel(rect: Rect) {
    let left_tab = [
        vec2(rect.x - 6.0, rect.y + 7.0),
        vec2(rect.x + 8.0, rect.y + rect.h * 0.5),
        vec2(rect.x - 6.0, rect.y + rect.h - 7.0),
    ];
    let right_tab = [
        vec2(rect.x + rect.w + 6.0, rect.y + 7.0),
        vec2(rect.x + rect.w - 8.0, rect.y + rect.h * 0.5),
        vec2(rect.x + rect.w + 6.0, rect.y + rect.h - 7.0),
    ];
    draw_beveled_rect(
        Rect::new(rect.x + 4.0, rect.y + 5.0, rect.w, rect.h),
        7.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_triangle(
        left_tab[0],
        left_tab[1],
        left_tab[2],
        Color::from_rgba(143, 111, 71, 212),
    );
    draw_triangle(
        right_tab[0],
        right_tab[1],
        right_tab[2],
        Color::from_rgba(143, 111, 71, 212),
    );
    draw_beveled_rect(rect, 7.0, Color::from_rgba(181, 156, 112, 228));
    draw_beveled_rect_lines(rect, 7.0, 1.5, Color::from_rgba(235, 201, 137, 220));
    draw_beveled_rect_lines(
        Rect::new(rect.x + 4.0, rect.y + 4.0, rect.w - 8.0, rect.h - 8.0),
        5.0,
        1.0,
        Color::from_rgba(76, 51, 32, 92),
    );
    draw_flourish_line(
        rect.x + rect.w - 31.0,
        rect.y + rect.h * 0.5,
        rect.x + rect.w - 9.0,
        rect.y + rect.h * 0.5,
        false,
    );
    for x in [rect.x + 8.0, rect.x + rect.w - 8.0] {
        draw_circle(
            x,
            rect.y + rect.h * 0.5,
            2.2,
            Color::from_rgba(76, 51, 32, 128),
        );
        draw_circle(
            x - 0.7,
            rect.y + rect.h * 0.5 - 0.7,
            1.0,
            Color::from_rgba(255, 229, 158, 138),
        );
    }
}

fn draw_hud_atmosphere(sw: f32, sh: f32) {
    draw_soft_circle(
        vec2(sw * 0.50, sh * 0.45),
        160.0,
        Color::from_rgba(59, 223, 211, 28),
        8,
    );
    draw_soft_circle(
        vec2(sw * 0.78, sh * 0.31),
        210.0,
        Color::from_rgba(246, 178, 92, 18),
        8,
    );
    draw_soft_circle(
        vec2(sw * 0.27, sh * 0.23),
        170.0,
        Color::from_rgba(238, 171, 84, 14),
        7,
    );
    draw_soft_circle(
        vec2(sw * 0.50, sh - 42.0),
        260.0,
        Color::from_rgba(44, 30, 18, 42),
        8,
    );
}

fn draw_soft_circle(center: Vec2, radius: f32, color: Color, steps: usize) {
    for step in (1..=steps).rev() {
        let t = step as f32 / steps as f32;
        let alpha = color.a * (1.0 - t) * 0.42;
        draw_circle(
            center.x,
            center.y,
            radius * t,
            Color::new(color.r, color.g, color.b, alpha),
        );
    }
}

fn draw_edge_foliage(sw: f32, sh: f32) {
    draw_foliage_silhouette(vec2(46.0, sh - 18.0), false, 1.1);
    draw_foliage_silhouette(vec2(135.0, sh - 6.0), false, 0.72);
    draw_foliage_silhouette(vec2(sw - 52.0, sh - 18.0), true, 1.08);
    draw_foliage_silhouette(vec2(sw - 142.0, sh - 8.0), true, 0.75);
}

fn draw_foliage_silhouette(root: Vec2, mirrored: bool, scale: f32) {
    let sign = if mirrored { -1.0 } else { 1.0 };
    let leaf_dark = Color::from_rgba(19, 50, 29, 150);
    let leaf_mid = Color::from_rgba(42, 84, 45, 118);
    let stem = Color::from_rgba(45, 35, 22, 132);
    draw_line(
        root.x,
        root.y,
        root.x + sign * 54.0 * scale,
        root.y - 56.0 * scale,
        3.0 * scale,
        stem,
    );
    for (index, offset) in [
        vec2(10.0, -8.0),
        vec2(24.0, -22.0),
        vec2(38.0, -34.0),
        vec2(54.0, -50.0),
    ]
    .iter()
    .enumerate()
    {
        let base = root + vec2(sign * offset.x * scale, offset.y * scale);
        let width = (22.0 + index as f32 * 4.0) * scale;
        let height = (12.0 + index as f32 * 2.0) * scale;
        let color = if index % 2 == 0 { leaf_dark } else { leaf_mid };
        draw_triangle(
            base,
            base + vec2(sign * width, -height),
            base + vec2(sign * 7.0 * scale, height * 0.4),
            color,
        );
        draw_triangle(
            base + vec2(sign * 3.0 * scale, -3.0 * scale),
            base + vec2(-sign * width * 0.45, -height * 0.75),
            base + vec2(-sign * 5.0 * scale, height * 0.45),
            color,
        );
    }
}

fn draw_medallion_backplate(center: Vec2, radius: f32) {
    draw_circle(
        center.x + 5.0,
        center.y + 8.0,
        radius + 16.0,
        Color::from_rgba(0, 0, 0, 78),
    );
    draw_circle(
        center.x,
        center.y,
        radius + 12.0,
        Color::from_rgba(94, 70, 36, 156),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius + 12.0,
        2.0,
        Color::from_rgba(242, 205, 126, 178),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius + 6.0,
        1.0,
        Color::from_rgba(45, 30, 18, 124),
    );
    for index in 0..8 {
        let angle = index as f32 * std::f32::consts::TAU / 8.0 + 0.12;
        let base = center + vec2(angle.cos(), angle.sin()) * (radius + 10.0);
        draw_poly(
            base.x,
            base.y,
            4,
            if index % 2 == 0 { 5.0 } else { 3.5 },
            45.0,
            Color::from_rgba(242, 205, 126, 170),
        );
    }
}

fn draw_medallion_ticks(center: Vec2, radius: f32) {
    let tick = Color::from_rgba(242, 205, 126, 132);
    for index in 0..20 {
        let angle = index as f32 * std::f32::consts::TAU / 20.0;
        let inner = center + vec2(angle.cos(), angle.sin()) * (radius - 13.0);
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius - 8.0);
        draw_line(inner.x, inner.y, outer.x, outer.y, 1.0, tick);
    }
    for index in 0..5 {
        let angle = index as f32 * std::f32::consts::TAU / 5.0 - 0.2;
        let point = center + vec2(angle.cos(), angle.sin()) * (radius - 24.0);
        draw_circle(point.x, point.y, 1.5, Color::from_rgba(102, 226, 190, 150));
    }
}

fn draw_coin_chip_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 8.0, rect.y - 7.0, rect.w + 16.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 4.0, back.y + 6.0, back.w, back.h),
        12.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 12.0, Color::from_rgba(119, 80, 39, 136));
    draw_panel_texture(back, 12.0, Color::from_rgba(119, 80, 39, 136), 0.66);
    draw_beveled_rect_lines(back, 12.0, 1.1, Color::from_rgba(235, 196, 118, 130));
}

fn draw_coin_chip_connector(rect: Rect) {
    let y = rect.y + rect.h * 0.5;
    let color = Color::from_rgba(242, 205, 126, 118);
    draw_line(
        rect.x - 34.0,
        y + 2.0,
        rect.x + 9.0,
        y + 2.0,
        3.0,
        Color::from_rgba(0, 0, 0, 62),
    );
    draw_line(rect.x - 34.0, y, rect.x + 9.0, y, 1.5, color);
    draw_circle_lines(rect.x - 18.0, y, 9.0, 1.2, color);
    draw_small_diamond(vec2(rect.x - 2.0, y), Color::from_rgba(91, 223, 205, 130));
    draw_leaf_cluster_scaled(vec2(rect.x - 31.0, y + 14.0), false, 0.34);
}

fn draw_coin_face(center: Vec2) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        15.0,
        Color::from_rgba(0, 0, 0, 70),
    );
    draw_circle(
        center.x,
        center.y,
        14.0,
        Color::from_rgba(208, 151, 50, 255),
    );
    draw_circle(
        center.x - 2.0,
        center.y - 2.0,
        10.0,
        Color::from_rgba(235, 181, 72, 230),
    );
    draw_circle_lines(
        center.x,
        center.y,
        14.0,
        2.0,
        Color::from_rgba(255, 229, 148, 230),
    );
    draw_circle_lines(
        center.x,
        center.y,
        8.5,
        0.9,
        Color::from_rgba(118, 75, 30, 128),
    );
    draw_line(
        center.x - 4.5,
        center.y - 6.0,
        center.x + 4.5,
        center.y - 6.0,
        1.0,
        Color::from_rgba(255, 236, 166, 210),
    );
    draw_line(
        center.x,
        center.y - 5.0,
        center.x,
        center.y + 6.0,
        1.0,
        Color::from_rgba(118, 75, 30, 128),
    );
    draw_circle(
        center.x - 4.0,
        center.y - 5.0,
        3.0,
        Color::from_rgba(255, 236, 166, 210),
    );
}

fn draw_goal_note_hardware(rect: Rect, has_icon: bool) {
    let sheet_width = if has_icon {
        rect.w - 112.0
    } else {
        rect.w - 32.0
    };
    let sheet = Rect::new(rect.x + 15.0, rect.y + 89.0, sheet_width, 75.0);
    draw_beveled_rect(sheet, 7.0, Color::from_rgba(76, 60, 39, 74));
    draw_beveled_rect_lines(sheet, 7.0, 0.9, Color::from_rgba(242, 205, 126, 58));
    draw_panel_texture(sheet, 7.0, Color::from_rgba(76, 60, 39, 74), 0.54);

    let margin_x = rect.x + 18.0;
    draw_line(
        margin_x,
        rect.y + 52.0,
        margin_x,
        rect.y + rect.h - 18.0,
        1.0,
        Color::from_rgba(242, 205, 126, 80),
    );
    for index in 0..6 {
        let y = rect.y + 61.0 + index as f32 * 22.0;
        draw_circle(margin_x, y, 1.8, Color::from_rgba(242, 205, 126, 128));
        draw_circle(margin_x, y, 0.8, Color::from_rgba(48, 33, 21, 154));
    }

    for point in [
        vec2(rect.x + 31.0, rect.y + 52.0),
        vec2(rect.x + rect.w - 31.0, rect.y + 52.0),
        vec2(rect.x + 31.0, rect.y + rect.h - 18.0),
        vec2(rect.x + rect.w - 31.0, rect.y + rect.h - 18.0),
    ] {
        draw_poly(
            point.x + 1.0,
            point.y + 2.0,
            4,
            4.5,
            45.0,
            Color::from_rgba(0, 0, 0, 74),
        );
        draw_poly(
            point.x,
            point.y,
            4,
            4.5,
            45.0,
            Color::from_rgba(242, 205, 126, 142),
        );
    }

    draw_line(
        rect.x + rect.w - 18.0,
        rect.y + 55.0,
        rect.x + rect.w - 18.0,
        rect.y + rect.h - 22.0,
        0.8,
        Color::from_rgba(84, 218, 198, 58),
    );
    draw_leaf_cluster_scaled(
        vec2(rect.x + rect.w - 31.0, rect.y + rect.h - 30.0),
        true,
        0.34,
    );
}

fn draw_goal_action_strip(rect: Rect) {
    let strip = Rect::new(rect.x + 16.0, rect.y + rect.h - 29.0, rect.w - 32.0, 22.0);
    draw_beveled_rect(
        Rect::new(strip.x + 2.0, strip.y + 3.0, strip.w, strip.h),
        6.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_beveled_rect(strip, 6.0, Color::from_rgba(44, 38, 30, 142));
    draw_beveled_rect_lines(strip, 6.0, 0.9, Color::from_rgba(242, 205, 126, 82));
    draw_small_diamond(
        vec2(strip.x + strip.w - 11.0, strip.y + strip.h * 0.5),
        Color::from_rgba(91, 223, 205, 122),
    );
}

fn draw_time_panel_hardware(rect: Rect) {
    let text_band = Rect::new(rect.x + 9.0, rect.y + 8.0, rect.w - 72.0, rect.h - 16.0);
    draw_beveled_rect(text_band, 8.0, Color::from_rgba(10, 11, 13, 58));
    draw_beveled_rect_lines(text_band, 8.0, 0.9, Color::from_rgba(242, 205, 126, 54));
    for y in [rect.y + 35.0, rect.y + 70.0] {
        draw_line(
            text_band.x + 12.0,
            y,
            text_band.x + text_band.w - 12.0,
            y,
            0.9,
            Color::from_rgba(242, 205, 126, 54),
        );
        draw_small_diamond(
            vec2(text_band.x + text_band.w * 0.5, y),
            Color::from_rgba(242, 205, 126, 86),
        );
    }

    let sun_center = vec2(rect.x + rect.w - 34.0, rect.y + 32.0);
    draw_circle(
        sun_center.x + 3.0,
        sun_center.y + 4.0,
        25.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_circle(
        sun_center.x,
        sun_center.y,
        24.0,
        Color::from_rgba(92, 62, 35, 172),
    );
    draw_circle_lines(
        sun_center.x,
        sun_center.y,
        24.0,
        1.1,
        Color::from_rgba(242, 205, 126, 154),
    );
    draw_circle_lines(
        sun_center.x,
        sun_center.y,
        18.0,
        0.8,
        Color::from_rgba(255, 238, 181, 62),
    );
    for index in 0..12 {
        let angle = index as f32 * std::f32::consts::TAU / 12.0;
        let point = sun_center + vec2(angle.cos(), angle.sin()) * 22.0;
        draw_circle(point.x, point.y, 1.1, Color::from_rgba(242, 205, 126, 122));
    }

    for point in [
        vec2(rect.x + 13.0, rect.y + 12.0),
        vec2(rect.x + rect.w - 13.0, rect.y + 12.0),
        vec2(rect.x + 13.0, rect.y + rect.h - 12.0),
        vec2(rect.x + rect.w - 13.0, rect.y + rect.h - 12.0),
    ] {
        draw_circle(point.x, point.y, 2.0, Color::from_rgba(242, 205, 126, 120));
        draw_circle(
            point.x - 0.6,
            point.y - 0.6,
            0.8,
            Color::from_rgba(255, 238, 181, 144),
        );
    }
}

fn draw_journal_note_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 8.0, rect.y - 7.0, rect.w + 16.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        11.0,
        Color::from_rgba(0, 0, 0, 82),
    );
    draw_beveled_rect(back, 11.0, Color::from_rgba(93, 66, 37, 146));
    draw_panel_texture(back, 11.0, Color::from_rgba(93, 66, 37, 146), 0.76);
    draw_beveled_rect_lines(back, 11.0, 1.2, Color::from_rgba(235, 196, 118, 126));
    for point in [
        vec2(back.x + 15.0, back.y + 15.0),
        vec2(back.x + back.w - 15.0, back.y + 15.0),
        vec2(back.x + back.w - 15.0, back.y + back.h - 15.0),
        vec2(back.x + 15.0, back.y + back.h - 15.0),
    ] {
        draw_circle(point.x, point.y, 2.4, Color::from_rgba(244, 204, 128, 158));
        draw_circle(point.x, point.y, 1.0, Color::from_rgba(49, 33, 20, 140));
    }
}

fn draw_small_plaque_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 14.0, rect.y - 6.0, rect.w + 28.0, rect.h + 12.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        12.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 12.0, Color::from_rgba(120, 82, 42, 136));
    draw_panel_texture(back, 12.0, Color::from_rgba(120, 82, 42, 136), 0.68);
    draw_beveled_rect_lines(back, 12.0, 1.1, Color::from_rgba(235, 196, 118, 132));
}

fn draw_vertical_plaque_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 8.0, rect.y - 8.0, rect.w + 16.0, rect.h + 16.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        11.0,
        Color::from_rgba(0, 0, 0, 78),
    );
    draw_beveled_rect(back, 11.0, Color::from_rgba(95, 62, 32, 116));
    draw_panel_texture(back, 11.0, Color::from_rgba(95, 62, 32, 116), 0.62);
    draw_beveled_rect_lines(back, 11.0, 1.0, Color::from_rgba(235, 196, 118, 112));
}

fn draw_compass_backplate(center: Vec2, radius: f32) {
    let brass_shadow = Color::from_rgba(0, 0, 0, 84);
    let warm = Color::from_rgba(112, 76, 41, 138);
    draw_circle(center.x + 5.0, center.y + 8.0, radius + 12.0, brass_shadow);
    draw_circle(center.x, center.y, radius + 9.0, warm);
    draw_circle_lines(
        center.x,
        center.y,
        radius + 9.0,
        1.5,
        Color::from_rgba(235, 196, 118, 138),
    );
    for angle in [
        0.0_f32,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    ] {
        let point = center + vec2(angle.cos(), angle.sin()) * (radius + 5.0);
        draw_small_diamond(point, Color::from_rgba(242, 205, 126, 190));
    }
}

fn draw_compass_ticks(center: Vec2, radius: f32) {
    for index in 0..32 {
        let angle = index as f32 * std::f32::consts::TAU / 32.0;
        let is_cardinal = index % 8 == 0;
        let inner_radius = if is_cardinal {
            radius - 18.0
        } else {
            radius - 12.0
        };
        let inner = center + vec2(angle.cos(), angle.sin()) * inner_radius;
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius - 6.0);
        let alpha = if is_cardinal { 142 } else { 78 };
        draw_line(
            inner.x,
            inner.y,
            outer.x,
            outer.y,
            if is_cardinal { 1.4 } else { 0.8 },
            Color::from_rgba(232, 198, 129, alpha),
        );
    }
}

fn draw_compass_map_texture(center: Vec2, radius: f32) {
    for index in 0..9 {
        let y = center.y - 36.0 + index as f32 * 9.0;
        let width = (radius - 22.0) * (1.0 - ((y - center.y).abs() / radius).min(0.84));
        draw_line(
            center.x - width,
            y,
            center.x + width,
            y + if index % 2 == 0 { 1.5 } else { -1.0 },
            0.7,
            Color::from_rgba(236, 211, 162, 26),
        );
    }
    for index in 0..5 {
        let x = center.x - 28.0 + index as f32 * 14.0;
        draw_line(
            x,
            center.y - 35.0,
            x + 4.0,
            center.y + 36.0,
            0.6,
            Color::from_rgba(52, 34, 23, 34),
        );
    }
}

fn draw_compass_rosette(center: Vec2) {
    let teal = Color::from_rgba(91, 223, 205, 116);
    let brass = Color::from_rgba(242, 205, 126, 166);
    draw_circle(center.x, center.y, 12.0, Color::from_rgba(28, 24, 20, 110));
    for angle in [
        0.0_f32,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    ] {
        let tip = center + vec2(angle.cos(), angle.sin()) * 14.0;
        let left = center + vec2((angle + 2.45).cos(), (angle + 2.45).sin()) * 5.0;
        let right = center + vec2((angle - 2.45).cos(), (angle - 2.45).sin()) * 5.0;
        draw_triangle(tip, left, right, brass);
    }
    draw_circle(center.x, center.y, 4.0, teal);
    draw_circle_lines(
        center.x,
        center.y,
        12.0,
        1.0,
        Color::from_rgba(242, 205, 126, 148),
    );
}

fn draw_ornate_divider(x: f32, y: f32, width: f32, opacity: f32) {
    let color = Color::from_rgba(220, 182, 109, (132.0 * opacity) as u8);
    let center = x + width * 0.5;
    draw_line(x, y, center - 12.0, y, 1.0, color);
    draw_line(center + 12.0, y, x + width, y, 1.0, color);
    draw_small_diamond(
        vec2(center, y),
        Color::from_rgba(242, 205, 126, (176.0 * opacity) as u8),
    );
}

fn draw_panel_filigree(rect: Rect, opacity: f32) {
    let color = Color::from_rgba(242, 205, 126, (112.0 * opacity) as u8);
    let dark = Color::from_rgba(46, 30, 18, (120.0 * opacity) as u8);
    let corner_gap = 14.0;
    for (x, y, sx, sy) in [
        (rect.x + corner_gap, rect.y + corner_gap, 1.0, 1.0),
        (rect.x + rect.w - corner_gap, rect.y + corner_gap, -1.0, 1.0),
        (rect.x + corner_gap, rect.y + rect.h - corner_gap, 1.0, -1.0),
        (
            rect.x + rect.w - corner_gap,
            rect.y + rect.h - corner_gap,
            -1.0,
            -1.0,
        ),
    ] {
        draw_circle_lines(x + sx * 8.0, y + sy * 8.0, 7.0, 1.0, color);
        draw_line(x, y + sy * 13.0, x + sx * 22.0, y + sy * 13.0, 1.0, color);
        draw_line(x + sx * 13.0, y, x + sx * 13.0, y + sy * 22.0, 1.0, color);
        draw_circle(x + sx * 8.0, y + sy * 8.0, 1.3, dark);
    }

    if rect.h > 80.0 {
        draw_panel_side_knot(vec2(rect.x + 2.0, rect.y + rect.h * 0.5), 1.0, opacity);
        draw_panel_side_knot(
            vec2(rect.x + rect.w - 2.0, rect.y + rect.h * 0.5),
            -1.0,
            opacity,
        );
    }

    if rect.w > 150.0 {
        let top = vec2(rect.x + rect.w * 0.5, rect.y + 2.0);
        let bottom = vec2(rect.x + rect.w * 0.5, rect.y + rect.h - 2.0);
        draw_small_diamond(
            top,
            Color::from_rgba(242, 205, 126, (154.0 * opacity) as u8),
        );
        draw_small_diamond(
            bottom,
            Color::from_rgba(242, 205, 126, (122.0 * opacity) as u8),
        );
    }
}

fn draw_panel_side_knot(center: Vec2, direction: f32, opacity: f32) {
    let color = Color::from_rgba(242, 205, 126, (130.0 * opacity) as u8);
    let glow = Color::from_rgba(87, 214, 199, (72.0 * opacity) as u8);
    draw_poly(center.x, center.y, 4, 5.5, 45.0, color);
    draw_circle_lines(center.x + direction * 10.0, center.y, 7.0, 1.0, color);
    draw_line(
        center.x + direction * 3.0,
        center.y,
        center.x + direction * 22.0,
        center.y,
        1.0,
        color,
    );
    draw_circle(center.x + direction * 10.0, center.y, 2.0, glow);
}

fn draw_banner_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 38.0, rect.y - 7.0, rect.w + 76.0, rect.h + 14.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        15.0,
        Color::from_rgba(0, 0, 0, 74),
    );
    draw_beveled_rect(back, 15.0, Color::from_rgba(139, 101, 58, 172));
    draw_panel_texture(back, 15.0, Color::from_rgba(139, 101, 58, 172), 0.86);
    draw_beveled_rect_lines(back, 15.0, 1.6, Color::from_rgba(244, 204, 128, 196));
    draw_beveled_rect_lines(
        Rect::new(back.x + 8.0, back.y + 8.0, back.w - 16.0, back.h - 16.0),
        10.0,
        1.0,
        Color::from_rgba(61, 40, 24, 114),
    );
    draw_banner_wing(vec2(back.x + 9.0, back.y + back.h * 0.5), -1.0);
    draw_banner_wing(vec2(back.x + back.w - 9.0, back.y + back.h * 0.5), 1.0);
    draw_leaf_cluster_scaled(vec2(back.x + 66.0, back.y + 5.0), false, 0.36);
    draw_leaf_cluster_scaled(vec2(back.x + back.w - 66.0, back.y + 5.0), true, 0.36);
}

fn draw_title_plaque_caps(rect: Rect) {
    for side in [-1.0, 1.0] {
        let center = vec2(
            if side < 0.0 {
                rect.x - 6.0
            } else {
                rect.x + rect.w + 6.0
            },
            rect.y + rect.h * 0.5,
        );
        draw_poly(
            center.x + side * 2.0,
            center.y + 2.0,
            4,
            9.0,
            45.0,
            Color::from_rgba(0, 0, 0, 76),
        );
        draw_poly(
            center.x,
            center.y,
            4,
            9.0,
            45.0,
            Color::from_rgba(176, 124, 62, 194),
        );
        draw_poly(
            center.x,
            center.y,
            4,
            5.0,
            45.0,
            Color::from_rgba(242, 205, 126, 178),
        );
    }
}

fn draw_banner_inner_hardware(rect: Rect) {
    let brass = Color::from_rgba(242, 205, 126, 116);
    let dark = Color::from_rgba(0, 0, 0, 72);
    let y_top = rect.y + 10.0;
    let y_bottom = rect.y + rect.h - 10.0;
    draw_line(
        rect.x + 44.0,
        y_top + 1.0,
        rect.x + rect.w - 44.0,
        y_top + 1.0,
        1.8,
        dark,
    );
    draw_line(
        rect.x + 44.0,
        y_top,
        rect.x + rect.w - 44.0,
        y_top,
        1.0,
        brass,
    );
    draw_line(
        rect.x + 44.0,
        y_bottom,
        rect.x + rect.w - 44.0,
        y_bottom,
        0.9,
        Color::from_rgba(242, 205, 126, 74),
    );

    for side in [-1.0, 1.0] {
        let x = if side < 0.0 {
            rect.x + 30.0
        } else {
            rect.x + rect.w - 30.0
        };
        let center = vec2(x, rect.y + rect.h * 0.5);
        draw_circle(
            center.x + side * 1.0,
            center.y + 2.0,
            12.0,
            Color::from_rgba(0, 0, 0, 70),
        );
        draw_circle(center.x, center.y, 11.0, Color::from_rgba(101, 72, 42, 182));
        draw_circle_lines(
            center.x,
            center.y,
            11.0,
            1.0,
            Color::from_rgba(242, 205, 126, 164),
        );
        draw_poly(
            center.x,
            center.y,
            4,
            5.0,
            45.0,
            Color::from_rgba(80, 219, 205, 140),
        );
        draw_line(
            center.x + side * 12.0,
            center.y,
            center.x + side * 34.0,
            center.y,
            1.0,
            brass,
        );
    }

    for point in [
        vec2(rect.x + 60.0, rect.y + 14.0),
        vec2(rect.x + rect.w - 60.0, rect.y + 14.0),
        vec2(rect.x + 60.0, rect.y + rect.h - 14.0),
        vec2(rect.x + rect.w - 60.0, rect.y + rect.h - 14.0),
    ] {
        draw_circle(point.x, point.y, 2.1, Color::from_rgba(242, 205, 126, 142));
        draw_circle(
            point.x - 0.6,
            point.y - 0.6,
            0.8,
            Color::from_rgba(255, 238, 181, 150),
        );
    }
}

fn draw_gem_mount(center: Vec2) {
    draw_circle(
        center.x + 2.0,
        center.y + 3.0,
        19.0,
        Color::from_rgba(0, 0, 0, 72),
    );
    draw_circle(center.x, center.y, 18.0, Color::from_rgba(95, 67, 37, 178));
    draw_circle_lines(
        center.x,
        center.y,
        18.0,
        1.2,
        Color::from_rgba(242, 205, 126, 166),
    );
    for angle in [
        0.0_f32,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    ] {
        let point = center + vec2(angle.cos(), angle.sin()) * 16.0;
        draw_circle(point.x, point.y, 1.8, Color::from_rgba(242, 205, 126, 148));
    }
}

fn draw_banner_wing(center: Vec2, direction: f32) {
    let outer = Color::from_rgba(167, 119, 61, 168);
    let trim = Color::from_rgba(242, 205, 126, 190);
    draw_triangle(
        center + vec2(direction * 2.0, -27.0),
        center + vec2(direction * 52.0, -11.0),
        center + vec2(direction * 2.0, 0.0),
        outer,
    );
    draw_triangle(
        center + vec2(direction * 2.0, 27.0),
        center + vec2(direction * 52.0, 11.0),
        center + vec2(direction * 2.0, 0.0),
        outer,
    );
    draw_line(
        center.x + direction * 7.0,
        center.y - 21.0,
        center.x + direction * 43.0,
        center.y - 8.0,
        1.3,
        trim,
    );
    draw_line(
        center.x + direction * 7.0,
        center.y + 21.0,
        center.x + direction * 43.0,
        center.y + 8.0,
        1.3,
        trim,
    );
    draw_small_diamond(center + vec2(direction * 18.0, 0.0), trim);
}

fn draw_belt_backplate(rect: Rect) {
    let back = Rect::new(rect.x - 20.0, rect.y - 6.0, rect.w + 40.0, rect.h + 10.0);
    draw_beveled_rect(
        Rect::new(back.x + 5.0, back.y + 7.0, back.w, back.h),
        13.0,
        Color::from_rgba(0, 0, 0, 76),
    );
    draw_beveled_rect(back, 13.0, Color::from_rgba(112, 78, 43, 138));
    draw_beveled_rect_lines(back, 13.0, 1.2, Color::from_rgba(235, 196, 118, 142));
    draw_panel_texture(back, 13.0, Color::from_rgba(112, 78, 43, 138), 0.72);
}

fn draw_belt_hardware(rect: Rect, slot_size: f32, gap: f32) {
    let rail = Rect::new(rect.x + 26.0, rect.y + 10.0, rect.w - 52.0, rect.h - 25.0);
    draw_beveled_rect(rail, 10.0, Color::from_rgba(24, 19, 16, 92));
    draw_beveled_rect_lines(rail, 10.0, 0.9, Color::from_rgba(238, 196, 119, 76));
    draw_beveled_rect_lines(
        Rect::new(rail.x + 5.0, rail.y + 5.0, rail.w - 10.0, rail.h - 10.0),
        7.0,
        0.8,
        Color::from_rgba(255, 238, 181, 38),
    );

    for side in [-1.0, 1.0] {
        let center = vec2(
            if side < 0.0 {
                rect.x + 16.0
            } else {
                rect.x + rect.w - 16.0
            },
            rect.y + rect.h * 0.5,
        );
        draw_circle(
            center.x + side * 2.0,
            center.y + 3.0,
            12.0,
            Color::from_rgba(0, 0, 0, 58),
        );
        draw_poly(
            center.x,
            center.y,
            6,
            12.0,
            30.0,
            Color::from_rgba(116, 78, 41, 164),
        );
        draw_poly_lines(
            center.x,
            center.y,
            6,
            12.0,
            30.0,
            1.1,
            Color::from_rgba(242, 205, 126, 164),
        );
        draw_small_diamond(center, Color::from_rgba(85, 222, 207, 146));
    }

    for index in 1..HOTBAR_SLOT_COUNT {
        let x = rect.x + 40.0 + index as f32 * slot_size + (index as f32 - 0.5) * gap;
        draw_line(
            x,
            rect.y + 20.0,
            x,
            rect.y + rect.h - 35.0,
            0.8,
            Color::from_rgba(242, 205, 126, 46),
        );
        draw_circle(
            x,
            rect.y + rect.h - 23.0,
            2.0,
            Color::from_rgba(242, 205, 126, 96),
        );
    }

    draw_flourish_line(
        rect.x + rect.w * 0.5 - 52.0,
        rect.y + rect.h - 12.0,
        rect.x + rect.w * 0.5 - 10.0,
        rect.y + rect.h - 12.0,
        true,
    );
    draw_flourish_line(
        rect.x + rect.w * 0.5 + 10.0,
        rect.y + rect.h - 12.0,
        rect.x + rect.w * 0.5 + 52.0,
        rect.y + rect.h - 12.0,
        false,
    );
}

fn draw_ornate_panel(rect: Rect, fill: Color, opacity: f32) {
    let bevel = rect.w.min(rect.h).min(18.0) * 0.45;
    draw_beveled_rect(
        Rect::new(rect.x + 6.0, rect.y + 8.0, rect.w, rect.h),
        bevel,
        Color::new(0.0, 0.0, 0.0, 0.28 * opacity),
    );
    draw_beveled_rect(rect, bevel, fill);
    draw_panel_texture(rect, bevel, fill, opacity);
    draw_beveled_rect_lines(
        rect,
        bevel,
        2.0,
        Color::new(brass().r, brass().g, brass().b, opacity),
    );
    draw_beveled_rect_lines(
        Rect::new(rect.x + 4.0, rect.y + 4.0, rect.w - 8.0, rect.h - 8.0),
        (bevel - 3.0).max(4.0),
        1.0,
        Color::from_rgba(255, 239, 184, (58.0 * opacity) as u8),
    );
    draw_corner_marks(rect, opacity);
}

fn draw_panel_texture(rect: Rect, bevel: f32, fill: Color, opacity: f32) {
    let warm_fill = fill.r > fill.b;
    let light = if warm_fill {
        Color::from_rgba(255, 222, 159, (24.0 * opacity) as u8)
    } else {
        Color::from_rgba(255, 232, 176, (18.0 * opacity) as u8)
    };
    let dark = Color::from_rgba(0, 0, 0, (26.0 * opacity) as u8);
    let inset = (bevel + 4.0).min(rect.w * 0.18).min(rect.h * 0.28);
    let rows = ((rect.h / 18.0).ceil() as usize).clamp(1, 8);

    for row in 0..rows {
        let y = rect.y + inset + 5.0 + row as f32 * 14.0;
        if y > rect.y + rect.h - inset {
            break;
        }
        let offset = (row % 3) as f32 * 7.0;
        let x1 = rect.x + inset + offset;
        let x2 = rect.x + rect.w - inset - 5.0 - (row % 2) as f32 * 10.0;
        draw_line(x1, y, x2, y + 0.7, 1.0, light);
        if row % 2 == 0 {
            draw_line(x1 + 8.0, y + 3.0, x2 - 16.0, y + 3.6, 1.0, dark);
        }
    }

    let scuffs = ((rect.w / 78.0).ceil() as usize).clamp(1, 6);
    for scuff in 0..scuffs {
        let x = rect.x + inset + 14.0 + scuff as f32 * 66.0;
        if x > rect.x + rect.w - inset - 16.0 {
            break;
        }
        let y = rect.y + rect.h - inset - 10.0 - (scuff % 2) as f32 * 12.0;
        draw_line(x, y, x + 18.0, y - 4.0, 1.0, dark);
    }
}

fn draw_corner_marks(rect: Rect, opacity: f32) {
    let color = Color::from_rgba(242, 202, 126, (168.0 * opacity) as u8);
    let len = 16.0;
    for (x, y, sx, sy) in [
        (rect.x, rect.y, 1.0, 1.0),
        (rect.x + rect.w, rect.y, -1.0, 1.0),
        (rect.x, rect.y + rect.h, 1.0, -1.0),
        (rect.x + rect.w, rect.y + rect.h, -1.0, -1.0),
    ] {
        draw_line(x, y + sy * len, x + sx * len, y, 1.5, color);
        draw_circle(x + sx * 10.0, y + sy * 10.0, 2.0, color);
    }
}

fn draw_beveled_rect(rect: Rect, bevel: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    draw_rectangle(rect.x + bevel, rect.y, rect.w - bevel * 2.0, rect.h, color);
    draw_rectangle(rect.x, rect.y + bevel, rect.w, rect.h - bevel * 2.0, color);

    let center_tl = vec2(rect.x + bevel, rect.y + bevel);
    let center_tr = vec2(rect.x + rect.w - bevel, rect.y + bevel);
    let center_br = vec2(rect.x + rect.w - bevel, rect.y + rect.h - bevel);
    let center_bl = vec2(rect.x + bevel, rect.y + rect.h - bevel);
    draw_triangle(
        vec2(rect.x + bevel, rect.y),
        vec2(rect.x, rect.y + bevel),
        center_tl,
        color,
    );
    draw_triangle(
        vec2(rect.x + rect.w - bevel, rect.y),
        vec2(rect.x + rect.w, rect.y + bevel),
        center_tr,
        color,
    );
    draw_triangle(
        vec2(rect.x + rect.w, rect.y + rect.h - bevel),
        vec2(rect.x + rect.w - bevel, rect.y + rect.h),
        center_br,
        color,
    );
    draw_triangle(
        vec2(rect.x, rect.y + rect.h - bevel),
        vec2(rect.x + bevel, rect.y + rect.h),
        center_bl,
        color,
    );
}

fn draw_beveled_rect_lines(rect: Rect, bevel: f32, thickness: f32, color: Color) {
    let bevel = bevel.min(rect.w * 0.5).min(rect.h * 0.5);
    let points = [
        vec2(rect.x + bevel, rect.y),
        vec2(rect.x + rect.w - bevel, rect.y),
        vec2(rect.x + rect.w, rect.y + bevel),
        vec2(rect.x + rect.w, rect.y + rect.h - bevel),
        vec2(rect.x + rect.w - bevel, rect.y + rect.h),
        vec2(rect.x + bevel, rect.y + rect.h),
        vec2(rect.x, rect.y + rect.h - bevel),
        vec2(rect.x, rect.y + bevel),
    ];
    for index in 0..points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        draw_line(start.x, start.y, end.x, end.y, thickness, color);
    }
}

fn draw_flourish_line(x1: f32, y1: f32, x2: f32, y2: f32, left: bool) {
    let color = Color::from_rgba(221, 177, 96, 185);
    draw_line(x1, y1, x2, y2, 2.0, color);
    let sign = if left { 1.0 } else { -1.0 };
    let curl_x = if left { x1 + 22.0 } else { x1 - 22.0 };
    draw_circle_lines(curl_x, y1, 10.0, 1.5, color);
    draw_line(x2, y2, x2 + sign * 18.0, y2 - 12.0, 1.5, color);
    draw_line(x2, y2, x2 + sign * 18.0, y2 + 12.0, 1.5, color);
}

fn draw_title_vines(x: f32, y: f32, width: f32) {
    let color = Color::from_rgba(221, 177, 96, 150);
    draw_line(x + 42.0, y + 8.0, x + 118.0, y + 8.0, 1.5, color);
    draw_line(
        x + width - 118.0,
        y + 8.0,
        x + width - 42.0,
        y + 8.0,
        1.5,
        color,
    );
    draw_circle_lines(x + 126.0, y + 8.0, 8.0, 1.2, color);
    draw_circle_lines(x + width - 126.0, y + 8.0, 8.0, 1.2, color);
    draw_leaf_cluster_scaled(vec2(x + 58.0, y + 9.0), false, 0.42);
    draw_leaf_cluster_scaled(vec2(x + width - 58.0, y + 9.0), true, 0.42);
}

fn draw_keycap(rect: Rect, key: &str, blue: bool) {
    let fill = if blue {
        Color::from_rgba(39, 75, 110, 235)
    } else {
        Color::from_rgba(29, 31, 36, 235)
    };
    draw_rectangle(rect.x + 2.0, rect.y + 3.0, rect.w, rect.h, shadow());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        Color::from_rgba(239, 217, 174, 190),
    );
    draw_centered_text(
        key,
        rect.x,
        rect.y + rect.h - 5.0,
        rect.w,
        15.0,
        bright_ink(),
    );
}

fn draw_bag_icon(center: Vec2, scale: f32) {
    let fill = Color::from_rgba(205, 160, 112, 230);
    let dark = Color::from_rgba(80, 54, 34, 190);
    let w = 20.0 * scale;
    let h = 17.0 * scale;
    draw_rectangle(center.x - w * 0.5, center.y - h * 0.15, w, h, fill);
    draw_circle(center.x - w * 0.25, center.y - h * 0.15, w * 0.24, fill);
    draw_circle(center.x + w * 0.25, center.y - h * 0.15, w * 0.24, fill);
    draw_circle_lines(center.x, center.y - h * 0.25, w * 0.28, 1.2, dark);
    draw_line(
        center.x - w * 0.35,
        center.y + h * 0.25,
        center.x + w * 0.35,
        center.y + h * 0.25,
        1.0,
        dark,
    );
}

fn draw_book_icon(center: Vec2, scale: f32) {
    let cover = Color::from_rgba(63, 95, 88, 210);
    let pages = Color::from_rgba(226, 204, 162, 190);
    let w = 20.0 * scale;
    let h = 17.0 * scale;
    draw_rectangle(center.x - w * 0.5, center.y - h * 0.5, w, h, pages);
    draw_rectangle(
        center.x - w * 0.45,
        center.y - h * 0.45,
        w * 0.42,
        h * 0.9,
        cover,
    );
    draw_rectangle(
        center.x + w * 0.03,
        center.y - h * 0.45,
        w * 0.42,
        h * 0.9,
        cover,
    );
    draw_line(
        center.x,
        center.y - h * 0.45,
        center.x,
        center.y + h * 0.45,
        1.0,
        brass_light(),
    );
}

fn draw_spark_icon(center: Vec2, scale: f32) {
    let color = Color::from_rgba(112, 222, 199, 230);
    let radius = 12.0 * scale;
    draw_line(
        center.x - radius,
        center.y,
        center.x + radius,
        center.y,
        1.4,
        color,
    );
    draw_line(
        center.x,
        center.y - radius,
        center.x,
        center.y + radius,
        1.4,
        color,
    );
    draw_line(
        center.x - radius * 0.6,
        center.y - radius * 0.6,
        center.x + radius * 0.6,
        center.y + radius * 0.6,
        1.0,
        color,
    );
    draw_line(
        center.x - radius * 0.6,
        center.y + radius * 0.6,
        center.x + radius * 0.6,
        center.y - radius * 0.6,
        1.0,
        color,
    );
    draw_circle(center.x, center.y, 2.4 * scale, bright_ink());
}

fn draw_gem(center: Vec2, radius: f32) {
    draw_poly(
        center.x,
        center.y,
        4,
        radius,
        45.0,
        Color::from_rgba(73, 213, 220, 240),
    );
    draw_poly(
        center.x,
        center.y,
        4,
        radius - 4.0,
        45.0,
        Color::from_rgba(185, 255, 244, 230),
    );
    draw_circle_lines(center.x, center.y, radius + 3.0, 1.5, brass_light());
}

fn draw_small_diamond(center: Vec2, color: Color) {
    draw_poly(center.x, center.y, 4, 6.0, 45.0, color);
}

fn draw_circle_arc(
    center: Vec2,
    radius: f32,
    start_degrees: f32,
    sweep_degrees: f32,
    thickness: f32,
    color: Color,
) {
    let segments = (sweep_degrees.abs() / 8.0).ceil().max(5.0) as usize;
    let start = start_degrees.to_radians();
    let sweep = sweep_degrees.to_radians();
    let mut previous = center + vec2(start.cos(), start.sin()) * radius;

    for step in 1..=segments {
        let angle = start + sweep * step as f32 / segments as f32;
        let next = center + vec2(angle.cos(), angle.sin()) * radius;
        draw_line(previous.x, previous.y, next.x, next.y, thickness, color);
        previous = next;
    }
}

fn draw_flower(center: Vec2, scale: f32) {
    let petal = Color::from_rgba(244, 233, 189, 238);
    let petal_shadow = Color::from_rgba(180, 138, 104, 128);
    let core = Color::from_rgba(241, 188, 72, 245);
    let radius = 4.5 * scale;
    for index in 0..5 {
        let angle = index as f32 * std::f32::consts::TAU / 5.0 - 0.3;
        let point = center + vec2(angle.cos(), angle.sin()) * (7.0 * scale);
        draw_circle(
            point.x + 0.8 * scale,
            point.y + 1.1 * scale,
            radius,
            petal_shadow,
        );
        draw_circle(point.x, point.y, radius, petal);
    }
    draw_circle(center.x, center.y, 3.2 * scale, core);
}

fn draw_leaf_cluster(center: Vec2, mirrored: bool) {
    draw_leaf_cluster_scaled(center, mirrored, 1.0);
}

fn draw_leaf_cluster_scaled(center: Vec2, mirrored: bool, scale: f32) {
    let sign = if mirrored { -1.0 } else { 1.0 };
    let leaf = Color::from_rgba(91, 142, 76, 230);
    let light = Color::from_rgba(150, 190, 105, 230);
    draw_triangle(
        center + vec2(0.0, -12.0) * scale,
        center + vec2(sign * 20.0, -4.0) * scale,
        center + vec2(sign * 3.0, 4.0) * scale,
        leaf,
    );
    draw_triangle(
        center + vec2(sign * 3.0, 0.0) * scale,
        center + vec2(sign * 26.0, 13.0) * scale,
        center + vec2(sign * 2.0, 16.0) * scale,
        light,
    );
    draw_circle(
        center.x - sign * 8.0 * scale,
        center.y + 4.0 * scale,
        4.0 * scale,
        Color::from_rgba(239, 226, 172, 245),
    );
}

fn draw_sun_icon(center: Vec2, radius: f32) {
    let color = Color::from_rgba(242, 173, 56, 255);
    for index in 0..8 {
        let angle = index as f32 * std::f32::consts::TAU / 8.0;
        let inner = center + vec2(angle.cos(), angle.sin()) * (radius + 4.0);
        let outer = center + vec2(angle.cos(), angle.sin()) * (radius + 12.0);
        draw_line(inner.x, inner.y, outer.x, outer.y, 2.0, color);
    }
    draw_circle(center.x, center.y, radius, color);
    draw_circle(
        center.x - 4.0,
        center.y - 4.0,
        radius * 0.35,
        Color::from_rgba(255, 232, 143, 230),
    );
}

fn draw_centered_text(
    text: &str,
    x: f32,
    baseline_y: f32,
    width: f32,
    font_size: f32,
    color: Color,
) {
    let measured = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        color,
    );
}

fn draw_centered_text_shadowed(
    text: &str,
    x: f32,
    baseline_y: f32,
    width: f32,
    font_size: f32,
    color: Color,
) {
    let measured = measure_text(text, None, font_size as u16, 1.0);
    draw_text_shadowed(
        text,
        x + (width - measured.width) * 0.5,
        baseline_y,
        font_size,
        color,
    );
}

fn draw_text_shadowed(text: &str, x: f32, baseline_y: f32, font_size: f32, color: Color) {
    draw_text(
        text,
        x + 1.5,
        baseline_y + 2.0,
        font_size,
        Color::from_rgba(0, 0, 0, 130),
    );
    draw_text(text, x, baseline_y, font_size, color);
}

fn draw_wrapped_text_limited(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    font_size: f32,
    line_height: f32,
    color: Color,
    max_lines: usize,
) {
    let mut lines = macroquad_toolkit::ui::wrap_text(text, max_width, font_size);
    if lines.len() > max_lines {
        lines.truncate(max_lines);
        if let Some(last) = lines.last_mut() {
            let trimmed = last.trim_end_matches('.').to_owned();
            *last = truncate_text_to_width(&format!("{trimmed}..."), max_width, font_size);
        }
    }
    for (index, line) in lines.iter().enumerate() {
        draw_text(line, x, y + index as f32 * line_height, font_size, color);
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

fn fill_slate() -> Color {
    Color::from_rgba(34, 34, 45, 228)
}

fn slot_glow(index: usize) -> Color {
    match index % 5 {
        0 => Color::from_rgba(223, 77, 70, 255),
        1 => Color::from_rgba(75, 158, 232, 255),
        2 => Color::from_rgba(112, 203, 115, 255),
        3 => Color::from_rgba(232, 184, 76, 255),
        _ => Color::from_rgba(180, 92, 224, 255),
    }
}

fn bright_ink() -> Color {
    Color::from_rgba(246, 238, 213, 255)
}

fn muted_ink() -> Color {
    Color::from_rgba(186, 174, 145, 255)
}

fn parchment() -> Color {
    Color::from_rgba(226, 204, 162, 255)
}

fn brass() -> Color {
    Color::from_rgba(189, 140, 69, 255)
}

fn brass_light() -> Color {
    Color::from_rgba(242, 205, 126, 255)
}

fn shadow() -> Color {
    Color::from_rgba(0, 0, 0, 108)
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
