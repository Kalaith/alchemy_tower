use macroquad::prelude::*;

pub(crate) const HOTBAR_SLOT_COUNT: usize = 8;

pub(crate) struct HudPotionSlot {
    pub(crate) key_label: &'static str,
    pub(crate) icon_id: Option<String>,
    pub(crate) amount: u32,
}

pub(crate) struct HudGoal {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) detail: String,
    pub(crate) action: String,
    pub(crate) icon_id: Option<String>,
    pub(crate) amount_text: String,
}

pub(crate) struct HudFeedbackView {
    pub(crate) position: Vec2,
    pub(crate) radius: f32,
    pub(crate) color: Color,
    pub(crate) sparkle_points: [Vec2; 8],
    pub(crate) burst_scale: f32,
}

pub(crate) struct HudView {
    pub(crate) vitality_value: String,
    pub(crate) coins_value: String,
    pub(crate) clock_text: String,
    pub(crate) season_weather_text: String,
    pub(crate) day_text: String,
    pub(crate) sleep_warning_text: Option<String>,
    pub(crate) goal_prefix: String,
    pub(crate) goal: HudGoal,
    pub(crate) status_text: String,
    pub(crate) area_label: String,
    pub(crate) potions: [HudPotionSlot; 3],
    pub(crate) inventory_count: u32,
    pub(crate) effect_count: usize,
    pub(crate) feedbacks: Vec<HudFeedbackView>,
}